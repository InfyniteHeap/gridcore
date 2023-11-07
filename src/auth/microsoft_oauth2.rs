// This module is a rather early implementation!!!
// That means you can not use this module!
// All of errors will be handled at frontend.

// CURRENT STATUS: XSTS REQUEST CAN RETURN JSON RESPONSE BUT THIS IS AN UNEXPECTED RESPONSE.

use std::collections::HashMap;

use reqwest::{header::*, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// These constants are URLs that will receive POST (some are GET) request and response JSON data.
const FETCH_MICROSOFT_OAUTH2_TOKEN: &str = "https://login.live.com/oauth20_token.srf";
const XBOX_AUTHENTICATE: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTHORIZE: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const FETCH_MINECRAFT_ACCESS_TOKEN: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const CHECK_IF_PLAYER_OWN_MINECRAFT: &str =
    "https://api.minecraftservices.com/entitlements/mcstore";
const FETCH_MINECRAFT_UUID_AND_USERNAME: &str =
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
    // Because the first request should send a HashMap, I didn't use "send_request" function.
    let client = Client::new();
    client
        .post(FETCH_MICROSOFT_OAUTH2_TOKEN)
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

        send_post_request(None, Some(paras), FETCH_MINECRAFT_ACCESS_TOKEN).await
    }

    pub async fn check_if_player_own_minecraft(&self) -> Result<String, reqwest::Error> {
        send_get_request(&self.access_token, CHECK_IF_PLAYER_OWN_MINECRAFT).await
    }

    pub async fn request_minecraft_uuid_and_username(&self) -> Result<String, reqwest::Error> {
        send_get_request(&self.access_token, FETCH_MINECRAFT_UUID_AND_USERNAME).await
    }
}

// Send POST requests and return results.
pub async fn send_post_request<T: Serialize>(
    headers: Option<HeaderMap>,
    paras: Option<T>,
    url: &str,
) -> Result<String, reqwest::Error> {
    // Create a client. This is the preparation of sending "POST" request.
    let client = Client::new();

    // Match cases that whether "headers" exist.
    match headers {
        // Case: both "headers" and "paras" exist.
        Some(headers) => {
            client
                .post(url)
                .headers(headers)
                .json(&paras)
                .send()
                .await?
                .text()
                .await
        }
        // Case: only have "paras".
        None => client.post(url).json(&paras).send().await?.text().await,
    }
}

// Send GET requests and return results.
pub async fn send_get_request(token: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    client
        .get(url)
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await
}

// Transfer responses into JSON, fetch necessary fields and store them in the instance of structure.
// Use turbofish syntax to simplify data processing.
pub fn parse_response(response: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str::<Value>(response)
}

pub fn fetch_data(result: Value, content: &str) -> Option<String> {
    let data = result.get(content)?.to_string();
    Some(data)
}
