use core::fmt;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&(8 - *self as u8).to_string())
    }
}
