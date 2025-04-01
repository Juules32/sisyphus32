use core::fmt;
use std::{ops::{Index, IndexMut}, mem::transmute};

use crate::{color::Color, consts::{PIECE_TYPE_COUNT, PLAYER_COUNT}};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
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
}

impl Piece {
    pub const WHITE_PIECES: [Piece; PIECE_TYPE_COUNT / 2] = [
        Piece::WP,
        Piece::WN,
        Piece::WB,
        Piece::WR,
        Piece::WQ,
        Piece::WK,
    ];

    pub const BLACK_PIECES: [Piece; PIECE_TYPE_COUNT / 2] = [
        Piece::BP,
        Piece::BN,
        Piece::BB,
        Piece::BR,
        Piece::BQ,
        Piece::BK,
    ];

    pub const ALL_PIECES: [Piece; PIECE_TYPE_COUNT] = [
        Piece::WP,
        Piece::WN,
        Piece::WB,
        Piece::WR,
        Piece::WQ,
        Piece::WK,
        Piece::BP,
        Piece::BN,
        Piece::BB,
        Piece::BR,
        Piece::BQ,
        Piece::BK,
    ];

    pub const ALL_PIECES_EXPECT_PAWNS_AND_KINGS: [Piece; PIECE_TYPE_COUNT - PLAYER_COUNT * 2] = [
        Piece::WN,
        Piece::WB,
        Piece::WR,
        Piece::WQ,
        Piece::BN,
        Piece::BB,
        Piece::BR,
        Piece::BQ,
    ];

    #[inline(always)]
    pub fn color(self) -> Color {
        if self as u8 <= 5 {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl<T, const N: usize> Index<Piece> for [T; N] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T, const N: usize> IndexMut<Piece> for [T; N] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl From<u8> for Piece {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

impl From<char> for Piece {
    #[inline(always)]
    fn from(ch: char) -> Self {
        match ch {
            'P' => Piece::WP,
            'N' => Piece::WN,
            'B' => Piece::WB,
            'R' => Piece::WR,
            'Q' => Piece::WQ,
            'K' => Piece::WK,
            'p' => Piece::BP,
            'n' => Piece::BN,
            'b' => Piece::BB,
            'r' => Piece::BR,
            'q' => Piece::BQ,
            'k' => Piece::BK,
            _ => panic!("Illegal piece char found!"),
        }
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> char {
        match piece {
            Piece::WP => 'P',
            Piece::WN => 'N',
            Piece::WB => 'B',
            Piece::WR => 'R',
            Piece::WQ => 'Q',
            Piece::WK => 'K',
            Piece::BP => 'p',
            Piece::BN => 'n',
            Piece::BB => 'b',
            Piece::BR => 'r',
            Piece::BQ => 'q',
            Piece::BK => 'k',
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Piece::WP => "♟",
            Piece::WN => "♞",
            Piece::WB => "♝",
            Piece::WR => "♜",
            Piece::WQ => "♛",
            Piece::WK => "♚",
            Piece::BP => "♙",
            Piece::BN => "♘",
            Piece::BB => "♗",
            Piece::BR => "♖",
            Piece::BQ => "♕",
            Piece::BK => "♔",
        };
        f.pad(s)
    }
}
