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

use board_state::BoardState;
use engine::Engine;

fn main() {

    move_init::init();

    let bs = BoardState::starting_position();

    let mut engine = Engine {board_state: bs, nodes: 0};

    engine.perft_test(5);
}
