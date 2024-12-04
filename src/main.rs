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
mod todo;

use board_state::BoardState;
use castling_rights::CastlingRights;
use piece::PieceType;
use square::Square;

fn main() {

    move_init::init();

    pl!(move_init::generate_bishop_moves_on_the_fly(Square::A4, Square::D7.to_bb()));

    pl!(move_gen::get_queen_mask(Square::D4, Square::E4.to_bb()));

    let mut bs = BoardState::default();
    bs.set_piece(PieceType::WK, Square::E1);
    bs.set_piece(PieceType::WN, Square::C6);
    bs.set_piece(PieceType::BK, Square::A8);
    bs.set_piece(PieceType::WP, Square::B7);
    bs.set_piece(PieceType::WB, Square::G2);
    bs.set_piece(PieceType::BP, Square::F1);
    bs.set_piece(PieceType::WP, Square::E5);
    bs.en_passant_sq = Square::D6;
    bs.castling_rights = CastlingRights::DEFAULT;
    bs.populate_occupancies();

    let ml = move_gen::generate_moves(&bs);
    pl!(ml);
    pl!(bs);
}
