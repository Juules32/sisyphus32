use std::{fmt::Display, ops::{Add, Neg, AddAssign, Sub, SubAssign}};

use crate::consts::MAX_DEPTH;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Score(i16);

impl Score {
    pub const CHECKMATE: Score = Score(10000);
    pub const DRAW: Score = Score(0);
    pub const ZERO: Score = Score(0);
    pub const BLANK: Score = Score(0);
    pub const STALEMATE: Score = Score(0);
    pub const REPETITION: Score = Score(0);
    pub const START_ALPHA: Score = Score(-32001);
    pub const START_BETA: Score = Score(32001);
    
    #[inline(always)]
    pub fn abs(self) -> Score {
        Score::from(self.0.abs())
    }

    #[inline(always)]
    pub fn signum(self) -> Score {
        Score::from(self.0.signum())
    }

    #[inline(always)]
    pub fn checkmate_minus_depth(depth: usize) -> Score {
        Score::from(Self::CHECKMATE.0 - depth as i16)
    }
    
    #[inline(always)]
    pub fn is_checkmate(self) -> bool {
        self.abs() >= Score::checkmate_minus_depth(MAX_DEPTH)
    }

    #[inline(always)]
    pub fn is_positive(self) -> bool {
        self.0 > 0
    }

    #[inline(always)]
    pub fn is_negative(self) -> bool {
        self.0 < 0
    }
}

impl From<i16> for Score {
    fn from(value: i16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Score> for f32 {
    fn from(score: Score) -> Self {
        score.0 as f32
    }
}

impl From<Score> for i16 {
    fn from(score: Score) -> Self {
        score.0
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from(-self.0)
    }
}

impl Add for Score {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Score::from(self.0 + other.0)
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Score {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Score::from(self.0 - other.0)
    }
}

impl SubAssign for Score {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl Add<i16> for Score {
    type Output = Self;
    fn add(self, rhs: i16) -> Self::Output {
        Score::from(self.0 + rhs)
    }
}

impl AddAssign<i16> for Score {
    fn add_assign(&mut self, other: i16) {
        self.0 += other;
    }
}

impl Sub<i16> for Score {
    type Output = Self;
    fn sub(self, rhs: i16) -> Self::Output {
        Score::from(self.0 - rhs)
    }
}

impl SubAssign<i16> for Score {
    fn sub_assign(&mut self, other: i16) {
        self.0 -= other;
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
