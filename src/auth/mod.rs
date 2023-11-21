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

mod microsoft_oauth2;

use std::future::Future;

pub use self::microsoft_oauth2::*;
use crate::json::*;

pub async fn send_and_parse_data<'a, F, Fut>(
    func: F,
    para: &'a str,
    key: &str,
    err_msg: &str,
) -> String
where
    F: Fn(&'a str) -> Fut,
    Fut: Future<Output = Result<String, reqwest::Error>>,
{
    match func(para).await {
        Ok(data) => match parse_response(&data) {
            Ok(data) => match fetch_value(data.clone(), key) {
                // This string should be used in the next step and be taken out of this nest.
                Some(val) => val,
                // Eject a dialog to prompt user "Failed to fetch data from response!"
                None => panic!("{err_msg}"),
            },
            // Eject a dialog to prompt user "Failed to parse response: e".
            Err(e) => panic!("{e}"),
        },
        // Eject a dialog to prompt user "Failed fetch response from remote: e".
        Err(e) => panic!("{e}"),
    }
}
