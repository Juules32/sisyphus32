use core::fmt;
use std::mem::transmute;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Rank {
    R8 = 0,
    R7 = 1,
    R6 = 2,
    R5 = 3,
    R4 = 4,
    R3 = 5,
    R2 = 6,
    R1 = 7,
}

impl From<u8> for Rank {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&(8 - *self as u8).to_string())
    }
}
