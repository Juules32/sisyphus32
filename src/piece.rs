use core::fmt;
use std::ops::{Index, IndexMut};

use crate::bitboard::Bitboard;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PieceType {
    WP = 0,
    WN = 1,
    WB = 2,
    WR = 3,
    WQ = 4,
    WK = 5,
    BP = 6,
    BN = 7,
    BB = 8,
    BR = 9,
    BQ = 10,
    BK = 11,
}

impl PieceType {
    pub const ALL: [PieceType; 12] = [
        PieceType::WP,
        PieceType::WN,
        PieceType::WB,
        PieceType::WR,
        PieceType::WQ,
        PieceType::WK,
        PieceType::BP,
        PieceType::BN,
        PieceType::BB,
        PieceType::BR,
        PieceType::BQ,
        PieceType::BK,
    ];
}

// Allows indexing with PieceType
impl Index<PieceType> for [Bitboard; 12] {
    type Output = Bitboard;

    fn index(&self, index: PieceType) -> &Self::Output {
        &self[index as usize]
    }
}

// Allows modifying array elements when indexing with PieceType
impl IndexMut<PieceType> for [Bitboard; 12] {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            PieceType::WP => "♟",
            PieceType::WN => "♞",
            PieceType::WB => "♝",
            PieceType::WR => "♜",
            PieceType::WQ => "♛",
            PieceType::WK => "♚",
            PieceType::BP => "♙",
            PieceType::BN => "♘",
            PieceType::BB => "♗",
            PieceType::BR => "♖",
            PieceType::BQ => "♕",
            PieceType::BK => "♔",
        };
        f.pad(s)
    }
}
