// NOTE: The following combinations of features are not allowed to be used together:
#[cfg(all(feature = "unit_bb", feature = "unit_bb_array"))]
compile_error!("feature \"unit_bb\" and feature \"unit_bb_array\" cannot be enabled at the same time!");

#[cfg(all(feature = "unit_revert_clone", feature = "unit_revert_undo"))]
compile_error!("feature \"unit_revert_clone\" and feature \"unit_revert_undo\" cannot be enabled at the same time!");

#[cfg(all(feature = "unit_minimax", feature = "unit_negamax"))]
compile_error!("feature \"unit_minimax\" and feature \"unit_negamax\" cannot be enabled at the same time!");

#[cfg(all(feature = "unit_revert_undo", feature = "unit_bb_array"))]
compile_error!("feature \"unit_revert_undo\" and feature \"unit_bb_array\" cannot be enabled at the same time!");

mod bit_move;
mod bit_twiddles;
mod bitboard;
mod castling_rights;
mod color;
mod consts;
mod ctor;
mod error;
mod eval_move;
mod eval_position;
mod fen;
mod file;
mod history_heuristic;
mod killer_moves;
mod magic_numbers;
mod move_flag;
mod move_generation;
mod move_list;
mod move_masks;
mod opening_book;
mod perft;
mod piece;
mod position;
mod rank;
mod rng;
mod score;
mod search;
mod square;
mod syzygy;
mod timer;
mod transposition_table;
mod uci;
mod versions;
mod zobrist;

pub use bit_move::BitMove;
pub use fen::FenString;
pub use move_generation::{Legal, MoveGeneration, PseudoLegal};
pub use perft::Perft;
pub use position::Position;
pub use search::Search;
pub use uci::Uci;
pub use versions::{BASE_VERSIONS, VERSIONS};
pub use zobrist::ZobristKey;

pub(crate) use color::Color;
pub(crate) use eval_move::EvalMove;
pub(crate) use move_flag::MoveFlag;
pub(crate) use piece::Piece;
pub(crate) use score::Score;
pub(crate) use square::Square;
pub(crate) use eval_position::EvalPosition;
pub(crate) use move_masks::MoveMasks;
pub(crate) use transposition_table::{TranspositionTable, TTNodeType, TTData};
pub(crate) use history_heuristic::HistoryHeuristic;
pub(crate) use killer_moves::KillerMoves;
pub(crate) use bitboard::Bitboard;
pub(crate) use file::File;
pub(crate) use rank::Rank;
pub(crate) use castling_rights::CastlingRights;
pub(crate) use rng::RandomNumberGenerator;
pub(crate) use bit_move::{Move, ScoringMove};
pub(crate) use move_list::MoveList;
pub(crate) use opening_book::OpeningBook;
pub(crate) use syzygy::SyzygyTablebase;
pub(crate) use timer::Timer;
