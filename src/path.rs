#[cfg(target_os = "windows")]
pub const MINECRAFT_ROOT: &str = "./.minecraft";

#[cfg(target_os = "macos")]
pub const MINECRAFT_ROOT: &str = "./minecraft";

#[cfg(target_os = "linux")]
pub const MINECRAFT_ROOT: &str = "./.minecraft";

pub const CONFIGURATIONS_DIRECTORY: &str = "./config";
