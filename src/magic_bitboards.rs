use crate::{bitboard::Bitboard, move_masks::{self, MoveMasks}, rng::RandomNumberGenerator, square::Square};

const MAX_SLIDER_MOVE_PERMUTATIONS: usize = 4096;
const NUM_CANDIDATES: usize = 10_000_000;

#[derive(Default)]
struct MagicBitboardGenerator {
    rng: RandomNumberGenerator
}

impl MagicBitboardGenerator {
    fn generate_magic_bitboard_candidate(&mut self) -> Bitboard {
        Bitboard(self.rng.generate_sparse_u64())
    }

    pub fn generate_magic_bitboard(&mut self, square: Square, num_relevant_bits: u8, is_bishop: bool) -> Bitboard {
        let mut occupancies = [Bitboard::EMPTY; MAX_SLIDER_MOVE_PERMUTATIONS];
        let mut moves = [Bitboard::EMPTY; MAX_SLIDER_MOVE_PERMUTATIONS];
        let mask = unsafe { if is_bishop { move_masks::BISHOP_MASKS[square] } else { move_masks::ROOK_MASKS[square] } };
        let max_occupancy_index = 1 << num_relevant_bits;

        for i in 0..max_occupancy_index {
            occupancies[i] = MoveMasks::generate_occupancy_permutation(i as u32, num_relevant_bits, mask);
            
            if is_bishop {
                moves[i] = MoveMasks::generate_bishop_moves_on_the_fly(square, occupancies[i]);
            } else {
                moves[i] = MoveMasks::generate_rook_moves_on_the_fly(square, occupancies[i]);
            }
        }

        for _ in 0..NUM_CANDIDATES {
            let magic_bitboard_candidate = self.generate_magic_bitboard_candidate();
            
            // Skip inappropriate magic bitboards
            if Bitboard(mask.0.wrapping_mul(magic_bitboard_candidate.0) & 0xFF00000000000000).count_bits() < 6 {
                continue;
            }

            let mut used_moves = [Bitboard::EMPTY; MAX_SLIDER_MOVE_PERMUTATIONS];

            let mut failed = false;
            for i in 0..max_occupancy_index {
                if failed { break };

                let magic_index = ((occupancies[i].0.wrapping_mul(magic_bitboard_candidate.0)) >> (64 - num_relevant_bits)) as usize;

                if used_moves[magic_index].is_empty() {
                    used_moves[magic_index] = moves[i];
                } else if used_moves[magic_index] != moves[i] {
                    failed = true;
                }
            }

            if !failed {
                return magic_bitboard_candidate;
            }
        }

        panic!("No magic bitboard could be found");
    }

    // Prints magic bitboards which can be copied and used for move generation
    pub fn print_magic_bitboards(&mut self) {

        println!("\nRook magic bitboards:");
        for square in Square::ALL_SQUARES {
            println!("0x{:x},", self.generate_magic_bitboard(square, move_masks::ROOK_RELEVANT_BITS[square], false).0);
        }
        
        println!("\nBishop magic bitboards:");
        for square in Square::ALL_SQUARES {
            println!("0x{:x},", self.generate_magic_bitboard(square, move_masks::BISHOP_RELEVANT_BITS[square], true).0);
        }
    }
}
