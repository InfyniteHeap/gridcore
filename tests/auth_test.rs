use std::future::Future;

use gridcore::{auth::*, json::*};

use tokio::runtime::Runtime;

#[test]
fn login_test() {
    // Create a Tokio runtime.
    let tokio_rt = Runtime::new().unwrap();
    // Assume there is a string that contains a Microsoft authorization code.
    let auth_code = "".to_string();

    let microsoft_authorization_token = tokio_rt.block_on(send_and_parse_data(
        request_microsoft_oauth2_token,
        &auth_code,
        "access_token",
        "Value not found!",
    ));

    println!("{:#?}\n", &microsoft_authorization_token);

    let xbox_authentication_token = tokio_rt.block_on(send_and_parse_data(
        request_xbox_authentication,
        &microsoft_authorization_token,
        "Token",
        "Value not found!",
    ));

    println!("{:#?}\n", &xbox_authentication_token);

    // This is only a temporary method, aiming to check parsing errors.
    match tokio_rt.block_on(request_xsts_authorization(&xbox_authentication_token)) {
        Ok(data) => match parse_response(&data) {
            Ok(data) => println!("{:#?}", data),
            Err(e) => panic!("{e}"),
        },
        Err(e) => panic!("{e}"),
    }
}

// This function will may be seperated as it is too complicated!
async fn send_and_parse_data<'a, F, Fut>(func: F, para: &'a str, key: &str, err_msg: &str) -> String
where
    F: Fn(&'a str) -> Fut,
    Fut: Future<Output = Result<String, reqwest::Error>>,
{
    match func(para).await {
        Ok(data) => match parse_response(&data) {
            Ok(data) => match fetch_value(data.clone(), key) {
                // This string should be used for the next step.
                Some(val) => val,
                // Eject a dialog to prompt user "Failed to fetch data from response!"
                None => panic!("{err_msg}"),
            },
            // Eject a dialog to prompt user "Failed to parse response: e".
            Err(e) => panic!("{e}"),
        },
        // Eject a dialog to prompt user "Failed fetch response from remote: e".
        Err(e) => panic!("{e}"),
    }
}
