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

use bit_move::{BitMove, MoveFlag};
use board_state::BoardState;
use engine::Engine;
use piece::PieceType;
use square::Square;

fn main() {
    move_init::init();

    let bs = fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR d KQkq -").unwrap();

    pl!(bs);
    let mut engine = Engine {
        board_state: bs,
        nodes: 0,
    };

    for i in 1..=7 {
        engine.perft_test(i);
    }
}
