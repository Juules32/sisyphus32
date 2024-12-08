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

impl From<char> for Rank {
    #[inline(always)]
    fn from(ch: char) -> Self {
        match ch {
            '1' => Self::R1,
            '2' => Self::R2,
            '3' => Self::R3,
            '4' => Self::R4,
            '5' => Self::R5,
            '6' => Self::R6,
            '7' => Self::R7,
            '8' => Self::R8,
            _ => panic!("Illegal rank char!"),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&(8 - *self as u8).to_string())
    }
}
