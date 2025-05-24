// Recommended usage: `cargo run --bin test_all`

use std::{env, process::Command};

use sisyphus32::versions::{BASE_VERSIONS, VERSIONS};

fn main() {
    env::set_var("RUSTFLAGS", "-Awarnings");
    env::set_var("RUST_BACKTRACE", "1");

    for version_name in VERSIONS.iter().chain(BASE_VERSIONS.iter()) {
        // Build feature binary
        let status = Command::new("cargo")
            .args(["test", "--release", "--no-default-features", "--features", &format!("{version_name},unit_small_tt")])
            .status()
            .expect("Failed to execute cargo test");

        if !status.success() {
            eprintln!("Test failed for version: {}", version_name);
            panic!("Tests failed! Exiting early...");  // Force exit with failure
        }
    }

    println!("All tests passed!");
}
