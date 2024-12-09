use crate::bitboard::Bitboard;
use crate::file::File;
use crate::rank::Rank;
use core::fmt;
use std::mem::transmute;
use std::ops::{Index, IndexMut};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
    None
}

impl Square {
    #[inline(always)]
    pub fn to_bb(self) -> Bitboard {
        Bitboard::from(1 << (self as u64))
    }

    #[inline(always)]
    pub fn rank(self) -> Rank {
        Rank::from(self as u8 >> 3 & 0b0000_0111)
    }

    #[inline(always)]
    pub fn file(self) -> File {
        File::from(self as u8 & 0b0000_0111)
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

#[allow(dead_code)]
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

impl From<u8> for Square {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

pub struct SquareParseError(&'static str);

impl TryFrom<&str> for Square {
    type Error = SquareParseError;

    #[inline(always)]
    fn try_from(sq_str: &str) -> Result<Self, Self::Error> {
        if sq_str.len() != 2 {
            return Err(SquareParseError("Invalid string length!"));
        }

        let mut chars_iter = sq_str.chars();
        let file_char = chars_iter.next().ok_or(SquareParseError("Missing file character"))?;
        let rank_char = chars_iter.next().ok_or(SquareParseError("Missing rank character"))?;

        Ok(Self::from(Rank::from(rank_char) as u8 * 8 + File::from(file_char) as u8))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Self::None {
            f.pad("No Square")
        } else {
            f.pad(&format!("{}{}", self.file(), self.rank()))
        }
    }
}
