use std::any::Any;
use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::Value;

/// Send POST request and receive response.
pub(crate) async fn send_post_request(
    url: &str,
    headers: Option<HeaderMap>,
    load: &dyn Any,
) -> Result<String, reqwest::Error> {
    match headers {
        Some(headers) if load.is::<Value>() => {
            Client::new()
                .post(url)
                .headers(headers)
                // We already identified that `load` is `Value`.
                .json(load.downcast_ref::<Value>().unwrap())
                .send()
                .await?
                .text()
                .await
        }
        Some(headers) if load.is::<HashMap<&str, &str>>() => {
            Client::new()
                .post(url)
                .headers(headers)
                // We already identified that `load` is `HashMap<&str, &str>`.
                .form(load.downcast_ref::<HashMap<&str, &str>>().unwrap())
                .send()
                .await?
                .text()
                .await
        }
        None if load.is::<Value>() => {
            Client::new()
                .post(url)
                // We already identified that `load` is `Value`.
                .json(load.downcast_ref::<Value>().unwrap())
                .send()
                .await?
                .text()
                .await
        }
        None if load.is::<HashMap<&str, &str>>() => {
            Client::new()
                .post(url)
                // We already identified that `load` is `HashMap<&str, &str>`.
                .form(load.downcast_ref::<HashMap<&str, &str>>().unwrap())
                .send()
                .await?
                .text()
                .await
        }
        // The `load` is either `Value` or `HashMap<&str, &str>`,
        // so other cases will never reach out.
        _ => unreachable!(),
    }
}

/// Send GET request and receive response.
pub(crate) async fn send_get_request(url: &str, token: &str) -> Result<String, reqwest::Error> {
    Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await
}
