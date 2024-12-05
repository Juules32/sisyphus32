use crate::bitboard::Bitboard;
use crate::file::File;
use crate::rank::Rank;
use core::fmt;
use std::mem::transmute;
use std::ops::{Index, IndexMut};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Square {
    A8 = 0,
    B8 = 1,
    C8 = 2,
    D8 = 3,
    E8 = 4,
    F8 = 5,
    G8 = 6,
    H8 = 7,
    A7 = 8,
    B7 = 9,
    C7 = 10,
    D7 = 11,
    E7 = 12,
    F7 = 13,
    G7 = 14,
    H7 = 15,
    A6 = 16,
    B6 = 17,
    C6 = 18,
    D6 = 19,
    E6 = 20,
    F6 = 21,
    G6 = 22,
    H6 = 23,
    A5 = 24,
    B5 = 25,
    C5 = 26,
    D5 = 27,
    E5 = 28,
    F5 = 29,
    G5 = 30,
    H5 = 31,
    A4 = 32,
    B4 = 33,
    C4 = 34,
    D4 = 35,
    E4 = 36,
    F4 = 37,
    G4 = 38,
    H4 = 39,
    A3 = 40,
    B3 = 41,
    C3 = 42,
    D3 = 43,
    E3 = 44,
    F3 = 45,
    G3 = 46,
    H3 = 47,
    A2 = 48,
    B2 = 49,
    C2 = 50,
    D2 = 51,
    E2 = 52,
    F2 = 53,
    G2 = 54,
    H2 = 55,
    A1 = 56,
    B1 = 57,
    C1 = 58,
    D1 = 59,
    E1 = 60,
    F1 = 61,
    G1 = 62,
    H1 = 63,
    None = 64,
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

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Self::None {
            f.pad("No Square")
        }
        else {
            f.pad(&format!("{}{}", self.file(), self.rank()))
        }
    }
}
