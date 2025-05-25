use core::fmt;
use std::mem::transmute;

use crate::FileParseError;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum File {
    FA = 0,
    FB = 1,
    FC = 2,
    FD = 3,
    FE = 4,
    FF = 5,
    FG = 6,
    FH = 7,
}

impl From<u8> for File {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

impl TryFrom<char> for File {
    type Error = FileParseError;

    #[inline(always)]
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'a' => Ok(Self::FA),
            'b' => Ok(Self::FB),
            'c' => Ok(Self::FC),
            'd' => Ok(Self::FD),
            'e' => Ok(Self::FE),
            'f' => Ok(Self::FF),
            'g' => Ok(Self::FG),
            'h' => Ok(Self::FH),
            _ => Err(FileParseError(ch)),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_char = (b'a' + *self as u8) as char;
        f.pad(&f_char.to_string())
    }
}
