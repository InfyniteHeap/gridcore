use reqwest::{header::HeaderMap, Client};
use serde_json::Value;

/// Send POST request and receive response.
pub(crate) async fn send_post_request(
    headers: Option<HeaderMap>,
    load: Value,
    url: &str,
) -> Result<String, reqwest::Error> {
    // Match cases that whether "headers" exist.
    match headers {
        // Case: both "headers" and "load" exist.
        Some(headers) => {
            Client::new()
                .post(url)
                .headers(headers)
                .json(&load)
                .send()
                .await?
                .text()
                .await
        }
        // Case: only "load" exists.
        None => {
            Client::new()
                .post(url)
                .json(&load)
                .send()
                .await?
                .text()
                .await
        }
    }
}

/// Send GET request and receive response.
pub(crate) async fn send_get_request(token: &str, url: &str) -> Result<String, reqwest::Error> {
    Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await
}
