use core::fmt;
use std::mem::transmute;

use thiserror::Error;

use crate::consts::RANK_COUNT;

// NOTE: The rank enum can be unintuitive to work with since it starts with the eighth rank.
// Changing the ordering could impact all places where Rank is used!
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Rank {
    R1 = 7,
    R2 = 6,
    R3 = 5,
    R4 = 4,
    R5 = 3,
    R6 = 2,
    R7 = 1,
    R8 = 0,
}

impl From<u8> for Rank {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

#[derive(Error, Debug)]
#[error("Couldn't parse rank: {0}")]
pub struct RankParseError(char);

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
            _ => Err(RankParseError(ch)),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&(RANK_COUNT as u8 - *self as u8).to_string())
    }
}
