#![allow(dead_code)]

// NOTE: The following pairs of features are not allowed to be used together:
#[cfg(all(feature = "perft_parallelize", feature = "perft_single_thread"))]
compile_error!("feature \"perft_parallelize\" and feature \"perft_single_thread\" cannot be enabled at the same time!");

#[cfg(all(feature = "board_representation_bitboard", feature = "board_representation_array"))]
compile_error!("feature \"board_representation_bitboard\" and feature \"board_representation_array\" cannot be enabled at the same time!");

#[cfg(all(feature = "revert_with_clone", feature = "revert_with_undo_move"))]
compile_error!("feature \"revert_with_clone\" and feature \"revert_with_undo_move\" cannot be enabled at the same time!");

#[cfg(all(feature = "sliders_magic_bitboards", feature = "sliders_on_the_fly"))]
compile_error!("feature \"sliders_magic_bitboards\" and feature \"sliders_on_the_fly\" cannot be enabled at the same time!");

#[cfg(all(feature = "revert_with_undo_move", feature = "board_representation_array"))]
compile_error!("feature \"revert_with_undo_move\" and feature \"board_representation_array\" cannot be enabled at the same time!");

mod bit_move;
mod bitboard;
mod position;
mod castling_rights;
mod color;
mod uci;
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
mod search;
mod eval;
mod move_generation;

use uci::Uci;

fn main() {
    move_masks::init();
    Uci::default().init();
}
