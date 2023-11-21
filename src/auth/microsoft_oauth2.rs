// CURRENT STATUS: XSTS REQUEST CAN RETURN JSON RESPONSE BUT NOT THE EXPECTED ONE.

use std::collections::HashMap;

use reqwest::{header::*, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::https::*;

// These constants are URLs that will receive POST (some are GET) request and return response.
const REQUEST_MICROSOFT_OAUTH2_TOKEN: &str = "https://login.live.com/oauth20_token.srf";
const XBOX_AUTHENTICATE: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTHORIZE: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const REQUEST_MINECRAFT_ACCESS_TOKEN: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const CHECK_IF_PLAYER_OWN_MINECRAFT: &str =
    "https://api.minecraftservices.com/entitlements/mcstore";
const REQUEST_MINECRAFT_UUID_AND_USERNAME: &str =
    "https://api.minecraftservices.com/minecraft/profile";

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftProfile {
    // The Minecraft access token.
    pub access_token: String,
    // The UUID which is frequently used to verify a player's identity.
    pub uuid: String,
    // The username which will display in the game.
    pub username: String,
}

pub async fn request_microsoft_oauth2_token(
    authorization_code: &str,
) -> Result<String, reqwest::Error> {
    // The request header.
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    // The parameters.
    let mut paras = HashMap::new();
    paras.insert("client_id", "00000000402b5328");
    paras.insert("code", authorization_code);
    paras.insert("grant_type", "authorization_code");
    paras.insert("redirect_uri", "https://login.live.com/oauth20_desktop.srf");
    paras.insert("scope", "service::user.auth.xboxlive.com::MBI_SSL");

    // Send POST request and receive response.
    // Because the first request should send a HashMap,
    // function "send_post_request" is not used here.
    Client::new()
        .post(REQUEST_MICROSOFT_OAUTH2_TOKEN)
        .headers(headers)
        .form(&paras)
        .send()
        .await?
        .text()
        .await
}

pub async fn request_xbox_authentication(access_token: &str) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let paras = json!(
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

    send_post_request(Some(headers), Some(paras), XBOX_AUTHENTICATE).await
}

pub async fn request_xsts_authorization(xbox_token: &str) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let paras = json!(
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

    send_post_request(Some(headers), Some(paras), XSTS_AUTHORIZE).await
}

impl MinecraftProfile {
    pub fn new() -> Self {
        Self {
            access_token: String::new(),
            uuid: String::new(),
            username: String::new(),
        }
    }

    pub async fn request_minecraft_access_token(
        &mut self,
        xsts_token: &str,
        uhs: &str,
    ) -> Result<String, reqwest::Error> {
        let paras = json!(
            {
                "identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token)
            }
        );

        send_post_request(None, Some(paras), REQUEST_MINECRAFT_ACCESS_TOKEN).await
    }

    pub async fn check_if_player_own_minecraft(&self) -> Result<String, reqwest::Error> {
        send_get_request(&self.access_token, CHECK_IF_PLAYER_OWN_MINECRAFT).await
    }

    pub async fn request_minecraft_uuid_and_username(&self) -> Result<String, reqwest::Error> {
        send_get_request(&self.access_token, REQUEST_MINECRAFT_UUID_AND_USERNAME).await
    }
}
