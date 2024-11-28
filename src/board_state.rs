use core::fmt;

use crate::{bitboard::Bitboard, piece::PieceType, square::Square};

pub struct BoardState {
    pub bbs: [Bitboard; 12],
    pub wo: Bitboard,
    pub bo: Bitboard,
    pub ao: Bitboard,
}

impl BoardState {

    #[inline(always)]
    pub fn merge_occupancies(&mut self) {
        self.ao = self.wo | self.bo;
    }

    #[inline(always)]
    pub fn populate_occupancies(&mut self) {
        self.wo = 
            self.bbs[PieceType::WP] | 
            self.bbs[PieceType::WN] |
            self.bbs[PieceType::WB] |
            self.bbs[PieceType::WR] |
            self.bbs[PieceType::WQ] |
            self.bbs[PieceType::WK];
        self.bo = 
            self.bbs[PieceType::BP] | 
            self.bbs[PieceType::BN] |
            self.bbs[PieceType::BB] |
            self.bbs[PieceType::BR] |
            self.bbs[PieceType::BQ] |
            self.bbs[PieceType::BK];
        
        self.merge_occupancies();
    }

    pub fn starting_position() -> BoardState {
        BoardState {
            bbs: [
                Bitboard::WP,
                Bitboard::WN,
                Bitboard::WB,
                Bitboard::WR,
                Bitboard::WQ,
                Bitboard::WK,
                Bitboard::BP,
                Bitboard::BN,
                Bitboard::BB,
                Bitboard::BR,
                Bitboard::BQ,
                Bitboard::BK,
            ],
            wo: Bitboard::WHITE_PIECES,
            bo: Bitboard::BLACK_PIECES,
            ao: Bitboard::ALL_PIECES
        }
    }

    #[inline(always)]
    pub fn set_piece(&mut self, piece: PieceType, sq: Square) {
        self.bbs[piece].set_sq(sq);
    }
}

impl Default for BoardState {
    fn default() -> BoardState {
        BoardState {
            bbs: [Bitboard::EMPTY; 12],
            wo: Bitboard::EMPTY,
            bo: Bitboard::EMPTY,
            ao: Bitboard::EMPTY
        }
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");
        for rank in 0..8 {
            s += &format!("  {}  ", 8 - rank);
            for file in 0..8 {
                let mut is_occupied = false;
                let sq = Square(rank * 8 + file);
                for piece_type in PieceType::ALL {
                    if Bitboard::is_set_sq(&self.bbs[piece_type], sq) {
                        s += &format!("{} ", piece_type.to_string());
                        is_occupied = true;
                    }
                }
                if !is_occupied {
                    s += ". ";
                }
            }
            s += "\n";
        }
        s += "\n     a b c d e f g h\n";
        f.pad(&s)
    }
}
