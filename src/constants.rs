//! # Constants
//!
//! Collects constants and enumerations used by whole project.

use std::fmt::Display;


#[cfg(target_os = "macos")]
pub const MINECRAFT_ROOT: &str = "./minecraft";

#[cfg(not(target_os = "macos"))]
pub const MINECRAFT_ROOT: &str = "./.minecraft";

pub const CONFIG_DIRECTORY: &str = "./config";
pub const PROFILE_FILE_NAME: &str = "profile.json";
pub const CONFIG_FILE_NAME: &str = "config.toml";

pub const OFFICIAL: &str = "https://piston-meta.mojang.com";
pub const BANGBANG93: &str = "https://bmclapi2.bangbang93.com";
pub const ASSETS_OFFICIAL: &str = "https://resources.download.minecraft.net";
pub const ASSETS_BANGBANG93: &str = "https://bmclapi2.bangbang93.com/assets";

#[derive(Copy, Clone, PartialEq)]
pub enum DownloadSource {
    Official,
    Bangbang93,
}

#[derive(Copy, Clone)]
pub enum Category {
    Client,
    Server,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Client => String::from("client"),
                Self::Server => String::from("server"),
            }
        )
    }
}
