mod bitboard;
mod square;
mod rank;
mod file;
mod board_state;
mod piece;
mod macros;

use board_state::BoardState;
use piece::PieceType;
use square::Square;

fn main() {
    let mut bs = BoardState::starting_position();

    pl!(bs);
    bs.set_piece(PieceType::WP, Square::A4);
    pl!(bs);
}
