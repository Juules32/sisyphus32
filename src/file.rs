use core::fmt;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code)]
pub enum File {
    F8 = 0,
    F7 = 1,
    F6 = 2,
    F5 = 3,
    F4 = 4,
    F3 = 5,
    F2 = 6,
    F1 = 7,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_char = ('a' as u8 + self.value()) as char;
        f.pad(&f_char.to_string())
    }
}

impl File {
    pub fn value(self) -> u8 {
        self as u8
    }
}
