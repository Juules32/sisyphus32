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
mod magic_bitboards;

use bit_move::{BitMove, MoveFlag};
use board_state::BoardState;
use file::File;
use magic_bitboards::MagicBitboardGenerator;
use move_gen::move_init;
use bitboard::Bitboard;
use piece::PieceType;
use square::Square;

fn main() {
    unsafe { move_init::init() };

    pl!(move_init::get_bishop_moves_on_the_fly(Square::A4, Square::D7.to_bb()));

    let mut mbg = MagicBitboardGenerator{ seed: 1804289383 };

    mbg.print_magic_bitboards();
}
