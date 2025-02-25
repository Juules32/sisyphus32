// Recommended usage: `cargo run --bin samply_profile <samply_binary_name>`
// NOTE: This has only been tested for windows
// NOTE: This requires samply to be installed

use std::{env, process::{exit, Command}};

const PROFILE_NAME: &str = "release-with-debug";

fn main() {
    env::set_var("RUSTFLAGS", "-Awarnings");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <binary_name>", args[0]);
        exit(1);
    }

    let samply_binary_name = &args[1];

    let status = Command::new("cargo")
        .args(["build", &format!("--profile={PROFILE_NAME}"), "--bin", samply_binary_name])
        .status()
        .expect("Failed to execute cargo build");

    if !status.success() {
        eprintln!("Build failed for binary: {}", samply_binary_name);
    }

    let status = Command::new("samply")
        .args(["record", &format!("target/{PROFILE_NAME}/{samply_binary_name}.exe")])
        .status()
        .expect("Failed to execute cargo build");

    if !status.success() {
        eprintln!("Samply profile failed for binary: {}", samply_binary_name);
    }
}
