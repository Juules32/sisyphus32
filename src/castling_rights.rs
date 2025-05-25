use crate::{consts::SQUARE_COUNT, Square};
use core::fmt;

// Castling right update constants
const INDEX_2_CASTLING_RIGHTS: [u8; SQUARE_COUNT] = [
    0b0111, 0b1111, 0b1111, 0b1111, 0b0011, 0b1111, 0b1111, 0b1011,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1101, 0b1111, 0b1111, 0b1111, 0b1100, 0b1111, 0b1111, 0b1110
];

#[derive(Clone, Copy, PartialEq)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub const DEFAULT: CastlingRights = CastlingRights(0b1111);
    pub const NONE: CastlingRights = CastlingRights(0b0000);

    pub const WK: CastlingRights = CastlingRights(0b0001);
    pub const WQ: CastlingRights = CastlingRights(0b0010);
    pub const BK: CastlingRights = CastlingRights(0b0100);
    pub const BQ: CastlingRights = CastlingRights(0b1000);

    #[inline(always)]
    pub fn update(&mut self, source: Square, target: Square) {
        self.0 &= INDEX_2_CASTLING_RIGHTS[source] & INDEX_2_CASTLING_RIGHTS[target];
    }

    #[inline(always)]
    pub fn wk(&self) -> bool {
        self.0 & CastlingRights::WK.0 != 0
    }

    #[inline(always)]
    pub fn wq(&self) -> bool {
        self.0 & CastlingRights::WQ.0 != 0
    }

    #[inline(always)]
    pub fn bk(&self) -> bool {
        self.0 & CastlingRights::BK.0 != 0
    }

    #[inline(always)]
    pub fn bq(&self) -> bool {
        self.0 & CastlingRights::BQ.0 != 0
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CastlingRights::NONE => f.pad("-"),
            _ => {
                let wk = if self.wk() { "K" } else { "" };
                let wq = if self.wq() { "Q" } else { "" };
                let bk = if self.bk() { "k" } else { "" };
                let bq = if self.bq() { "q" } else { "" };
                f.pad(&format!("{wk}{wq}{bk}{bq}"))
            }
        }
    }
}
