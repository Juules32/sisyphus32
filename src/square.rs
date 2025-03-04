use crate::{rank::{Rank, RankParseError}, bitboard::Bitboard, file::{File, FileParseError}};
use core::fmt;
use std::{ops::{Index, IndexMut}, mem::transmute};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

impl Square {
    pub const ALL_SQUARES: [Square; 64] = [
        Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
        Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
        Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
        Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
        Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
        Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
        Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
        Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1
    ];

    #[inline(always)]
    pub fn to_bb(self) -> Bitboard {
        Bitboard::from(1 << (self as u64))
    }

    #[inline(always)]
    pub fn rank(self) -> Rank {
        Rank::from(self.rank_as_u8())
    }

    #[inline(always)]
    pub fn file(self) -> File {
        File::from(self.file_as_u8())
    }

    #[inline(always)]
    pub fn rank_as_u8(self) -> u8 {
        self as u8 >> 3 & 0b0000_0111
    }

    #[inline(always)]
    pub fn file_as_u8(self) -> u8 {
        self as u8 & 0b0000_0111
    }

    #[inline(always)]
    pub fn above(self) -> Square {
        Square::from(self as u8 - 8)
    }

    #[inline(always)]
    pub fn below(self) -> Square {
        Square::from(self as u8 + 8)
    }

    #[inline(always)]
    pub fn left(self) -> Square {
        Square::from(self as u8 - 1)
    }

    #[inline(always)]
    pub fn right(self) -> Square {
        Square::from(self as u8 + 1)
    }
}

impl<T, const N: usize> Index<Square> for [T; N] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T, const N: usize> IndexMut<Square> for [T; N] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl From<Bitboard> for Square {
    #[inline(always)]
    fn from(bitboard: Bitboard) -> Self {
        debug_assert_eq!(bitboard.count_bits(), 1);
        bitboard.get_lsb()
    }
}

impl From<u8> for Square {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

#[derive(Debug)]
pub struct SquareParseError(pub &'static str);

impl TryFrom<&str> for Square {
    type Error = SquareParseError;

    fn try_from(sq_str: &str) -> Result<Self, Self::Error> {
        if sq_str.len() != 2 {
            return Err(SquareParseError("Invalid string length!"));
        }

        let mut chars_iter = sq_str.chars();
        let file_char = chars_iter.next().ok_or(SquareParseError("Missing file character"))?;
        let rank_char = chars_iter.next().ok_or(SquareParseError("Missing rank character"))?;

        let rank = Rank::try_from(rank_char).map_err(|RankParseError(msg)| SquareParseError(msg))?;
        let file = File::try_from(file_char).map_err(|FileParseError(msg)| SquareParseError(msg))?;

        Ok(Self::from(rank as u8 * 8 + file as u8))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{}{}", self.file(), self.rank()))
    }
}
