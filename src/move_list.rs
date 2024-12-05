use crate::bit_move::BitMove;
use core::fmt;
use std::ops::{Index, IndexMut};

pub const MAX_MOVES: usize = 255;

pub struct MoveList {
    array: [BitMove; MAX_MOVES],
    size: usize
}

impl Default for MoveList {
    #[inline]
    fn default() -> Self {
        MoveList {
            array: [BitMove::EMPTY; MAX_MOVES],
            size: 0,
        }
    }
}

impl MoveList {
    #[inline(always)]
    pub fn add(&mut self, mv: BitMove) {
        debug_assert!(self.size < MAX_MOVES);
        
        unsafe {
            let end = self.array.get_unchecked_mut(self.size);
            *end = mv;
            self.size += 1;
        }
    }
}

impl Index<usize> for MoveList {
    type Output = BitMove;

    #[inline(always)]
    fn index(&self, index: usize) -> &BitMove {
        &self.array[index]
    }
}

impl IndexMut<usize> for MoveList {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut BitMove {
        &mut self.array[index]
    }
}

impl fmt::Display for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("
  |----------------------------------------------------------------|
  | Printing move data for {:<3} moves                               |
  | Source   | Target   | Piece    | Capture  | Flag               |
  |----------------------------------------------------------------|\n", self.size);

        for i in 0..self.size {
            s += &self[i].to_row_string();
        }

        s += &format!("  |----------------------------------------------------------------|");

        f.pad(&s)
    }
}
