/// Really simple CLI app to obtain the date of the last Mac OS Time Machine backup
/// e.g. `tmlatestbackup`

use std::process::{exit, Command};

fn main() {
    // Try several times because sometimes, when the disk is idle, the command fails
    for _i in 0..3 {
        // Call 'tmutil latestbackup'
        let output = Command::new("/usr/bin/tmutil")
            .arg("latestbackup")
            .output()
            .expect("Error: Failed to execute 'tmutil latestbackup'.");
        // Check call result
        if output.status.success() {
            println!(
                "{}",
                String::from_utf8_lossy(&output.stdout)
                    .split("/")
                    .last()
                    .unwrap()
            );
            exit(0);
        }
    }

    // If never attemp finish on success
    println!("Error: Failed to execute 'tmutil latestbackup'. Is connected the Time Machine disk?");
    exit(1);
}
