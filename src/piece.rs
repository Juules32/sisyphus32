use core::fmt;
use std::ops::{Index, IndexMut};

use crate::{bitboard::Bitboard, color::Color};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    WP = 0b0,
    WN = 0b1,
    WB = 0b10,
    WR = 0b11,
    WQ = 0b100,
    WK = 0b101,
    BP = 0b110,
    BN = 0b111,
    BB = 0b1000,
    BR = 0b1001,
    BQ = 0b1010,
    BK = 0b1011,
    None = 0b1100,
}

impl PieceType {
    pub const WHITE_PIECES: [PieceType; 6] = [
        PieceType::WP,
        PieceType::WN,
        PieceType::WB,
        PieceType::WR,
        PieceType::WQ,
        PieceType::WK,
    ];

    pub const BLACK_PIECES: [PieceType; 6] = [
        PieceType::BP,
        PieceType::BN,
        PieceType::BB,
        PieceType::BR,
        PieceType::BQ,
        PieceType::BK,
    ];

    pub const ALL_PIECES: [PieceType; 12] = [
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

    #[inline]
    pub fn color(self) -> Color {
        if PieceType::WHITE_PIECES.contains(&self) {
            Color::WHITE
        }
        else if PieceType::BLACK_PIECES.contains(&self) {
            Color::BLACK
        }
        else {
            Color::NULL
        }
    }
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
            PieceType::None => "No Piece",
        };
        f.pad(s)
    }
}
