use std::env;
use std::io;

fn main() {
    println!("Hello, world!");
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!("BUILD_TIME={}", env!("BUILD_TIME"));
    println!("COMMIT_HASH={}", env!("COMMIT_HASH"));

    // Wait for user input before exiting
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
