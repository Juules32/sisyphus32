#![allow(dead_code)]

// NOTE: The following combinations of features are not allowed to be used together:
#[cfg(all(feature = "perft_parallelize", feature = "perft_single_thread"))]
compile_error!("feature \"perft_parallelize\" and feature \"perft_single_thread\" cannot be enabled at the same time!");

#[cfg(all(feature = "board_representation_bitboard", feature = "board_representation_array"))]
compile_error!("feature \"board_representation_bitboard\" and feature \"board_representation_array\" cannot be enabled at the same time!");

#[cfg(all(feature = "revert_with_clone", feature = "revert_with_undo_move"))]
compile_error!("feature \"revert_with_clone\" and feature \"revert_with_undo_move\" cannot be enabled at the same time!");

#[cfg(all(feature = "sliders_magic_bitboards", feature = "sliders_on_the_fly"))]
compile_error!("feature \"sliders_magic_bitboards\" and feature \"sliders_on_the_fly\" cannot be enabled at the same time!");

#[cfg(
    any(
        all(feature = "search_minimax", feature = "search_negamax"),
        all(feature = "search_minimax", feature = "search_random"),
        all(feature = "search_negamax", feature = "search_random")
    )
)]
compile_error!("only one of the following features can be enabled at the same time: \"search_minimax\", \"search_negamax\", \"search_random\"!");

#[cfg(all(feature = "eval_basic", feature = "eval_piece_positions"))]
compile_error!("feature \"eval_basic\" and feature \"eval_piece_positions\" cannot be enabled at the same time!");

// NOTE: The following pairs of features are too unpractical to be used together:
#[cfg(all(feature = "revert_with_undo_move", feature = "board_representation_array"))]
compile_error!("feature \"revert_with_undo_move\" and feature \"board_representation_array\" cannot be enabled at the same time!");

pub mod bit_move;
pub mod bitboard;
pub mod position;
pub mod castling_rights;
pub mod color;
pub mod uci;
pub mod fen;
pub mod zobrist;
pub mod file;
pub mod macros;
pub mod magic_bitboards;
pub mod move_masks;
pub mod move_list;
pub mod piece;
pub mod rank;
pub mod square;
pub mod transposition_table;
pub mod timer;
pub mod perft;
pub mod bit_twiddles;
pub mod move_flag;
pub mod search;
pub mod eval;
pub mod rng;
pub mod move_generation;
