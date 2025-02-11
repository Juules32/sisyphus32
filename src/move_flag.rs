use std::mem::transmute;
use core::fmt;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveFlag {
    None,
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

impl From<u8> for MoveFlag {
    #[inline(always)]
    fn from(number: u8) -> Self {
        unsafe { transmute::<u8, Self>(number) }
    }
}

impl fmt::Display for MoveFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            MoveFlag::None => "None",
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
        };
        f.pad(name)
    }
}
