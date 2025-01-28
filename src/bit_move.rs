use crate::{move_flag::MoveFlag, piece::PieceType, square::Square};
use core::fmt;
use std::mem::transmute;

const SOURCE_MASK: u32 =  0b0000_0000_0000_0000_0000_0000_0011_1111;
const TARGET_MASK: u32 =  0b0000_0000_0000_0000_0000_1111_1100_0000;
const PIECE_MASK: u32 =   0b0000_0000_0000_0000_1111_0000_0000_0000;
const CAPTURE_MASK: u32 = 0b0000_0000_0000_1111_0000_0000_0000_0000;
const FLAG_MASK: u32 =    0b0000_0000_1111_0000_0000_0000_0000_0000;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitMove(u32);

impl BitMove {
    pub const EMPTY: BitMove = BitMove(0);

    #[inline(always)]
    fn source(&self) -> Square {
        Square::from((self.0 & SOURCE_MASK) as u8)
    }

    #[inline(always)]
    fn target(&self) -> Square {
        Square::from(((self.0 & TARGET_MASK) >> 6) as u8)
    }

    #[inline(always)]
    fn piece(&self) -> PieceType {
        PieceType::from(((self.0 & PIECE_MASK) >> 12) as u8)
    }

    #[inline(always)]
    fn capture(&self) -> PieceType {
        PieceType::from(((self.0 & CAPTURE_MASK) >> 16) as u8)
    }

    #[inline(always)]
    fn flag(&self) -> MoveFlag {
        MoveFlag::from(((self.0 & FLAG_MASK) >> 20) as u8)
    }

    #[inline(always)]
    pub fn encode(
        source: Square, 
        target: Square, 
        piece: PieceType, 
        capture: PieceType, 
        flag: MoveFlag
    ) -> BitMove {
        BitMove(
            source as u32 | 
            (target as u32) << 6 | 
            (piece as u32) << 12 | 
            (capture as u32) << 16 | 
            (flag as u32) << 20
        )
    }

    #[inline(always)]
    pub fn decode(&self) -> (Square, Square, PieceType, PieceType, MoveFlag) {
        (self.source(), self.target(), self.piece(), self.capture(), self.flag())
    }

    pub fn to_row_string(self) -> String {
        format!(
            "  | {:<8} | {:<8} | {:<8} | {:<8} | {:<19} |\n",
            self.source(),
            self.target(),
            self.piece(),
            self.capture(),
            self.flag()
        )
    }

    pub fn to_uci_string(self) -> String {
        format!(
            "{}{}{}",
            self.source(),
            self.target(),
            match self.flag() {
                MoveFlag::PromoN => "n",
                MoveFlag::PromoB => "b",
                MoveFlag::PromoR => "r",
                MoveFlag::PromoQ => "q",
                _ => "",
            }
        )
    }
}

impl fmt::Display for BitMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!(
            "
  Raw move data: {:b}
  Source Square: {}
  Target Square: {}
  Piece Type:    {}
  Capture:       {}
  Move Flag:     {}\n",
            self.0,
            self.source(),
            self.target(),
            self.piece(),
            self.capture(),
            self.flag()
        ))
    }
}
