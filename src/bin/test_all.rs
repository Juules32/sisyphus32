// Recommended usage: `cargo run --bin test_all`

use std::{env, process::Command};

use sisyphus32::{BASE_FEATURES, FEATURES, OTHER_FEATURES};

fn main() {
    env::set_var("RUSTFLAGS", "-Awarnings");
    env::set_var("RUST_BACKTRACE", "1");

    for feature_name in BASE_FEATURES.iter().chain(FEATURES).chain(OTHER_FEATURES) {
        println!("Testing feature: {feature_name}");

        let status = Command::new("cargo")
            .args(["test", "--release", "--no-default-features", "--features", feature_name, "--", "--test-threads=1"])
            .status()
            .expect("Failed to execute cargo test");

        if !status.success() {
            eprintln!("Test failed for feature: {feature_name}");
            panic!("Tests failed! Exiting early...");
        }
    }

    println!("All tests passed!");
}
