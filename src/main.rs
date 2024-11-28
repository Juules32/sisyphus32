mod bitboard;
mod square;
mod rank;
mod file;
mod board_state;
mod piece;
mod macros;

use bitboard::Bitboard;
use square::{Square, ALL_SQUARES};
use board_state::BoardState;
use piece::PieceType;

fn main() {
    pl!(Bitboard::FILE_A);
    pl!(Bitboard::FILE_B);
    pl!(Bitboard::FILE_C);
    pl!(Bitboard::FILE_D);
    pl!(Bitboard::FILE_E);
    pl!(Bitboard::FILE_F);
    pl!(Bitboard::FILE_G);
    pl!(Bitboard::FILE_H);
    pl!(Bitboard::RANK_1);
    pl!(Bitboard::RANK_2);
    pl!(Bitboard::RANK_3);
    pl!(Bitboard::RANK_4);
    pl!(Bitboard::RANK_5);
    pl!(Bitboard::RANK_6);
    pl!(Bitboard::RANK_7);
    pl!(Bitboard::RANK_8);
    pl!(Bitboard::BLACK_SQUARES);
    pl!(Bitboard::WHITE_SQUARES);

    for square in ALL_SQUARES {
        pl!(square);
        pl!(square.to_bb());
    }

    let mut bb = Bitboard::EMPTY;
    dbg!(bb.is_empty());
    
    dbg!(bb.is_set_sq(Square::A4));
    bb.set_sq(Square::A4);
    bb.set_sq(Square::B5);
    dbg!(bb.is_set_sq(Square::A4));

    dbg!(bb.is_empty());

    pl!(bb);

    bb.pop_sq(Square::A8);
    bb.pop_sq(Square::A4);

    pl!(bb);

    dbg!(bb.is_not_empty());
    bb.pop_sq(Square::B5);
    dbg!(bb.is_not_empty());

    let bb2 = Bitboard(2425);
    let bb3 = Bitboard(222);
    let sq = Square(2);

    pl!((bb2 | bb3));
    pl!((bb2 | bb3));
    pl!(bb2);
    pl!(sq.to_bb());
    pl!((bb2 | sq.to_bb()));

    let bs = BoardState::starting_position();
    println!("{}", bs);


    let mut bs = BoardState {..Default::default()};
    pl!(bs);
    bs.set_piece(PieceType::BB, Square::A5);
    bs.set_piece(PieceType::WN, Square::H6);
    bs.populate_occupancies();
    pl!(bs.wo);
    pl!(bs.bo);
    pl!(bs.ao);
    pl!(bs);
}
