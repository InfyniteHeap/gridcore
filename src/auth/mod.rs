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

pub use self::microsoft_oauth2::*;
