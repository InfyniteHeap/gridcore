//! # Paths
//!
//! This module contains the paths to the directories and files used by the program.

#[cfg(target_os = "windows")]
pub const MINECRAFT_ROOT: &str = "./.minecraft";

#[cfg(target_os = "macos")]
pub const MINECRAFT_ROOT: &str = "./minecraft";

#[cfg(target_os = "linux")]
pub const MINECRAFT_ROOT: &str = "./.minecraft";

pub const CONFIG_DIRECTORY: &str = "./config";
pub const PROFILE_FILE_NAME: &str = "profile.json";
pub const CONFIG_FILE_NAME: &str = "config.toml";
