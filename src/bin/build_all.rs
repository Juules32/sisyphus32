// Recommended usage: `cargo run --bin build_all`
// NOTE: This has only been tested for windows

use std::{env, process::Command};

const PROFILE_NAME: &str = "release-all";
const PACKAGE_NAME: &str = "sisyphus32";

const BUILDS: &[&str] = &[
    "basic",
    "parallel_array",
    "array_copy_tt_killer",
];

fn main() {
    env::set_var("RUSTFLAGS", "-Awarnings");

    for feature_name in BUILDS {
        // Build feature binary
        let status = Command::new("cargo")
            .args(["build", &format!("--profile={PROFILE_NAME}"), "--no-default-features", "--features", feature_name, "--bin", PACKAGE_NAME])
            .status()
            .expect("Failed to execute cargo build");

        if !status.success() {
            eprintln!("Build failed for feature: {}", feature_name);
            continue;
        }

        // Rename binary
        let target_dir = format!("target/{PROFILE_NAME}");
        let from = format!("{target_dir}/{PACKAGE_NAME}.exe");
        let to = format!("{target_dir}/{PACKAGE_NAME}_{feature_name}.exe");

        std::fs::rename(&from, &to).expect("Failed to rename binary");
        println!("Built and renamed: {}", to);
    }
}
