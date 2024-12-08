use core::fmt;
use std::{mem::transmute, ops::{Index, IndexMut}};

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1
}

impl Color {
    #[inline(always)]
    pub fn switch(&mut self) {
        *self = self.opposite();
        debug_assert!(*self == Color::White || *self == Color::Black);
    }

    #[inline(always)]
    pub fn opposite(self) -> Color {
        debug_assert!(self == Color::White || self == Color::Black);
        Color::from(self as u8 ^ 0b1)
    }
}

impl<T, const N: usize> Index<Color> for [T; N] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T, const N: usize> IndexMut<Color> for [T; N] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl From<u8> for Color {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Color::White => "White",
            Color::Black => "Black",
        })
    }
}
