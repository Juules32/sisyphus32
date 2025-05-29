// Recommended usage: `cargo run --bin test_all`

use std::{env, process::Command};

use sisyphus32::{BASE_FEATURES, FEATURES};

fn main() {
    env::set_var("RUSTFLAGS", "-Awarnings");
    env::set_var("RUST_BACKTRACE", "1");

    for feature_name in BASE_FEATURES.iter() {
        // Build feature binary
        let status = Command::new("cargo")
            .args(["test", "--release", "--no-default-features", "--features", &format!("{feature_name}, small_tt"), "--", "--test-threads=1"])
            .status()
            .expect("Failed to execute cargo test");

        if !status.success() {
            eprintln!("Test failed for feature: {feature_name}");
            panic!("Tests failed! Exiting early...");  // Force exit with failure
        }
    }

    for feature_name in FEATURES.iter() {
        // Build feature binary
        let status = Command::new("cargo")
            .args(["test", "--release", "--features", &format!("{feature_name}, small_tt"), "--", "--test-threads=1"])
            .status()
            .expect("Failed to execute cargo test");

        if !status.success() {
            eprintln!("Test failed for feature: {feature_name}");
            panic!("Tests failed! Exiting early...");  // Force exit with failure
        }
    }

    println!("All tests passed!");
}
