#![allow(dead_code, unused_imports)]

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
mod perft;
mod bit_twiddles;

use bit_move::{BitMove, MoveFlag};
use bitboard::Bitboard;
use board_state::BoardState;
use engine::Engine;
use piece::PieceType;
use square::Square;

fn main() {
    move_init::init();

    let mut bs = fen::parse(fen::KIWIPETE_POSITION).unwrap();
    let ml = move_gen::generate_moves(&bs);
    pl!(ml);
    pl!(bs);

    pl!(move_gen::get_bishop_mask_old(Square::C6, bs.ao) & !bs.wo);

    perft::perft_test(&mut bs, 1, true);

    let mut bb = Bitboard::EMPTY;
    bb.set_sq(Square::G4);

    perft::short_perft_tests();
}
