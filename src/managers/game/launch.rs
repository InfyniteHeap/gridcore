use crate::constants::{CONFIG_DIRECTORY, MINECRAFT_ROOT, PROFILE_FILE_NAME};
use crate::error_handling::LaunchError;
use crate::utils::json_processer;

use std::collections::HashMap;
use std::env::consts::{ARCH, OS};
use std::process::Command;

use regex::Regex;
use serde::Serialize;
use serde_json::Value;

const CONFIG_NUMS: usize = 18;

#[derive(Default, Serialize)]
pub struct LaunchArguments {
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}

/// Sets the resolution of the game window.
///
/// In general, you're no need to set this manually.
pub struct Resolution(pub u16, pub u16);

pub async fn generate_launch_args(
    version: &str,
    jvm_x_args: &str,
) -> Result<Vec<String>, LaunchError> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_name = format!("{}.json", version);

    let data = json_processer::read(&manifest_path, &manifest_name).await?;
    let mut launch_args = LaunchArguments::default();

    // Get original launch arguments from JSON.
    if let Value::Object(args) = &data["arguments"] {
        for (arg_ty, arg_val) in args {
            match arg_ty.as_str() {
                "jvm" => {
                    if let Value::Array(jvm_args) = &arg_val[arg_ty] {
                        jvm_args.iter().for_each(|arg| {
                            if arg.is_string() {
                                // We already identified that `arg` is a string.
                                launch_args.jvm.push(arg.as_str().unwrap().to_string())
                            } else if arg.is_object() {
                                let name = &arg.as_object().unwrap()["rules"][0]["os"]["name"];
                                let arch = &arg.as_object().unwrap()["rules"][0]["os"]["arch"];

                                if [Value::Null, Value::String(OS.replace("macos", "osx"))]
                                    .contains(name)
                                    || [Value::Null, Value::String(ARCH.into())].contains(arch)
                                {
                                    let value = &arg.as_object().unwrap()["value"];

                                    if value.is_string() {
                                        launch_args.jvm.push(value.as_str().unwrap().to_string());
                                    } else if value.is_array() {
                                        launch_args.jvm.extend(
                                            value
                                                .as_array()
                                                .unwrap()
                                                .iter()
                                                .map(|arg| arg.as_str().unwrap().to_string()),
                                        );
                                    }
                                }
                            }
                        });
                    }
                }
                "game" => {
                    if let Value::Array(game_args) = &arg_val[arg_ty] {
                        game_args.iter().for_each(|arg| {
                            if arg.is_string() {
                                // We already identified that `arg` is a string.
                                launch_args.game.push(arg.as_str().unwrap().to_string())
                            }
                            // We deliberately omitted the cases when `arg` is an object,
                            // because they can be configured later.
                        });
                    }
                }
                // According to JSON file, other cases will never reach.
                _ => unreachable!(),
            }
        }
    } else if let Value::String(args) = &data["minecraftArguments"] {
        launch_args.game = args
            .split(' ')
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
    }

    let logging_arg = data["logging"]["client"]["argument"]
        .as_str()
        // We already identified that `arg` is a string.
        .unwrap()
        .to_owned();

    // Replace placeholders with actual arguments.
    // We first handle jvm arguments.

    // Then handle game arguments.
    let profile = json_processer::read(&CONFIG_DIRECTORY, PROFILE_FILE_NAME).await?;

    let mut game_args = HashMap::with_capacity(CONFIG_NUMS);
    game_args.insert(
        "username",
        String::from(profile["username"].as_str().unwrap()),
    );
    game_args.insert("version_name", version.to_string());
    game_args.insert("game_directory", MINECRAFT_ROOT.to_string());
    game_args.insert("assets_root", format!("{}/assets", MINECRAFT_ROOT));
    game_args.insert(
        "assets_index_name",
        data["assets"].as_str().unwrap().to_string(),
    );
    game_args.insert("auth_uuid", String::from(profile["uuid"].as_str().unwrap()));
    game_args.insert(
        "auth_access_token",
        String::from(profile["accessToken"].as_str().unwrap()),
    );
    game_args.insert(
        "clientid",
        String::from("MTFjMTBkZjctMmJlMC00ZTZmLTgxMDItMWYzODMxZDU4NDk0"),
    );
    game_args.insert("auth_xuid", String::from("2535472045104657"));
    game_args.insert("user_type", String::from("msa"));
    game_args.insert("version_type", data["type"].as_str().unwrap().to_string());

    let re = Regex::new(r"^\$\{(.*?)}$")?;

    for arg in launch_args.game.iter_mut() {
        match game_args.get(&re.captures(arg).unwrap()[1]) {
            Some(a) => {
                a.clone_into(arg);
            }
            None => continue,
        }
    }

    // We already identified that `arg` is a string.
    let log_arg = data["mainClass"].as_str().unwrap().to_owned();

    // We finally merge these parts of arguments into one vector.
    Ok(launch_args
        .jvm
        .into_iter()
        .chain(jvm_x_args.split(' ').map(|arg| arg.to_owned()))
        .chain([logging_arg])
        .chain(launch_args.game)
        .collect())
}

pub fn launch_game(args: Vec<String>) {
    let output = Command::new("java").args(args).output().unwrap();

    if !output.status.success() {
        println!(
            "Failed to launch Minecraft: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
}
