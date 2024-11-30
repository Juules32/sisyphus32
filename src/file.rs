use core::fmt;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
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

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_char = ('a' as u8 + *self as u8) as char;
        f.pad(&f_char.to_string())
    }
}
