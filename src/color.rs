use core::fmt;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color(u8);

impl Color {
    pub const WHITE: Color = Color(0b0);
    pub const BLACK: Color = Color(0b1);

    #[inline(always)]
    pub fn switch(&mut self) {
        self.0 ^= 0b1;
        debug_assert!(*self == Color::WHITE || *self == Color::BLACK);
    }

    #[inline(always)]
    pub fn opposite(&self) -> Color {
        debug_assert!(*self == Color::WHITE || *self == Color::BLACK);
        Color(self.0 ^ 0b1)
    }
}

impl<T, const N: usize> Index<Color> for [T; N] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl<T, const N: usize> IndexMut<Color> for [T; N] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Color::WHITE => "White",
            Color::BLACK => "Black",
            _ => panic!("Unknown side value!")
        })
    }
}
