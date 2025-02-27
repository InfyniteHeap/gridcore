use std::path::Path;

use gridcore::checksum;

#[ignore = "This test case must be manually tested on local machine."]
#[tokio::test]
// Note: Please run this test after downloading Minecraft files!
async fn cal_sha1() {
    let file_path = Path::new("./.minecraft/versions/1.21.1");
    let file_name = "1.21.4.jar";

    println!(
        "{}",
        checksum::calculate_sha1(file_path, file_name)
            .await
            .unwrap()
    )
}
