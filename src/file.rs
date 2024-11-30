use core::fmt;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum File {
    FH = 0,
    FG = 1,
    FF = 2,
    FE = 3,
    FD = 4,
    FC = 5,
    FB = 6,
    FA = 7,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_char = ('a' as u8 + *self as u8) as char;
        f.pad(&f_char.to_string())
    }
}
