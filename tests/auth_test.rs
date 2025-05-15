use gridcore::auth::{self, MinecraftProfile};
use gridcore::constants::CONFIG_DIRECTORY;
use gridcore::utils::json_processer;

#[ignore = "This test case must be manually tested on local machine."]
#[tokio::test]
async fn login_test() {
    // Assume there is a string that contains a Microsoft authorization code.
    let auth_code = "";

    let access_token = auth::request_microsoft_authorization_token(auth_code)
        .await
        .unwrap();
    let xbox_token = auth::request_xbox_authentication_response(&access_token)
        .await
        .unwrap();
    let (xsts_token, uhs) = auth::request_xsts_authorization_response(&xbox_token)
        .await
        .unwrap();

    let mut profile = MinecraftProfile::default();

    profile
        .request_access_token_response(&xsts_token, &uhs)
        .await
        .unwrap();
    profile.request_uuid_and_username_response().await.unwrap();
    profile.save_to_file().await.unwrap();

    let profile = json_processer::read(&CONFIG_DIRECTORY, "profile.json")
        .await
        .unwrap();
    println!("{}", profile);
}
