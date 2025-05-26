use std::mem::transmute;
use core::fmt;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveFlag {
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

impl MoveFlag {
    #[inline(always)]
    pub fn is_promotion(self) -> bool {
        matches!(self, Self::PromoQ | Self::PromoR | Self::PromoB | Self::PromoN)
    }

    #[inline(always)]
    pub fn is_castle(self) -> bool {
        matches!(self, Self::WKCastle | Self::WQCastle | Self::BKCastle | Self::BQCastle)
    }
    
    #[inline(always)]
    pub fn is_en_passant(self) -> bool {
        matches!(self, Self::WEnPassant | Self::BEnPassant)
    }
    
    #[inline(always)]
    pub fn is_double_pawn_push(self) -> bool {
        matches!(self, Self::WDoublePawn | Self::BDoublePawn)
    }
}

impl From<u8> for MoveFlag {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

impl fmt::Display for MoveFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match self {
            MoveFlag::WDoublePawn => "Double Pawn Push",
            MoveFlag::BDoublePawn => "Double Pawn Push",
            MoveFlag::WEnPassant => "En-passant",
            MoveFlag::BEnPassant => "En-passant",
            MoveFlag::WKCastle => "Kingside Castle",
            MoveFlag::WQCastle => "Queenside Castle",
            MoveFlag::BKCastle => "Kingside Castle",
            MoveFlag::BQCastle => "Queenside Castle",
            MoveFlag::PromoN => "Knight Promotion",
            MoveFlag::PromoB => "Bishop Promotion",
            MoveFlag::PromoR => "Rook Promotion",
            MoveFlag::PromoQ => "Queen Promotion",
        })
    }
}
