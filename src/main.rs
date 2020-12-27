/// Really simple CLI app to obtain the date of the last Mac OS Time Machine backup
/// e.g. `tmlatestbackup`

use std::process::Command;

fn main() {
    let output = Command::new("/usr/bin/tmutil")
        .arg("latestbackup")
        .output()
        .expect("Error: Failed to execute 'tmutil'.");

    assert!(output.status.success());
    println!(
        "{}",
        String::from_utf8_lossy(&output.stdout)
            .split("/")
            .last()
            .unwrap()
    );
}
