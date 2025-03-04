use crate::{eval_move::EvalMove, move_flag::MoveFlag, piece::Piece, position::Position, square::Square};
use core::fmt;
use std::{cmp::Ordering, fmt::Display, hash::Hash, mem};

#[cfg(feature = "unit_bb")]
const SOURCE_MASK: u32 =  0b0000_0000_0000_0000_0000_0000_0011_1111;
#[cfg(feature = "unit_bb")]
const TARGET_MASK: u32 =  0b0000_0000_0000_0000_0000_1111_1100_0000;
#[cfg(feature = "unit_bb")]
const PIECE_MASK: u32 =   0b0000_0000_0000_0000_1111_0000_0000_0000;
#[cfg(feature = "unit_bb")]
const CAPTURE_MASK: u32 = 0b0000_0000_0000_1111_0000_0000_0000_0000;
#[cfg(feature = "unit_bb")]
const FLAG_MASK: u32 =    0b0000_0000_1111_0000_0000_0000_0000_0000;

#[cfg(feature = "unit_bb_array")]
const SOURCE_MASK: u16 =  0b0000_0000_0011_1111;
#[cfg(feature = "unit_bb_array")]
const TARGET_MASK: u16 =  0b0000_1111_1100_0000;
#[cfg(feature = "unit_bb_array")]
const FLAG_MASK: u16 =    0b1111_0000_0000_0000;

pub trait Move: Copy + Default + Eq + Hash {
    fn get_bit_move(self) -> BitMove;
    fn new(position: &Position, bit_move: BitMove) -> Self;
}

/*------------------------------*\ 
             BitMove
\*------------------------------*/
#[cfg(feature = "unit_bb")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BitMove(u32);

// NOTE: Maintaining an array of piece positions allows moves to use only two bytes
#[cfg(feature = "unit_bb_array")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BitMove(u16);

impl Move for BitMove {
    #[inline(always)]
    fn get_bit_move(self) -> BitMove {
        self
    }
    
    fn new(_position: &Position, bit_move: BitMove) -> Self {
        bit_move
    }
}

impl BitMove {
    pub const EMPTY: BitMove = unsafe { mem::zeroed() };

    #[inline(always)]
    pub fn source(&self) -> Square {
        Square::from((self.0 & SOURCE_MASK) as u8)
    }

    #[inline(always)]
    pub fn target(&self) -> Square {
        Square::from(((self.0 & TARGET_MASK) >> 6) as u8)
    }

    #[cfg(feature = "unit_bb")]
    #[inline(always)]
    pub fn piece(&self) -> Piece {
        Piece::from(((self.0 & PIECE_MASK) >> 12) as u8)
    }

    #[cfg(feature = "unit_bb")]
    #[inline(always)]
    pub fn capture(&self) -> Piece {
        Piece::from(((self.0 & CAPTURE_MASK) >> 16) as u8)
    }

    #[cfg(feature = "unit_bb")]
    #[inline(always)]
    pub fn flag(&self) -> Option<MoveFlag> {
        unsafe { std::mem::transmute::<u8, Option<MoveFlag>>(((self.0 & FLAG_MASK) >> 20) as u8) }
    }

    // NOTE: For the array representation, the flag mask is offset by 12 instead of 20
    #[cfg(feature = "unit_bb_array")]
    #[inline(always)]
    pub fn flag(&self) -> Option<MoveFlag> {
        unsafe { std::mem::transmute::<u8, Option<MoveFlag>>(((self.0 & FLAG_MASK) >> 12) as u8) }
    }

    #[cfg(feature = "unit_bb")]
    #[inline(always)]
    pub fn encode(
        source: Square, 
        target: Square, 
        piece: Piece, 
        capture: Piece, 
        flag: Option<MoveFlag>
    ) -> BitMove {
        unsafe {
            BitMove(
                source as u32 | 
                (target as u32) << 6 | 
                (piece as u32) << 12 | 
                (capture as u32) << 16 | 
                (std::mem::transmute::<Option<MoveFlag>, u8>(flag) as u32) << 20
            )
        }
    }

    #[cfg(feature = "unit_bb_array")]
    #[inline(always)]
    pub fn encode(
        source: Square, 
        target: Square, 
        flag: Option<MoveFlag>
    ) -> BitMove {
        unsafe {
            BitMove(
                source as u16 | 
                (target as u16) << 6 | 
                (std::mem::transmute::<Option<MoveFlag>, u8>(flag) as u16) << 12
            )
        }
    }

    #[cfg(feature = "unit_bb")]
    #[inline(always)]
    pub fn decode(&self) -> (Square, Square, Piece, Piece, Option<MoveFlag>) {
        (self.source(), self.target(), self.piece(), self.capture(), self.flag())
    }

    #[cfg(feature = "unit_bb_array")]
    #[inline(always)]
    pub fn decode(&self) -> (Square, Square, Option<MoveFlag>) {
        (self.source(), self.target(), self.flag())
    }

    #[inline(always)]
    pub fn is_capture_or_promotion(self, position: &Position) -> bool {
        position.get_piece(self.target()) != Piece::None || self.flag().is_some_and(|f| f.is_promotion())
    }

    #[inline(always)]
    pub fn is_pp_capture_or_castle(self, position: &Position) -> bool {
        let source_piece = position.get_piece(self.source());
        let target_piece = position.get_piece(self.target());
        source_piece == Piece::WP ||
        source_piece == Piece::BP ||
        target_piece != Piece::None ||
        self.flag().is_some_and(|f| f.is_castle())
    }

    #[cfg(feature = "unit_bb")]
    pub fn to_row_string(self) -> String {
        format!(
            "  | {:<8} | {:<8} | {:<8} | {:<8} | {:<19?} |\n",
            self.source(),
            self.target(),
            self.piece(),
            self.capture(),
            self.flag()
        )
    }

    #[cfg(feature = "unit_bb_array")]
    pub fn to_row_string(self) -> String {
        format!(
            "  | {:<8} | {:<8} | {:<8} | {:<8} | {:<19?} |\n",
            self.source(),
            self.target(),
            "",
            "",
            self.flag()
        )
    }

    pub fn to_uci_string(self) -> String {
        format!(
            "{}{}{}",
            self.source(),
            self.target(),
            match self.flag() {
                Some(MoveFlag::PromoN) => "n",
                Some(MoveFlag::PromoB) => "b",
                Some(MoveFlag::PromoR) => "r",
                Some(MoveFlag::PromoQ) => "q",
                _ => "",
            }
        )
    }
}

impl Default for BitMove {
    fn default() -> Self {
        Self::EMPTY
    }
}

#[cfg(feature = "unit_bb")]
impl Display for BitMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!(
            "
  Raw move data: {:b}
  Source Square: {}
  Target Square: {}
  Piece Type:    {}
  Capture:       {}
  Move Flag:     {:?}\n",
            self.0,
            self.source(),
            self.target(),
            self.piece(),
            self.capture(),
            self.flag()
        ))
    }
}

#[cfg(feature = "unit_bb_array")]
impl Display for BitMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!(
            "
  Raw move data: {:b}
  Source Square: {}
  Target Square: {}
  Move Flag:     {:?}\n",
            self.0,
            self.source(),
            self.target(),
            self.flag()
        ))
    }
}

/*------------------------------*\ 
           ScoringMove
\*------------------------------*/
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ScoringMove {
    pub bit_move: BitMove,
    pub score: i16,
}

impl Move for ScoringMove {
    #[inline(always)]
    fn get_bit_move(self) -> BitMove {
        self.bit_move
    }
    
    #[inline(always)]
    fn new(position: &Position, bit_move: BitMove) -> Self {
        let score = EvalMove::eval(position, bit_move);
        Self { bit_move, score }
    }
}

impl ScoringMove {
    const EMPTY: ScoringMove = unsafe { mem::zeroed() };

    #[inline(always)]
    pub fn blank(score: i16) -> Self {
        Self {
            bit_move: BitMove::EMPTY,
            score
        }
    }

    #[inline(always)]
    pub fn new(bit_move: BitMove, score: i16) -> Self {
        Self {
            bit_move,
            score
        }
    }
}

impl Default for ScoringMove {
    #[inline(always)]
    fn default() -> Self {
        Self::EMPTY
    }
}

impl From<BitMove> for ScoringMove {
    #[inline(always)]
    fn from(bm: BitMove) -> Self {
        ScoringMove { bit_move: bm, score: 0 }
    }
}

impl Ord for ScoringMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for ScoringMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "unit_bb")]
    fn encode_and_decode_works() {
        let bit_move = BitMove::encode(Square::A1, Square::B1, Piece::WP, Piece::None, None);
        let (source, target, piece, capture, flag) = bit_move.decode();

        assert_eq!(source, Square::A1);
        assert_eq!(target, Square::B1);
        assert_eq!(piece, Piece::WP);
        assert_eq!(capture, Piece::None);
        assert_eq!(flag, None);
    }

    #[test]
    #[cfg(feature = "unit_bb_array")]
    fn encode_and_decode_works() {
        let bit_move = BitMove::encode(Square::A1, Square::B1, None);
        let (source, target, flag) = bit_move.decode();

        assert_eq!(source, Square::A1);
        assert_eq!(target, Square::B1);
        assert_eq!(flag, None);
    }
}
