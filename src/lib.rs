#![allow(dead_code)]

// NOTE: The following combinations of features are not allowed to be used together:
#[cfg(all(feature = "unit_bb", feature = "unit_bb_array"))]
compile_error!("feature \"unit_bb\" and feature \"unit_bb_array\" cannot be enabled at the same time!");

#[cfg(all(feature = "unit_revert_clone", feature = "unit_revert_undo"))]
compile_error!("feature \"unit_revert_clone\" and feature \"unit_revert_undo\" cannot be enabled at the same time!");

#[cfg(all(feature = "unit_minimax", feature = "unit_negamax"))]
compile_error!("feature \"unit_minimax\" and feature \"unit_negamax\" cannot be enabled at the same time!");

#[cfg(all(feature = "unit_revert_undo", feature = "unit_bb_array"))]
compile_error!("feature \"unit_revert_undo\" and feature \"unit_bb_array\" cannot be enabled at the same time!");

pub mod bit_move;
pub mod bitboard;
pub mod position;
pub mod castling_rights;
pub mod color;
pub mod uci;
pub mod fen;
pub mod zobrist;
pub mod file;
pub mod magic_numbers;
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
pub mod eval_move;
pub mod eval_position;
pub mod killer_moves;
pub mod history_heuristic;
pub mod rng;
pub mod move_generation;
pub mod versions;
pub mod syzygy;
pub mod consts;
pub mod score;
pub mod opening_book;
mod ctor;
