//! # Microsoft OAuth2
//!
//! The module that is used for Minecraft genius verification.
//!
//! Since Mojang has deprecated Mojang account verification method,
//! this module exclusively supports Microsoft OAuth2.
//!
//! ## Example
//!
//! This example will roughly demonstrate the steps to complete the verification process.
//!
//! Before starting, it's crucial to add `tokio` crate to `Cargo.toml`
//! with `rt-multi-thread` feture enabled:
//!
//! ```toml
//! [dependencies]
//! tokio = { version = "1.34.0", features = ["rt-multi-thread"] }
//! ```
//!
//! Then `use` these modules in the scope:
//!
//! ```rust
//! use std::future::Future;
//! use gridcore::{auth::*, json::*};
//! use tokio::runtime::Runtime;
//! ```
//!
//! Before starting the verification, let's create a Tokio Runtime:
//!
//! ```rust
//! let tokio_rt = Runtime::new().unwrap();
//! ```
//!
//! We require you initialize a value with a `String` type.
//!
//! For the purpose of this demonstration, let's assume there is a String type value
//! containing the Microsoft authorization code:
//!
//! ```rust
//! let auth_code = "".to_string();
//! ```

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
    /// The Minecraft access token.
    pub access_token: String,
    /// The UUID which is frequently used to verify a player's identity.
    pub uuid: String,
    /// The username which will display in the game.
    pub username: String,
}

/// Microsoft authorization code -> Microsoft authorization token
pub async fn request_microsoft_oauth2_response(
    authorization_code: &str,
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
    // Because the first request should send a HashMap,
    // function "send_post_request" is not used here.
    Client::new()
        .post(REQUEST_MICROSOFT_OAUTH2_TOKEN)
        .headers(headers)
        .form(&load)
        .send()
        .await?
        .text()
        .await
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

    send_post_request(Some(headers), Some(load), XBOX_AUTHENTICATE).await
}

/// Xbox token -> UHS, XSTS token
pub async fn request_xsts_authorization_response(
    xbox_token: &str,
) -> Result<String, reqwest::Error> {
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

    send_post_request(Some(headers), Some(load), XSTS_AUTHORIZE).await
}

impl MinecraftProfile {
    /// UHS, XSTS token -> Minecraft access token
    pub async fn request_minecraft_access_token_response(
        &self,
        xsts_token: &str,
        uhs: &str,
    ) -> Result<String, reqwest::Error> {
        let load = json!(
            {
                "identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token)
            }
        );

        send_post_request(None, Some(load), REQUEST_MINECRAFT_ACCESS_TOKEN).await
    }

    /// Minecraft access token
    pub async fn check_if_player_own_minecraft(&self) -> Result<String, reqwest::Error> {
        send_get_request(&self.access_token, CHECK_IF_PLAYER_OWN_MINECRAFT).await
    }

    /// Minecraft access token -> Minecraft username, Minecraft UUID
    pub async fn request_minecraft_uuid_and_username_response(
        &self,
    ) -> Result<String, reqwest::Error> {
        send_get_request(&self.access_token, REQUEST_MINECRAFT_UUID_AND_USERNAME).await
    }
}
