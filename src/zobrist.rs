use std::{fmt::Display, mem};

use ctor::ctor;

use crate::{castling_rights::CastlingRights, color::Color, piece::PieceType, position::Position, rng::RandomNumberGenerator, square::Square};

// Constants for Zobrist hashing
const FILE_COUNT: usize = 8;
const PIECE_TYPES: usize = 12;
const SQUARES: usize = 64;
const CASTLING_PERMUTATIONS: usize = 16;

// Zobrist keys
static mut PIECE_KEYS: [[u64; SQUARES]; PIECE_TYPES] = unsafe { mem::zeroed() };
static mut CASTLING_KEYS: [u64; CASTLING_PERMUTATIONS] = unsafe { mem::zeroed() };
static mut EN_PASSANT_KEYS: [u64; FILE_COUNT] = unsafe { mem::zeroed() };
static mut SIDE_KEY: u64 = 0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ZobristKey(pub u64);

impl ZobristKey {
    #[inline(always)]
    pub fn generate(position: &Position) -> ZobristKey {
        let mut hash = 0_u64;
    
        unsafe {
            for square in Square::ALL_SQUARES {
                let piece = position.get_piece(square);
                if piece != PieceType::None {
                    hash ^= PIECE_KEYS[piece as usize][square];
                }
            }
    
            hash ^= CASTLING_KEYS[position.castling_rights.0 as usize];
    
            if position.en_passant_sq != Square::None {
                let file = position.en_passant_sq.file();
                hash ^= EN_PASSANT_KEYS[file as usize];
            }
    
            if position.side == Color::White {
                hash ^= SIDE_KEY;
            }
        }
    
        ZobristKey(hash)
    }

    #[inline(always)]
    pub fn mod_piece(&mut self, piece: PieceType, square: Square) {
        unsafe {
            if piece != PieceType::None {
                self.0 ^= PIECE_KEYS[piece as usize][square];
            }
        }
    }

    #[inline(always)]
    pub fn mod_castling(&mut self, castling_rights: CastlingRights) {
        unsafe { self.0 ^= CASTLING_KEYS[castling_rights.0 as usize]; }
    }

    #[inline(always)]
    pub fn mod_en_passant(&mut self, en_passant_square: Square) {
        unsafe { 
            if en_passant_square != Square::None {
                self.0 ^= EN_PASSANT_KEYS[en_passant_square.file() as usize]; 
            }
        }
    }

    #[inline(always)]
    pub fn mod_side(&mut self, side: Color) {
        unsafe { 
            if side == Color::White {
                self.0 ^= SIDE_KEY; 
            }
        }
    }
}

impl Display for ZobristKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{:b}", self.0))
    }
}

#[ctor]
#[allow(static_mut_refs)]
unsafe fn init_zobrist() {
    let mut rng = RandomNumberGenerator::default();
    
    for piece_array_key in PIECE_KEYS.iter_mut() {
        for square_key in piece_array_key.iter_mut() {
            *square_key = rng.generate_u64();
        }
    }

    for castling_key in CASTLING_KEYS.iter_mut() {
        *castling_key = rng.generate_u64();
    }

    for en_passant_key in EN_PASSANT_KEYS.iter_mut() {
        *en_passant_key = rng.generate_u64();
    }

    SIDE_KEY = rng.generate_u64();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zobrist_hash_consistency() {
        let position = Position::starting_position();
        let hash1 = ZobristKey::generate(&position);
        let hash2 = ZobristKey::generate(&position);
        assert_eq!(hash1, hash2, "Zobrist hash should be consistent for the same position");
    }

    #[test]
    #[cfg(feature = "board_representation_array")]
    fn test_zobrist_hash_different_positions() {
        let position1 = Position::starting_position();
        let mut position2 = Position::starting_position();

        // Modify position2 by moving e2 to e4
        position2.pps[Square::E2 as usize] = PieceType::None;
        position2.pps[Square::E4 as usize] = PieceType::WP;

        let hash1 = ZobristKey::generate(&position1);
        let hash2 = ZobristKey::generate(&position2);

        assert_ne!(hash1, hash2, "Different positions should have different hashes");
    }

    #[test]
    fn test_zobrist_hash_side_to_move() {
        let mut position = Position::starting_position();
        let hash1 = ZobristKey::generate(&position);

        // Change side to move
        position.side.switch();
        let hash2 = ZobristKey::generate(&position);

        assert_ne!(hash1, hash2, "Changing side to move should change the hash");
    }

    #[test]
    fn test_zobrist_hash_castling_rights() {
        let mut position = Position::starting_position();
        let hash1 = ZobristKey::generate(&position);

        // Remove castling rights
        position.castling_rights.0 = 0;
        let hash2 = ZobristKey::generate(&position);

        assert_ne!(hash1, hash2, "Changing castling rights should change the hash");
    }

    #[test]
    fn test_zobrist_hash_en_passant() {
        let mut position = Position::starting_position();
        let hash1 = ZobristKey::generate(&position);

        // Set an en passant square
        position.en_passant_sq = Square::E3;
        let hash2 = ZobristKey::generate(&position);

        assert_ne!(hash1, hash2, "Setting an en passant square should change the hash");

        // Reset en passant square
        position.en_passant_sq = Square::None;
        let hash3 = ZobristKey::generate(&position);

        assert_eq!(hash1, hash3, "Clearing en passant should restore original hash");
    }
}
