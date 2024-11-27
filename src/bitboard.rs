use core::fmt;
use std::ops::Shl;

use crate::square::Square;

pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline(always)]
    pub fn set_sq(&mut self, sq: u8) {
        self.0 |= 1 << sq;
    }

    #[inline(always)]
    pub fn pop_sq(&mut self, sq: u8) {
        self.0 &= !(1 << sq);
    }

    #[inline(always)]
    pub fn is_set_sq(self, sq: u8) -> bool {
        self.0 & 1 << sq != 0
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn is_not_empty(self) -> bool {
        self.0 != 0
    }

    pub fn print(&self) {
        println!("{}", &self);
    }
}

#[allow(dead_code)]
impl Bitboard {
    pub const FILE_A: Bitboard = Bitboard(0xff);
    pub const FILE_B: Bitboard = Bitboard(0xff00);
    pub const FILE_C: Bitboard = Bitboard(0xff0000);
    pub const FILE_D: Bitboard = Bitboard(0xff000000);
    pub const FILE_E: Bitboard = Bitboard(0xff00000000);
    pub const FILE_F: Bitboard = Bitboard(0xff0000000000);
    pub const FILE_G: Bitboard = Bitboard(0xff000000000000);
    pub const FILE_H: Bitboard = Bitboard(0xff00000000000000);

    pub const RANK_1: Bitboard = Bitboard(0x101010101010101);
    pub const RANK_2: Bitboard = Bitboard(0x202020202020202);
    pub const RANK_3: Bitboard = Bitboard(0x404040404040404);
    pub const RANK_4: Bitboard = Bitboard(0x808080808080808);
    pub const RANK_5: Bitboard = Bitboard(0x1010101010101010);
    pub const RANK_6: Bitboard = Bitboard(0x2020202020202020);
    pub const RANK_7: Bitboard = Bitboard(0x4040404040404040);
    pub const RANK_8: Bitboard = Bitboard(0x8080808080808080);

    pub const WHITE_SQUARES: Bitboard = Bitboard(0x55AA55AA55AA55AA);
    pub const BLACK_SQUARES: Bitboard = Bitboard(0xAA55AA55AA55AA55);
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for i in 0..8 {
            s += &format!("  {}  ", 8 - i);
            for j in 0..8 {
                let square = (self.0 >> (i * 8 + j)) & 1;
                s += if square != 0 {"O "} else {". "};
            }
            s += "\n";
        }

        s += "\n    a b c d e f g h\n";

        s += &format!("\nBitboard: {}\n", self.0);

        f.pad(&s)
    }
}

impl Shl<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn shl(self, rhs: Square) -> Bitboard {
        Bitboard((self.0).wrapping_shl(rhs.0 as u32))
    }
}
