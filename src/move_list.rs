use crate::{bit_move::{BitMove, Move, ScoringMove}, consts::MAX_MOVES};
use core::fmt;
use std::{mem, ops::{Index, IndexMut}};

pub struct MoveList<T> {
    array: [T; MAX_MOVES],
    size: usize,
}

impl<T: Move> MoveList<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, mv: &T) -> bool {
        self.array.contains(mv)
    }

    #[inline(always)]
    pub fn add(&mut self, mv: T) {
        debug_assert!(self.size < MAX_MOVES);

        unsafe {
            let end = self.array.get_unchecked_mut(self.size);
            *end = mv;
            self.size += 1;
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.array[..self.size].iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.array[..self.size].iter_mut()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn first(&self) -> T {
        self.array[0]
    }
}

impl<T: Move> Default for MoveList<T> {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl MoveList<ScoringMove> {
    #[inline(always)]
    pub fn sort_by_score(&mut self) {
        self.array[..self.size].sort_by(|a, b| b.cmp(a));
    }
}

pub struct MoveListIntoIter<T> {
    move_list: MoveList<T>,
    idx: usize,
}

impl<T: Move> Iterator for MoveListIntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.move_list.size {
            None
        } else {
            unsafe {
                let m = *self.move_list.array.get_unchecked(self.idx);
                self.idx += 1;
                Some(m)
            }
        }
    }
}

impl<T: Move> IntoIterator for MoveList<T> {
    type Item = T;

    type IntoIter = MoveListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        MoveListIntoIter {
            move_list: self,
            idx: 0,
        }
    }
}

impl<'a, T: Move + Sync + 'a> rayon::iter::IntoParallelRefIterator<'a> for MoveList<T> {
    type Item = &'a T;
    type Iter = rayon::slice::Iter<'a, T>;

    fn par_iter(&'a self) -> Self::Iter {
        self.array[..self.size].par_iter()
    }
}

impl<T> Index<usize> for MoveList<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &T {
        &self.array[index]
    }
}

impl<T> IndexMut<usize> for MoveList<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.array[index]
    }
}

impl fmt::Display for MoveList<BitMove> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("
    Printing move data for {} moves:
  |-----------------------------------------------------------------|
  | Source   | Target   | Piece    | Capture  | Flag                |
  |-----------------------------------------------------------------|\n", self.size);

        for i in 0..self.size {
            s += &self[i].to_row_string();
        }

        s += "  |-----------------------------------------------------------------|";

        f.pad(&s)
    }
}

impl fmt::Display for MoveList<ScoringMove> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("
    Printing move data for {} moves:
  |-----------------------------------------------------------------|
  | BitMove | Score                                                 |
  |-----------------------------------------------------------------|\n", self.size);

        for i in 0..self.size {
            s += &self[i].to_row_string();
        }

        s += "  |-----------------------------------------------------------------|";

        f.pad(&s)
    }
}

#[cfg(test)]
mod tests {

    use crate::{bit_move::ScoringMove, score::Score};

    use super::*;

    #[test]
    fn move_list_of_scoring_moves_finds_max() {
        let mut move_list = MoveList::<ScoringMove>::new();

        move_list.add(ScoringMove::blank(Score::from(-2)));
        move_list.add(ScoringMove::blank(Score::from(-1)));
        move_list.add(ScoringMove::blank(Score::from(0)));
        move_list.add(ScoringMove::blank(Score::from(1)));
        move_list.add(ScoringMove::blank(Score::from(2)));

        assert_eq!(move_list.iter().max().unwrap().score, Score::from(2));
        assert_eq!(move_list.iter().min().unwrap().score, Score::from(-2));
    }
}
