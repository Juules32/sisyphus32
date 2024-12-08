mod bitboard;
mod square;
mod rank;
mod file;
mod board_state;
mod piece;
mod macros;
mod bit_move;
mod castling_rights;
mod color;
mod move_list;
mod move_init;
mod move_gen;
mod magic_bitboards;
mod engine;
mod timer;
mod fen;

use bit_move::{BitMove, MoveFlag};
use board_state::BoardState;
use engine::Engine;
use piece::PieceType;
use square::Square;

fn main() {
    move_init::init();

    let bs = fen::parse(fen::STARTING_POSITION);

    pl!(bs);
    let mut engine = Engine {board_state: bs, nodes: 0};

    for i in 1..=7 {
        engine.perft_test(i);
    }
}
