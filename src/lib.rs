#![allow(dead_code)]

// NOTE: The following combinations of features are not allowed to be used together:
#[cfg(all(feature = "bb", feature = "bb_array"))]
compile_error!("feature \"bb\" and feature \"bb_array\" cannot be enabled at the same time!");

#[cfg(all(feature = "revert_clone", feature = "revert_undo"))]
compile_error!("feature \"revert_clone\" and feature \"revert_undo\" cannot be enabled at the same time!");

#[cfg(all(feature = "minimax", feature = "negamax"))]
compile_error!("feature \"minimax\" and feature \"negamax\" cannot be enabled at the same time!");

#[cfg(all(feature = "revert_undo", feature = "bb_array"))]
compile_error!("feature \"revert_undo\" and feature \"bb_array\" cannot be enabled at the same time!");

mod bit_move;
mod bit_twiddles;
mod bitboard;
mod bot_game;
mod castling_rights;
mod color;
mod consts;
mod error;
mod eval_move;
mod eval_position;
mod features;
mod fen;
mod file;
mod history_heuristic;
mod killer_moves;
mod magic_numbers;
mod move_flag;
mod move_generation;
mod move_list;
mod move_masks;
#[cfg(feature = "opening_book")]
mod opening_book;
mod perft;
mod piece;
mod position;
mod rank;
mod rng;
mod score;
mod search;
mod square;
#[cfg(feature = "syzygy_tablebase")]
mod syzygy;
mod timer;
mod transposition_table;
mod uci;
mod zobrist;
#[cfg(feature = "parallelize")]
mod global_thread_pool;

///*--------------------------------*\\\
//    PUBLIC LIBRARY FUNCTIONALITY    \\
//\*--------------------------------*/\\
pub use bit_move::{Move, BitMove, ScoringMove};
pub use bot_game::BotGame;
pub use castling_rights::CastlingRights;
pub use color::Color;
pub use error::*;
pub use eval_move::EvalMove;
pub use eval_position::EvalPosition;
pub use features::{BASE_FEATURES, FEATURES};
pub use fen::FenString;
pub use file::File;
pub use move_flag::MoveFlag;
pub use move_generation::{Legal, Filter, MoveGeneration, PseudoLegal};
pub use move_list::MoveList;
pub use perft::Perft;
pub use piece::Piece;
pub use position::Position;
pub use rank::Rank;
pub use score::Score;
pub use search::Search;
pub use square::Square;
pub use timer::Timer;
pub use uci::Uci;
pub use zobrist::ZobristKey;
// NOTE: Zobrist and versions are only necessary to make public because of
// /src/bin and /tests.

///*--------------------------------*\\\
//     SHARED CRATE FUNCTIONALITY     \\
//\*--------------------------------*/\\
use bitboard::Bitboard;
use consts::*;
#[cfg(feature = "parallelize")]
use global_thread_pool::GlobalThreadPool;
use history_heuristic::HistoryHeuristic;
use killer_moves::KillerMoves;
use move_masks::MoveMasks;
#[cfg(feature = "opening_book")]
use opening_book::OpeningBook;
use rng::RandomNumberGenerator;
#[cfg(feature = "syzygy_tablebase")]
use syzygy::SyzygyTablebase;
use transposition_table::{TranspositionTable, TTNodeType, TTData};

///*--------------------------------*\\\
//      AUTO-INIT FUNCTIONALITY       \\
//\*--------------------------------*/\\
pub unsafe fn init() {
    MoveMasks::init_move_masks();
    EvalPosition::init_positional_masks();
    ZobristKey::init_zobrist_keys();
    TranspositionTable::init();

    #[cfg(feature = "parallelize")]
    GlobalThreadPool::init();
}

#[cfg(not(target_arch = "wasm32"))]
#[ctor::ctor]
unsafe fn ctor() {
    crate::init();
}
