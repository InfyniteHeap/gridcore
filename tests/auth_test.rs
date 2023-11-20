use std::future::Future;

use gridcore::auth::microsoft_oauth2::*;
use gridcore::json::*;

use tokio::runtime::Runtime;

static mut MICROSOFT_AUTHORIZATION_TOKEN: String = String::new();
static mut XBOX_AUTHENTICATION_TOKEN: String = String::new();
// static mut XSTS_AUTHORIZATION_TOKEN: String = String::new();
// static mut UHS: String = String::new();

#[test]
fn login() {
    // Create a Tokio runtime.
    let tokio_rt = Runtime::new().unwrap();
    // Suppose there is a string that holds a Microsoft authorization code.
    let auth_code = "".to_string();

    tokio_rt.block_on(send_and_parse_data(
        request_microsoft_oauth2_token,
        &auth_code,
        "access_token",
        unsafe { &mut MICROSOFT_AUTHORIZATION_TOKEN },
        "err_msg",
    ));

    println!("{:#?}\n", unsafe { &MICROSOFT_AUTHORIZATION_TOKEN });

    tokio_rt.block_on(send_and_parse_data(
        request_xbox_authentication,
        unsafe { &MICROSOFT_AUTHORIZATION_TOKEN },
        "Token",
        unsafe { &mut XBOX_AUTHENTICATION_TOKEN },
        "err_msg",
    ));

    println!("{:#?}\n", unsafe { &XBOX_AUTHENTICATION_TOKEN });
}

async fn send_and_parse_data<'a, F: Fn(&'a str) -> Fut, Fut>(
    func: F,
    para: &'a str,
    key: &str,
    val: &mut String,
    err_msg: &str,
) where
    Fut: Future<Output = Result<String, reqwest::Error>>,
{
    match func(para).await {
        Ok(data) => match parse_response(&data) {
            Ok(data) => match fetch_value(data.clone(), key) {
                // This string should be used in the next step and be taken out of this nest.
                Some(v) => *val = v,
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
