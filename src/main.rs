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

use bit_move::{BitMove, MoveFlag};
use board_state::BoardState;
use piece::PieceType;
use square::Square;

fn main() {
    let mut bs = BoardState::starting_position();
    let bm = BitMove::encode(Square::B2, Square::B5, PieceType::WP, PieceType::None, MoveFlag::Null);
    bs.make_move(bm);
    pl!(bs);

    let bm = BitMove::encode(Square::A7, Square::A5, PieceType::BP, PieceType::None, MoveFlag::BDoublePawn);
    bs.make_move(bm);
    pl!(bs);

    let bm = BitMove::encode(Square::B5, Square::A6, PieceType::WP, PieceType::None, MoveFlag::WEnPassant);
    bs.make_move(bm);
    pl!(bs);

    let bm = BitMove::encode(Square::D7, Square::D6, PieceType::BP, PieceType::None, MoveFlag::Null);
    bs.make_move(bm);
    pl!(bs);

    let bm = BitMove::encode(Square::A6, Square::B7, PieceType::WP, PieceType::BP, MoveFlag::Null);
    bs.make_move(bm);
    pl!(bs);

    let bm = BitMove::encode(Square::E8, Square::D7, PieceType::BK, PieceType::None, MoveFlag::Null);
    bs.make_move(bm);
    pl!(bs);

    let bm = BitMove::encode(Square::H1, Square::H4, PieceType::WR, PieceType::None, MoveFlag::Null);
    bs.make_move(bm);
    pl!(bs);
}
