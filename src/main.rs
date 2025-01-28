#![allow(dead_code, unused_imports)]

mod bit_move;
mod bitboard;
mod position;
mod castling_rights;
mod color;
mod engine;
mod fen;
mod file;
mod macros;
mod magic_bitboards;
mod move_masks;
mod move_list;
mod piece;
mod rank;
mod square;
mod timer;
mod perft;
mod bit_twiddles;
mod move_flag;

fn main() {
    move_masks::init();
    perft::short_perft_tests();
}
