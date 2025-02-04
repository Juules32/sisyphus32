use crate::{bit_twiddles, square::Square};
use core::fmt;
use std::{mem::transmute, ops::*};

#[derive(Clone, Copy, PartialEq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline(always)]
    pub fn set_sq(&mut self, sq: Square) {
        self.0 |= 1 << sq as u8;
    }

    #[inline(always)]
    pub fn pop_sq(&mut self, sq: Square) {
        self.0 &= !(1 << sq as u8);
    }

    #[inline(always)]
    pub fn is_set_sq(&self, sq: Square) -> bool {
        self.0 & (1 << sq as u8) != 0
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub fn shift_upwards(&self, amount: usize) -> Bitboard {
        Bitboard(self.0 >> amount)
    }

    #[inline(always)]
    pub fn shift_downwards(&self, amount: usize) -> Bitboard {
        Bitboard(self.0 << amount)
    }

    #[inline(always)]
    pub fn count_bits(self) -> u8 {
        bit_twiddles::count_bits(self.0)
    }

    #[inline(always)]
    pub fn get_lsb(self) -> Square {
        debug_assert_ne!(self.count_bits(), 0);
        Square::from(bit_twiddles::get_lsb(self.0))
    }

    #[inline(always)]
    pub fn pop_lsb(&mut self) -> Square {
        let lsb = self.get_lsb();
        self.pop_sq(lsb);
        lsb
    }

    #[inline(always)]
    pub fn to_sq(self) -> Square {
        debug_assert_eq!(self.count_bits(), 1);
        self.get_lsb()
    }
}

impl Bitboard {
    pub const FILE_A: Bitboard = Bitboard(0x101010101010101);
    pub const FILE_B: Bitboard = Bitboard(0x202020202020202);
    pub const FILE_C: Bitboard = Bitboard(0x404040404040404);
    pub const FILE_D: Bitboard = Bitboard(0x808080808080808);
    pub const FILE_E: Bitboard = Bitboard(0x1010101010101010);
    pub const FILE_F: Bitboard = Bitboard(0x2020202020202020);
    pub const FILE_G: Bitboard = Bitboard(0x4040404040404040);
    pub const FILE_H: Bitboard = Bitboard(0x8080808080808080);

    pub const RANK_8: Bitboard = Bitboard(0xFF);
    pub const RANK_7: Bitboard = Bitboard(0xFF00);
    pub const RANK_6: Bitboard = Bitboard(0xFF0000);
    pub const RANK_5: Bitboard = Bitboard(0xFF000000);
    pub const RANK_4: Bitboard = Bitboard(0xFF00000000);
    pub const RANK_3: Bitboard = Bitboard(0xFF0000000000);
    pub const RANK_2: Bitboard = Bitboard(0xFF000000000000);
    pub const RANK_1: Bitboard = Bitboard(0xFF00000000000000);

    pub const NOT_A: Bitboard = Bitboard(0xFEFEFEFEFEFEFEFE);
    pub const NOT_AB: Bitboard = Bitboard(0xFCFCFCFCFCFCFCFC);
    pub const NOT_H: Bitboard = Bitboard(0x7F7F7F7F7F7F7F7F);
    pub const NOT_GH: Bitboard = Bitboard(0x3F3F3F3F3F3F3F3F);

    pub const WHITE_SQUARES: Bitboard = Bitboard(0xAA55AA55AA55AA55);
    pub const BLACK_SQUARES: Bitboard = Bitboard(0x55AA55AA55AA55AA);

    pub const WHITE_STARTING_PIECES: Bitboard = Bitboard(0xFFFF000000000000);
    pub const BLACK_STARTING_PIECES: Bitboard = Bitboard(0xFFFF);
    pub const ALL_STARTING_PIECES: Bitboard = Bitboard(0xFFFF00000000FFFF);
    
    pub const EDGES: Bitboard = Bitboard(0xFF818181818181FF);
    pub const EMPTY: Bitboard = Bitboard(0x0);

    pub const W_KING_SIDE_MASK: Bitboard = Bitboard(0x6000000000000000);
    pub const W_QUEEN_SIDE_MASK: Bitboard = Bitboard(0xE00000000000000);
    pub const B_KING_SIDE_MASK: Bitboard = Bitboard(0x60);
    pub const B_QUEEN_SIDE_MASK: Bitboard = Bitboard(0xE);

    pub const BP: Bitboard = Bitboard::RANK_7;
    pub const BN: Bitboard = Bitboard(0x42);
    pub const BB: Bitboard = Bitboard(0x24);
    pub const BR: Bitboard = Bitboard(0x81);
    pub const BQ: Bitboard = Bitboard(0x8);
    pub const BK: Bitboard = Bitboard(0x10);

    pub const WP: Bitboard = Bitboard::RANK_2;
    pub const WN: Bitboard = Bitboard(0x4200000000000000);
    pub const WB: Bitboard = Bitboard(0x2400000000000000);
    pub const WR: Bitboard = Bitboard(0x8100000000000000);
    pub const WQ: Bitboard = Bitboard(0x800000000000000);
    pub const WK: Bitboard = Bitboard(0x1000000000000000);
}

macro_rules! impl_bb_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Bitboard> for Bitboard {
            type Output = Bitboard;

            #[inline(always)]
            fn $method(self, rhs: Bitboard) -> Bitboard {
                Bitboard(self.0 $op rhs.0 as u64)
            }
        }
    };
}

macro_rules! impl_bb_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<Bitboard> for Bitboard {

            #[inline(always)]
            fn $method(&mut self, rhs: Self) {
                self.0 $op rhs.0
            }
        }
    };
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl_bb_op!(BitAnd, bitand, &);
impl_bb_op!(BitOr, bitor, |);
impl_bb_op!(BitXor, bitxor, ^);

impl_bb_assign!(BitAndAssign, bitand_assign, &=);
impl_bb_assign!(BitOrAssign, bitor_assign, |=);
impl_bb_assign!(BitXorAssign, bitxor_assign, ^=);

impl From<u64> for Bitboard {
    #[inline(always)]
    fn from(number: u64) -> Self {
        unsafe { transmute::<u64, Self>(number) }
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");
        for i in 0..8 {
            s += &format!("  {}  ", 8 - i);
            for j in 0..8 {
                let square = (self.0 >> (i * 8 + j)) & 1;
                s += if square != 0 { "O " } else { ". " };
            }
            s += "\n";
        }
        s += "\n     a b c d e f g h\n";
        s += &format!("\nBitboard: {}\n", self.0);
        f.pad(&s)
    }
}
