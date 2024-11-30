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
mod move_gen;

use std::io::stdin;

use move_gen::move_init;

use bit_move::{BitMove, MoveFlag};
use bitboard::Bitboard;
use board_state::BoardState;
use color::Color;
use file::File;
use rank::Rank;
use move_list::MoveList;
use piece::PieceType;
use square::Square;


fn main() {
    unsafe { move_init::init() };

    pl!(Square::A3.file());
    pl!(File::FA);

    pl!(Square::A3.rank());
    pl!(Rank::R3);

    for square in Square::ALL_SQUARES {
        pl!(unsafe { move_init::BISHOP_MASKS[square] });
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Did not get correct input");
    }
}
