use crate::{piece::PieceType, square::Square};
use core::fmt;
use std::mem::transmute;


const SOURCE_MASK: u32 =  0b0000_0000_0000_0000_0011_1111;
const TARGET_MASK: u32 =  0b0000_0000_0000_1111_1100_0000;
const PIECE_MASK: u32 =   0b0000_0000_1111_0000_0000_0000;
const CAPTURE_MASK: u32 = 0b0000_1111_0000_0000_0000_0000;
const FLAG_MASK: u32 =    0b1111_0000_0000_0000_0000_0000;

#[derive(Clone, Copy)]
pub struct BitMove(u32);

#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum MoveFlag {
    Null,
    WEnPassant,
    BEnPassant,
    WDoublePawn,
    BDoublePawn,
    WKCastle,
    WQCastle,
    BKCastle,
    BQCastle,
    PromoN,
    PromoB,
    PromoR,
    PromoQ,
}

impl fmt::Display for MoveFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            MoveFlag::Null => "Null",
            MoveFlag::WDoublePawn => "White Double Pawn Push",
            MoveFlag::BDoublePawn => "Black Double Pawn Push",
            MoveFlag::WEnPassant => "White En-passant",
            MoveFlag::BEnPassant => "Black En-passant",
            MoveFlag::WKCastle => "White King Castle",
            MoveFlag::WQCastle => "White Queen Castle",
            MoveFlag::BKCastle => "Black King Castle",
            MoveFlag::BQCastle => "Black Queen Castle",
            MoveFlag::PromoN => "Promotion to Knight",
            MoveFlag::PromoB => "Promotion to Bishop",
            MoveFlag::PromoR => "Promotion to Rook",
            MoveFlag::PromoQ => "Promotion to Queen",
        };
        write!(f, "{}", name)
    }
}

impl BitMove {
    pub const NULL: BitMove = BitMove(0);

    #[inline(always)]
    pub fn source(&self) -> Square {
        unsafe { transmute::<u8, Square>((self.0 & SOURCE_MASK) as u8) }
    }

    #[inline(always)]
    pub fn target(&self) -> Square {
        unsafe { transmute::<u8, Square>(((self.0 & TARGET_MASK) >> 6) as u8) }
    }

    #[inline(always)]
    pub fn piece(&self) -> PieceType {
        unsafe { transmute::<u8, PieceType>(((self.0 & PIECE_MASK) >> 12) as u8) }
    }

    #[inline(always)]
    pub fn capture(&self) -> PieceType {
        unsafe { transmute::<u8, PieceType>(((self.0 & CAPTURE_MASK) >> 16) as u8) }
    }

    #[inline(always)]
    pub fn flag(&self) -> MoveFlag {
        unsafe { transmute::<u8, MoveFlag>(((self.0 & FLAG_MASK) >> 20) as u8) }
    }

    #[inline(always)]
    pub fn encode(source: Square, target: Square, piece: PieceType, capture: PieceType, flag: MoveFlag) -> BitMove {
        BitMove(source as u32 | (target as u32) << 6 | (piece as u32) << 12 | (capture as u32) << 16 | (flag as u32) << 20)
    }

    #[inline(always)]
    pub fn decode(&self) -> (Square, Square, PieceType, PieceType, MoveFlag) {
        (self.source(), self.target(), self.piece(), self.capture(), self.flag())
    }
}

impl fmt::Display for BitMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
  Raw move data: {:b}
  Source Square: {}
  Target Square: {}
  Piece Type:    {}
  Capture:       {}
  Move Flag:     {}
        ", self.0, self.source(), self.target(), self.piece(), self.capture(), self.flag())
    }
}
