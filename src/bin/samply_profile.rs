use sisyphus32::{move_masks::MoveMasks, perft::Perft};

fn main() {
    MoveMasks::init();
    Perft::long_perft_tests();
}
