#![allow(dead_code, unused_imports)]

mod bit_move;
mod bitboard;
mod board_state;
mod castling_rights;
mod color;
mod engine;
mod fen;
mod file;
mod macros;
mod magic_bitboards;
mod move_gen;
mod move_init;
mod move_list;
mod piece;
mod rank;
mod square;
mod timer;
mod perft;

use bit_move::{BitMove, MoveFlag};
use board_state::BoardState;
use engine::Engine;
use piece::PieceType;
use square::Square;

fn main() {
    move_init::init();

    let bs = BoardState::starting_position();
    let ml = move_gen::generate_moves(&bs);
    pl!(ml);

    perft::perft_tests();
}
