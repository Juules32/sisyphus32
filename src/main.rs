mod bitboard;
mod square;
mod rank;
mod file;

use bitboard::Bitboard;
use square::{Square, ALL_SQUARES};

fn main() {
    Bitboard::FILE_A.print();
    Bitboard::FILE_B.print();
    Bitboard::FILE_C.print();
    Bitboard::FILE_D.print();
    Bitboard::FILE_E.print();
    Bitboard::FILE_F.print();
    Bitboard::FILE_G.print();
    Bitboard::FILE_H.print();
    Bitboard::RANK_1.print();
    Bitboard::RANK_2.print();
    Bitboard::RANK_3.print();
    Bitboard::RANK_4.print();
    Bitboard::RANK_5.print();
    Bitboard::RANK_6.print();
    Bitboard::RANK_7.print();
    Bitboard::RANK_8.print();
    Bitboard::BLACK_SQUARES.print();
    Bitboard::WHITE_SQUARES.print();

    for square in ALL_SQUARES {
        square.print();
        square.to_bb().print();
    }

    let mut bb = Bitboard(0);
    dbg!(bb.is_empty());
    
    dbg!(bb.is_set_sq(Square::A4));
    bb.set_sq(Square::A4);
    bb.set_sq(Square::B5);
    dbg!(bb.is_set_sq(Square::A4));

    dbg!(bb.is_empty());

    bb.print();

    bb.pop_sq(Square::A8);
    bb.pop_sq(Square::A4);

    bb.print();

    dbg!(bb.is_not_empty());
    bb.pop_sq(Square::B5);
    dbg!(bb.is_not_empty());

    let bb2 = Bitboard(2425);
    let bb3 = Bitboard(222);
    let sq = Square(2);

    (bb2 | bb3).print();
    (bb2 | bb3).print();
    bb2.print();
    sq.to_bb().print();
    (bb2 | sq.to_bb()).print();
}
