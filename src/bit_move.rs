use crate::{piece::PieceType, square::Square};
use core::fmt;
use std::mem::transmute;


const SOURCE_MASK: u32 =  0b0000_0000_0000_000000_111111;
const TARGET_MASK: u32 =  0b0000_0000_0000_111111_000000;
const PIECE_MASK: u32 =   0b0000_0000_1111_000000_000000;
const CAPTURE_MASK: u32 = 0b0000_1111_0000_000000_000000;
const FLAG_MASK: u32 =    0b1111_0000_0000_000000_000000;

#[derive(Clone, Copy)]
pub struct BitMove {
    data: u32
}

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

    #[inline(always)]
    pub fn source(&self) -> Square {
        Square((self.data & SOURCE_MASK) as u8)
    }

    #[inline(always)]
    pub fn target(&self) -> Square {
        Square(((self.data & TARGET_MASK) >> 6) as u8)
    }

    #[inline(always)]
    pub fn piece(&self) -> PieceType {
        unsafe { transmute::<u8, PieceType>(((self.data & PIECE_MASK) >> 12) as u8) }
    }

    #[inline(always)]
    pub fn capture(&self) -> PieceType {
        unsafe { transmute::<u8, PieceType>(((self.data & CAPTURE_MASK) >> 16) as u8) }
    }

    #[inline(always)]
    pub fn flag(&self) -> MoveFlag {
        unsafe { transmute::<u8, MoveFlag>(((self.data & FLAG_MASK) >> 20) as u8) }
    }

    #[inline(always)]
    pub fn encode(source: Square, target: Square, piece: PieceType, capture: PieceType, flag: MoveFlag) -> BitMove {
        BitMove {
            data: source.0 as u32 | (target.0 as u32) << 6 | (piece as u32) << 12 | (capture as u32) << 16 | (flag as u32) << 20
        }
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
  Move Flag:     {}
        ", self.data, self.source(), self.target(), self.piece(), self.flag())
    }
}
