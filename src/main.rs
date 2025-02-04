#![allow(dead_code, unused_imports, unused_assignments)]

// NOTE: The following pairs of features are not allowed to be used together:

#[cfg(all(feature = "perft_parallelize", feature = "perft_single_thread"))]
compile_error!("feature \"perft_parallelize\" and feature \"perft_single_thread\" cannot be enabled at the same time!");

#[cfg(all(feature = "board_representation_bitboard", feature = "board_representation_array"))]
compile_error!("feature \"board_representation_bitboard\" and feature \"board_representation_array\" cannot be enabled at the same time!");

#[cfg(all(feature = "revert_with_clone", feature = "revert_with_undo_move"))]
compile_error!("feature \"revert_with_clone\" and feature \"revert_with_undo_move\" cannot be enabled at the same time!");

mod bit_move;
mod bitboard;
mod position;
mod castling_rights;
mod color;
mod engine;
mod fen;
mod file;
mod macros;
mod magic_bitboards;
mod move_masks;
mod move_list;
mod piece;
mod rank;
mod square;
mod timer;
mod perft;
mod bit_twiddles;
mod move_flag;

fn main() {
    move_masks::init();
    perft::short_perft_tests();
}
