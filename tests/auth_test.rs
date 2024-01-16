use gridcore::{auth::*, json::serialize_to_json};

#[test]
fn login_test() -> anyhow::Result<()> {
    // Assume there is a string that contains a Microsoft authorization code.
    let auth_code = "";

    let profile = request_player_profile(auth_code)?;

    let profile = serialize_to_json(profile)?;
    println!("{}", profile);

    Ok(())
}
