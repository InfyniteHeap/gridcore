use std::path::Path;

use gridcore::utils::sha1_checker;

#[ignore = "This test case must be manually tested on local machine."]
#[tokio::test]
// Note: Please run this test after downloading Minecraft files!
async fn cal_sha1() {
    let file_path = "./.minecraft/versions/1.21.5";
    let file_name = "1.21.5.jar";

    println!(
        "{}",
        sha1_checker::calculate_sha1(&file_path, file_name)
            .await
            .unwrap()
    )
}
