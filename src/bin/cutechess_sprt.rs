// Recommended usage: `cargo run --bin cutechess_sprt <new_version> <old_version>`
// NOTE: This has only been tested for windows with cutechess-cli-1.3.1
// NOTE: This requires cutechess-cli to be installed (https://github.com/cutechess/cutechess/releases)

use std::{env, process::{exit, Command}, time::{SystemTime, UNIX_EPOCH}};

const OPENINGS_NAME: &str = "openings";
const PROFILE_NAME: &str = "release-all";
const OPENING_PLIES: &str = "6";
const GAMES: &str = "1";
const ROUNDS: &str = "1000";
const REPEAT: &str = "2";
const MAX_MOVES: &str = "200";
const CONCURRENCY: &str = "4";
const RATING_INTERVAL: &str = "10";
const ELO0: &str = "0";
const ELO1: &str = "10";
const ALPHA: &str = "0.05";
const BETA: &str = "0.05";

fn main() {
    env::set_var("RUSTFLAGS", "-Awarnings");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <new_version> <old_version>", args[0]);
        exit(1);
    }

    let new_version = &args[1];
    let old_version = &args[2];

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs();

    let status = Command::new("cutechess-cli")
        .args([
            "-engine", &format!("name=sisyphus32_{new_version}"), &format!("cmd=./sisyphus32_{new_version}.exe"),
            "-engine", &format!("name=sisyphus32_{old_version}"), &format!("cmd=./sisyphus32_{old_version}.exe"),
            "-each", "tc=1+0.1", &format!("dir=./target/{PROFILE_NAME}/"), "proto=uci",
            "-openings", &format!("file=openings/{OPENINGS_NAME}.pgn"), "order=random", &format!("plies={OPENING_PLIES}"),
            "-games", GAMES, "-rounds", ROUNDS, "-repeat", REPEAT, "-maxmoves", MAX_MOVES, "-concurrency", CONCURRENCY, "-ratinginterval", RATING_INTERVAL,
            "-sprt", &format!("elo0={ELO0}"), &format!("elo1={ELO1}"), &format!("alpha={ALPHA}"), &format!("beta={BETA}"),
            "-pgnout", &format!("./pgn_results/{timestamp}.pgn"), "fi",
        ])
        .status()
        .expect("Failed to execute cutechess-cli command");

    if !status.success() {
        eprintln!("SPRT test failed");
    }
}
