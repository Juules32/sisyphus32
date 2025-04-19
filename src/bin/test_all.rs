// Recommended usage: `cargo run --bin test_all`

use std::{env, process::Command};

use sisyphus32::features::{BASE_FEATURES, FEATURES};

fn main() {
    env::set_var("RUSTFLAGS", "-Awarnings");

    for feature_name in FEATURES.iter().chain(BASE_FEATURES.iter()) {
        // Build feature binary
        let status = Command::new("cargo")
            .args(["test", "--release", "--no-default-features", "--features", &format!("{feature_name},unit_small_tt")])
            .status()
            .expect("Failed to execute cargo test");

        if !status.success() {
            eprintln!("Test failed for feature: {}", feature_name);
            panic!("Tests failed! Exiting early...");  // Force exit with failure
        }
    }

    println!("All tests passed!");
}
