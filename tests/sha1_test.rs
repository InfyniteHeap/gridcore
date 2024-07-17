use std::path::Path;

use gridcore::checksum;

#[test]
// Note: Please run this test after Minecrat files are downloaded!
fn cal_sha1() {
    let file_path = Path::new("./.minecraft/versions/1.21");
    let file_name = "1.21.jar";

    println!(
        "{}",
        checksum::calculate_sha1(file_path, file_name).unwrap()
    )
}
