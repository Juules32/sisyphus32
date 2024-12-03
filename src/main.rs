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

use bit_move::{BitMove, MoveFlag};
use board_state::BoardState;
use file::File;
use magic_bitboards::MagicBitboardGenerator;
use bitboard::Bitboard;
use piece::PieceType;
use square::Square;

fn main() {
    unsafe { move_init::init() };

    pl!(move_init::generate_bishop_moves_on_the_fly(Square::A4, Square::D7.to_bb()));

    pl!(move_gen::get_queen_mask(Square::D4, Square::E4.to_bb()));

    let mut bs = BoardState::default();
    bs.set_piece(PieceType::WK, Square::D8);
    bs.set_piece(PieceType::WN, Square::C6);
    bs.set_piece(PieceType::BK, Square::A8);
    bs.set_piece(PieceType::WP, Square::B7);

    bs.populate_occupancies();

    pl!(move_gen::generate_moves(&bs));
    pl!(bs);
}
