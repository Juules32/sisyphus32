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

use move_gen::move_init;

use bit_move::{BitMove, MoveFlag};
use bitboard::Bitboard;
use board_state::BoardState;
use color::Color;
use move_list::MoveList;
use piece::PieceType;
use square::Square;


fn main() {
    dbg!("hi?");
    
    move_init::init();

    for square in Square::ALL_SQUARES {
        pl!(unsafe { move_init::PAWN_QUIET_MASKS[Color::WHITE][square] });
    }
}
