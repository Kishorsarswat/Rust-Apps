use chrono::Utc;
use std::process::Command;

fn main() {
    // Get the current UTC time as the build time
    let build_time = { Utc::now().to_rfc3339() };
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // Get the latest Git commit hash
    let commit_hash = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
}
