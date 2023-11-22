use std::future::Future;

use serde_json::Value;
use tokio::runtime::Runtime;

use gridcore::auth::*;

#[test]
fn login_test() {
    // Create a Tokio runtime.
    let tokio_rt = Runtime::new().unwrap();
    // Assume there is a string that contains a Microsoft authorization code.
    let auth_code = "".to_string();

    let microsoft_oauth2_response = tokio_rt.block_on(fetch_response_from_remote(
        request_microsoft_oauth2_response,
        &auth_code,
    ));
    let microsoft_json_response = parse_response(&microsoft_oauth2_response);
    let microsoft_authorization_token = extract_value(
        &microsoft_json_response,
        "access_token",
        "Value not found: authorization token!",
    );

    println!("{:#?}\n", &microsoft_authorization_token);

    let xbox_response = tokio_rt.block_on(fetch_response_from_remote(
        request_xbox_authentication_response,
        &microsoft_authorization_token,
    ));
    let xbox_json_response = parse_response(&xbox_response);
    let xbox_authentication_token =
        extract_value(&xbox_json_response, "Token", "Value not found: xbox token!");

    println!("{:#?}\n", &xbox_authentication_token);

    let xsts_response = tokio_rt.block_on(fetch_response_from_remote(
        request_xsts_authorization_response,
        &xbox_authentication_token,
    ));
    let xsts_json_response = parse_response(&xsts_response);
    let uhs = extract_uhs(&xsts_json_response, "Value not found: uhs!");
    let xsts_authorization_token =
        extract_value(&xsts_json_response, "Token", "Value not found: xsts token!");

    println!("{:#?}\n", &uhs);
    println!("{:#?}\n", &xsts_authorization_token);

    let mut minecraft_profile: MinecraftProfile = Default::default();

    let minecraft_response = tokio_rt
        .block_on(
            minecraft_profile
                .request_minecraft_access_token_response(&xsts_authorization_token, &uhs),
        )
        .unwrap();
    let minecraft_json_response = parse_response(&minecraft_response);
    minecraft_profile.access_token = extract_value(
        &minecraft_json_response,
        "access_token",
        "Value not found: minecraft access token!",
    );

    println!("{:#?}\n", &minecraft_profile.access_token);

    let minecraft_response2 = tokio_rt
        .block_on(minecraft_profile.request_minecraft_uuid_and_username_response())
        .unwrap();
    let minecraft_json_response2 = parse_response(&minecraft_response2);
    minecraft_profile.uuid = extract_value(&minecraft_json_response2, "id", "Value not found: id!");
    minecraft_profile.username =
        extract_value(&minecraft_json_response2, "name", "Value not found: name!");

    let minecraft_response = serde_json::to_string_pretty(&minecraft_profile).unwrap();

    println!("{}\n", &minecraft_response);
}

async fn fetch_response_from_remote<'a, F, Fut>(func: F, para_for_func: &'a str) -> String
where
    F: Fn(&'a str) -> Fut,
    Fut: Future<Output = Result<String, reqwest::Error>>,
{
    match func(para_for_func).await {
        Ok(data) => data,
        Err(e) => panic!("{e}"),
    }
}

fn parse_response(response: &str) -> Value {
    match serde_json::from_str::<Value>(response) {
        Ok(data) => data,
        Err(e) => panic!("{e}"),
    }
}

fn extract_value(json_text: &Value, key: &str, err_msg: &str) -> String {
    match json_text[key].to_owned() {
        Value::String(val) => val,
        _ => panic!("{err_msg}"),
    }
}

fn extract_uhs(json_text: &Value, err_msg: &str) -> String {
    match json_text["DisplayClaims"]["xui"][0]["uhs"].to_owned() {
        Value::String(val) => val,
        _ => panic!("{err_msg}"),
    }
}
