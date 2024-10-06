//! # Microsoft OAuth2
//!
//! The module that is used for Minecraft genius verification.
//!
//! Since Mojang has deprecated Mojang account verification method,
//! this module exclusively supports Microsoft OAuth2.

use crate::file_system;
use crate::http;
use crate::json;
use crate::path::CONFIGURATIONS_DIRECTORY;

use std::collections::HashMap;
use std::path::Path;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::Serialize;
use serde_json::{json, Value};

// These constants are URLs that will receive POST (some are GET) request and return response.
const REQUEST_MICROSOFT_OAUTH2_TOKEN: &str = "https://login.live.com/oauth20_token.srf";
const XBOX_AUTHENTICATE: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTHORIZE: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const REQUEST_ACCESS_TOKEN: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const CHECK_IF_PLAYER_OWNS_GAME: &str = "https://api.minecraftservices.com/entitlements/mcstore";
const REQUEST_UUID_AND_USERNAME: &str = "https://api.minecraftservices.com/minecraft/profile";

#[derive(Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftProfile {
    /// The Minecraft access token.
    access_token: String,
    /// The UUID which is frequently used to verify a player's identity.
    uuid: String,
    /// The username which will display in the game.
    username: String,
}

/// Microsoft authorization code -> Microsoft authorization token
pub async fn request_microsoft_authorization_token(
    authorization_code: &'static str,
) -> Result<String, reqwest::Error> {
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
    let response =
        http::send_post_request(REQUEST_MICROSOFT_OAUTH2_TOKEN, Some(headers), &load).await?;

    match &json::parse_from_string(&response).await.unwrap()["access_token"] {
        Value::String(token) => Ok(token.to_owned()),
        _ => unreachable!(),
    }
}

/// Microsoft authorization token -> Xbox token
pub async fn request_xbox_authentication_response(
    access_token: &str,
) -> Result<String, reqwest::Error> {
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

    let response = http::send_post_request(XBOX_AUTHENTICATE, Some(headers), &load).await?;

    match &json::parse_from_string(&response).await.unwrap()["Token"] {
        Value::String(token) => Ok(token.to_owned()),
        _ => unreachable!(),
    }
}

/// Xbox token -> XSTS token, UHS
pub async fn request_xsts_authorization_response(
    xbox_token: &str,
) -> Result<(String, String), reqwest::Error> {
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

    let response = http::send_post_request(XSTS_AUTHORIZE, Some(headers), &load).await?;

    match (
        &json::parse_from_string(&response).await.unwrap()["Token"],
        &json::parse_from_string(&response).await.unwrap()["DisplayClaims"]["xui"][0]["uhs"],
    ) {
        (Value::String(token), Value::String(uhs)) => Ok((token.to_owned(), uhs.to_owned())),
        _ => unreachable!(),
    }
}

impl MinecraftProfile {
    /// UHS, XSTS token -> Minecraft access token
    pub async fn request_access_token_response(
        &mut self,
        xsts_token: &str,
        uhs: &str,
    ) -> Result<(), reqwest::Error> {
        let load = json!(
            {
                "identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token)
            }
        );

        let response = http::send_post_request(REQUEST_ACCESS_TOKEN, None, &load).await?;

        match &json::parse_from_string(&response).await.unwrap()["access_token"] {
            Value::String(access_token) => self.access_token.clone_from(access_token),
            _ => unreachable!(),
        }

        Ok(())
    }

    pub async fn check_if_player_owns_game(&self) -> Result<bool, reqwest::Error> {
        let response =
            http::send_get_request(CHECK_IF_PLAYER_OWNS_GAME, &self.access_token).await?;

        if let Value::Array(items) = &json::parse_from_string(&response).await.unwrap()["items"] {
            if !items.is_empty() {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Minecraft access token -> Minecraft username, Minecraft UUID
    pub async fn request_uuid_and_username_response(&mut self) -> Result<(), reqwest::Error> {
        let response =
            http::send_get_request(REQUEST_UUID_AND_USERNAME, &self.access_token).await?;

        match (
            &json::parse_from_string(&response).await.unwrap()["name"],
            &json::parse_from_string(&response).await.unwrap()["id"],
        ) {
            (Value::String(username), Value::String(uuid)) => {
                self.username.clone_from(username);
                self.uuid.clone_from(uuid);
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    pub async fn save_to_file(&self) -> anyhow::Result<()> {
        let contents = json::convert_to_string(self).await?;

        Ok(file_system::write_into_file(
            Path::new(CONFIGURATIONS_DIRECTORY),
            "profile.json",
            contents.as_bytes(),
        )
        .await?)
    }
}
