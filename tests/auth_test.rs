use std::future::Future;

use serde_json::Value;
use tokio::runtime::Runtime;

use gridcore::{auth::*, json::*};

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
    let microsoft_authorization_token = extract_value(&microsoft_json_response, "access_token");

    println!("{:#?}\n", &microsoft_authorization_token);

    let xbox_response = tokio_rt.block_on(fetch_response_from_remote(
        request_xbox_authentication_response,
        &microsoft_authorization_token,
    ));
    let xbox_json_response = parse_response(&xbox_response);
    let xbox_authentication_token = extract_value(&xbox_json_response, "Token");

    println!("{:#?}\n", &xbox_authentication_token);

    let xsts_response = tokio_rt.block_on(fetch_response_from_remote(
        request_xsts_authorization_response,
        &xbox_authentication_token,
    ));
    let xsts_json_response = parse_response(&xsts_response);
    let uhs = extract_uhs(&xsts_json_response);
    let xsts_authorization_token = extract_value(&xsts_json_response, "Token");

    println!("{:#?}\n", &uhs);
    println!("{:#?}\n", &xsts_authorization_token);

    let mut minecraft_profile: MinecraftProfile = Default::default();

    match tokio_rt.block_on(
        minecraft_profile.request_minecraft_access_token_response(&xsts_authorization_token, &uhs),
    ) {
        Ok(response) => {
            let minecraft_json_response = parse_response(&response);
            minecraft_profile.access_token =
                extract_value(&minecraft_json_response, "access_token");
        }
        Err(e) => panic!("{e}"),
    }

    println!("{:#?}\n", &minecraft_profile.access_token);

    match tokio_rt.block_on(minecraft_profile.request_minecraft_uuid_and_username_response()) {
        Ok(response) => {
            let minecraft_json_response2 = parse_response(&response);
            minecraft_profile.uuid = extract_value(&minecraft_json_response2, "id");
            minecraft_profile.username = extract_value(&minecraft_json_response2, "name");
        }
        Err(e) => panic!("{e}"),
    }

    let minecraft_response = serialize_to_json(minecraft_profile).unwrap();

    println!("{}\n", &minecraft_response);
}

// Functions below will explicitly handle all of errors.
async fn fetch_response_from_remote<'a, F, Fut>(func: F, para_for_func: &'a str) -> String
where
    F: Fn(&'a str) -> Fut,
    Fut: Future<Output = Result<String, reqwest::Error>>,
{
    match func(para_for_func).await {
        Ok(data) => data,
        // This should eject a window that prompt user "Failed to fetch response from remote!" and other details.
        Err(e) => panic!("{e}"),
    }
}

fn parse_response(response: &str) -> Value {
    match serde_json::from_str::<Value>(response) {
        Ok(data) => data,
        // This should eject a window that prompt user "Failed to parse response!" and other details.
        Err(e) => panic!("{e}"),
    }
}

fn extract_value(json_text: &Value, key: &str) -> String {
    match json_text[key].to_owned() {
        Value::String(val) => val,
        // This should eject a window that prompt user "Failed to extract value from returned json: {}!".
        _ => panic!("Failed to extract value from returned json: {}!", key),
    }
}

fn extract_uhs(json_text: &Value) -> String {
    match json_text["DisplayClaims"]["xui"][0]["uhs"].to_owned() {
        Value::String(val) => val,
        // This should eject a window that prompt user "Failed to extract value from returned json: uhs!".
        _ => panic!("Failed to extract value from returned json: uhs!"),
    }
}
