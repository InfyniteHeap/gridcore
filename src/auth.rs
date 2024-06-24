//! # Microsoft OAuth2
//!
//! The module that is used for Minecraft genius verification.
//!
//! Since Mojang has deprecated Mojang account verification method,
//! this module exclusively supports Microsoft OAuth2.
//!
//! ## Example
//!
//! To fetch your info from server, just use this function:
//!
//! ```rust
//! // Assume there is a string that contains a Microsoft authorization code.
//! let auth_code = "";
//! let profile: MinecraftProfile = request_player_profile(auth_code).unwrap();
//! ```

use crate::{
    https::*,
    json::{extract_value, parse_response},
};

use std::collections::HashMap;

use reqwest::{header::*, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Runtime;

// These constants are URLs that will receive POST (some are GET) request and return response.
const REQUEST_MICROSOFT_OAUTH2_TOKEN: &str = "https://login.live.com/oauth20_token.srf";
const XBOX_AUTHENTICATE: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTHORIZE: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const REQUEST_MINECRAFT_ACCESS_TOKEN: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
// const CHECK_IF_PLAYER_OWN_MINECRAFT: &str =
//     "https://api.minecraftservices.com/entitlements/mcstore";
const REQUEST_MINECRAFT_UUID_AND_USERNAME: &str =
    "https://api.minecraftservices.com/minecraft/profile";

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftProfile {
    /// The Minecraft access token.
    access_token: String,
    /// The UUID which is frequently used to verify a player's identity.
    uuid: String,
    /// The username which will display in the game.
    username: String,
}

pub fn request_player_profile(authorization_code: &str) -> anyhow::Result<MinecraftProfile> {
    let tokio_rt = Runtime::new()?;
    Ok(tokio_rt.block_on(request_microsoft_authorization_token(authorization_code))?)
}

/// Microsoft authorization code -> Microsoft authorization token
async fn request_microsoft_authorization_token(
    authorization_code: &str,
) -> Result<MinecraftProfile, reqwest::Error> {
    // The request header.
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    // The load that will be summitted.
    let mut load = HashMap::new();
    load.insert("client_id", "00000000402b5328");
    load.insert("code", authorization_code);
    load.insert("grant_type", "authorization_code");
    load.insert("redirect_uri", "https://login.live.com/oauth20_desktop.srf");
    load.insert("scope", "service::user.auth.xboxlive.com::MBI_SSL");

    // Send POST request and receive response.
    // Because the first request should send a HashMap,
    // function "send_post_request" is not used here.
    let response = Client::new()
        .post(REQUEST_MICROSOFT_OAUTH2_TOKEN)
        .headers(headers)
        .form(&load)
        .send()
        .await?
        .text()
        .await?;

    let token = extract_value(&parse_response(&response), &["access_token"]);

    request_xbox_authentication_response(&token).await
}

/// Microsoft authorization token -> Xbox token
async fn request_xbox_authentication_response(
    access_token: &str,
) -> Result<MinecraftProfile, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let load = json!(
        {
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": access_token,
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
        }
    );

    let response = send_post_request(Some(headers), load, XBOX_AUTHENTICATE).await?;

    let token = extract_value(&parse_response(&response), &["Token"]);

    request_xsts_authorization_response(&token).await
}

/// Xbox token -> UHS, XSTS token
async fn request_xsts_authorization_response(
    xbox_token: &str,
) -> Result<MinecraftProfile, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let load = json!(
        {
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [
                xbox_token
            ]
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT"
        }
    );

    let response = send_post_request(Some(headers), load, XSTS_AUTHORIZE).await?;

    let token = extract_value(&parse_response(&response), &["Token"]);
    let uhs = extract_value(
        &parse_response(&response),
        &["DisplayClaims", "xui", "0", "uhs"],
    );

    let minecraft_profile: MinecraftProfile = Default::default();

    minecraft_profile.return_profile(&token, &uhs).await
}

impl MinecraftProfile {
    pub(self) async fn return_profile(
        mut self,
        xsts_token: &str,
        uhs: &str,
    ) -> Result<Self, reqwest::Error> {
        self.request_minecraft_access_token_response(xsts_token, uhs)
            .await?;
        self.request_minecraft_uuid_and_username_response().await?;

        Ok(self)
    }
    /// UHS, XSTS token -> Minecraft access token
    async fn request_minecraft_access_token_response(
        &mut self,
        xsts_token: &str,
        uhs: &str,
    ) -> Result<(), reqwest::Error> {
        let load = json!(
            {
                "identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token)
            }
        );

        let response = send_post_request(None, load, REQUEST_MINECRAFT_ACCESS_TOKEN).await?;

        self.access_token = extract_value(&parse_response(&response), &["access_token"]);

        Ok(())
    }

    /// Minecraft access token -> Minecraft username, Minecraft UUID
    async fn request_minecraft_uuid_and_username_response(&mut self) -> Result<(), reqwest::Error> {
        let response =
            send_get_request(&self.access_token, REQUEST_MINECRAFT_UUID_AND_USERNAME).await?;

        self.username = extract_value(&parse_response(&response), &["name"]);
        self.uuid = extract_value(&parse_response(&response), &["id"]);

        Ok(())
    }
}
