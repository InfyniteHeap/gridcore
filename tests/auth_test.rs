use gridcore::auth::{self, MinecraftProfile};
use gridcore::json;
use gridcore::path::CONFIGURATIONS_DIRECTORY;

use std::path::Path;

use tokio::runtime::Runtime;

#[test]
fn login_test() {
    // Assume there is a string that contains a Microsoft authorization code.
    let auth_code = "";

    let tokio_rt = Runtime::new().unwrap();

    let access_token = tokio_rt
        .block_on(auth::request_microsoft_authorization_token(auth_code))
        .unwrap();
    let xbox_token = tokio_rt
        .block_on(auth::request_xbox_authentication_response(&access_token))
        .unwrap();
    let (xsts_token, uhs) = tokio_rt
        .block_on(auth::request_xsts_authorization_response(&xbox_token))
        .unwrap();

    let mut profile = MinecraftProfile::default();

    tokio_rt
        .block_on(profile.request_access_token_response(&xsts_token, &uhs))
        .unwrap();
    tokio_rt
        .block_on(profile.request_uuid_and_username_response())
        .unwrap();
    profile.save_to_file().unwrap();

    let profile = json::read(Path::new(CONFIGURATIONS_DIRECTORY), "profile.json").unwrap();
    println!("{}", profile);
}
