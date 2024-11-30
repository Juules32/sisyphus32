use crate::bitboard::Bitboard;
use crate::file::File;
use crate::rank::Rank;
use core::fmt;
use std::mem::transmute;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq)]
pub struct Square(pub u8);

impl Square {
    #[inline(always)]
    pub fn to_bb(self) -> Bitboard {
        Bitboard(1 << self.0)
    }

    #[inline(always)]
    pub fn rank(self) -> Rank {
        unsafe { transmute::<u8, Rank>(self.0 >> 3 & 0b0000_0111) }
    }

    #[inline(always)]
    pub fn file(self) -> File {
        unsafe { transmute::<u8, File>(self.0 & 0b0000_0111) }
    }

    #[inline(always)]
    pub fn above(self) -> Square {
        Square(self.0 - 8)
    }
    
    #[inline(always)]
    pub fn below(self) -> Square {
        Square(self.0 + 8)
    }

    #[inline(always)]
    pub fn left(self) -> Square {
        Square(self.0 - 1)
    }

    #[inline(always)]
    pub fn right(self) -> Square {
        Square(self.0 + 1)
    }
}

#[allow(dead_code)]
impl Square {
    pub const A8: Square = Square(0);
    pub const B8: Square = Square(1);
    pub const C8: Square = Square(2);
    pub const D8: Square = Square(3);
    pub const E8: Square = Square(4);
    pub const F8: Square = Square(5);
    pub const G8: Square = Square(6);
    pub const H8: Square = Square(7);
    pub const A7: Square = Square(8);
    pub const B7: Square = Square(9);
    pub const C7: Square = Square(10);
    pub const D7: Square = Square(11);
    pub const E7: Square = Square(12);
    pub const F7: Square = Square(13);
    pub const G7: Square = Square(14);
    pub const H7: Square = Square(15);
    pub const A6: Square = Square(16);
    pub const B6: Square = Square(17);
    pub const C6: Square = Square(18);
    pub const D6: Square = Square(19);
    pub const E6: Square = Square(20);
    pub const F6: Square = Square(21);
    pub const G6: Square = Square(22);
    pub const H6: Square = Square(23);
    pub const A5: Square = Square(24);
    pub const B5: Square = Square(25);
    pub const C5: Square = Square(26);
    pub const D5: Square = Square(27);
    pub const E5: Square = Square(28);
    pub const F5: Square = Square(29);
    pub const G5: Square = Square(30);
    pub const H5: Square = Square(31);
    pub const A4: Square = Square(32);
    pub const B4: Square = Square(33);
    pub const C4: Square = Square(34);
    pub const D4: Square = Square(35);
    pub const E4: Square = Square(36);
    pub const F4: Square = Square(37);
    pub const G4: Square = Square(38);
    pub const H4: Square = Square(39);
    pub const A3: Square = Square(40);
    pub const B3: Square = Square(41);
    pub const C3: Square = Square(42);
    pub const D3: Square = Square(43);
    pub const E3: Square = Square(44);
    pub const F3: Square = Square(45);
    pub const G3: Square = Square(46);
    pub const H3: Square = Square(47);
    pub const A2: Square = Square(48);
    pub const B2: Square = Square(49);
    pub const C2: Square = Square(50);
    pub const D2: Square = Square(51);
    pub const E2: Square = Square(52);
    pub const F2: Square = Square(53);
    pub const G2: Square = Square(54);
    pub const H2: Square = Square(55);
    pub const A1: Square = Square(56);
    pub const B1: Square = Square(57);
    pub const C1: Square = Square(58);
    pub const D1: Square = Square(59);
    pub const E1: Square = Square(60);
    pub const F1: Square = Square(61);
    pub const G1: Square = Square(62);
    pub const H1: Square = Square(63);
    pub const NO_SQ: Square = Square(64);

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
        &self[index.0 as usize]
    }
}

impl<T, const N: usize> IndexMut<Square> for [T; N] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Square::NO_SQ {
            f.pad("No Square")
        }
        else {
            write!(f, "{}{}", self.file(), self.rank())
        }
    }
}
