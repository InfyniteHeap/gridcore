use crate::json;
use crate::path::MINECRAFT_ROOT;

use std::env::consts::{ARCH, OS};
use std::path::Path;
use std::process::Command;

use serde::Serialize;
use serde_json::Value;

#[derive(Default, Serialize)]
pub struct LaunchArguments {
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}

pub fn get_launch_args(version: &str) -> anyhow::Result<Vec<String>> {
    let manifest_path = format!("{}/versions/{}", MINECRAFT_ROOT, version);
    let manifest_path = Path::new(&manifest_path);
    let manifest_name = format!("{}.json", version);

    let data = json::read(manifest_path, &manifest_name)?;
    let mut launch_args = LaunchArguments::default();

    if let Value::Object(args) = &data["arguments"] {
        for (arg_ty, arg_v) in args {
            match arg_ty.as_str() {
                "game" => {
                    if let Value::Array(game_args) = &arg_v[arg_ty] {
                        for arg in game_args {
                            match arg {
                                Value::String(str) => launch_args.game.push(str.to_owned()),
                                Value::Object(obj) => {
                                    if let Value::Object(features) = &obj["rules"][0]["features"] {
                                        for (_k, v) in features {
                                            match v {
                                                Value::Bool(_) => {
                                                    // TODO
                                                    todo!()
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
                "jvm" => {
                    if let Value::Array(jvm_args) = &arg_v[arg_ty] {
                        for arg in jvm_args {
                            match arg {
                                Value::String(str) => launch_args.jvm.push(str.to_owned()),
                                Value::Object(obj) => {
                                    if [Value::Null, Value::String(OS.replace("macos", "osx"))]
                                        .contains(&obj["rules"][0]["os"]["name"])
                                        || [Value::Null, Value::String(ARCH.into())]
                                            .contains(&obj["rules"][0]["os"]["arch"])
                                    {
                                        if let Value::String(arg) = &obj["value"][0] {
                                            launch_args.jvm.push(arg.to_owned());
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    } else if let Value::String(args) = &data["minecraftArguments"] {
        launch_args.game = args
            .split(' ')
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
    }

    for arg in &mut launch_args.game {
        if arg.starts_with('$') {
            *arg = String::new();
        }
    }

    Ok(launch_args
        .jvm
        .into_iter()
        .chain(launch_args.game)
        .collect())
}

pub fn launch_game(args: Vec<String>) -> anyhow::Result<()> {
    Command::new("java")
        .args(args)
        .status()
        .expect("Failed to launch Minecraft!");

    Ok(())
}
