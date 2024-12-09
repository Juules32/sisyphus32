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

#[derive(Debug)]
pub struct RankParseError(pub &'static str);

impl TryFrom<char> for Rank {
    type Error = RankParseError;

    #[inline(always)]
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '1' => Ok(Self::R1),
            '2' => Ok(Self::R2),
            '3' => Ok(Self::R3),
            '4' => Ok(Self::R4),
            '5' => Ok(Self::R5),
            '6' => Ok(Self::R6),
            '7' => Ok(Self::R7),
            '8' => Ok(Self::R8),
            _ => Err(RankParseError("Illegal rank char!")),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&(8 - *self as u8).to_string())
    }
}
