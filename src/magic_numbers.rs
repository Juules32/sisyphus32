use crate::{Bitboard, SQUARE_COUNT, MoveMasks, RandomNumberGenerator, Square};

const MAX_SLIDER_MOVE_PERMUTATIONS: usize = 4096;
const NUM_CANDIDATES: usize = 10_000_000;

#[derive(Default)]
pub(crate) struct MagicNumberGenerator {
    rng: RandomNumberGenerator
}

impl MagicNumberGenerator {
    fn generate_magic_number_candidate(&mut self) -> u64 {
        self.rng.generate_sparse_u64()
    }

    pub(crate) fn generate_magic_number(&mut self, square: Square, num_relevant_bits: u8, is_bishop: bool) -> u64 {
        let mut occupancies = [Bitboard::EMPTY; MAX_SLIDER_MOVE_PERMUTATIONS];
        let mut moves = [Bitboard::EMPTY; MAX_SLIDER_MOVE_PERMUTATIONS];
        let mask = if is_bishop { MoveMasks::get_bishop_base_mask(square) } else { MoveMasks::get_rook_base_mask(square) };
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
            let magic_number_candidate = self.generate_magic_number_candidate();
            
            // Skip inappropriate magic numbers
            if Bitboard(mask.0.wrapping_mul(magic_number_candidate) & 0xFF00000000000000).count_bits() < 6 {
                continue;
            }

            let mut used_moves = [Bitboard::EMPTY; MAX_SLIDER_MOVE_PERMUTATIONS];

            let mut failed = false;
            for i in 0..max_occupancy_index {
                if failed { break };

                let magic_index = ((occupancies[i].0.wrapping_mul(magic_number_candidate)) >> (SQUARE_COUNT as u8 - num_relevant_bits)) as usize;

                if used_moves[magic_index].is_empty() {
                    used_moves[magic_index] = moves[i];
                } else if used_moves[magic_index] != moves[i] {
                    failed = true;
                }
            }

            if !failed {
                return magic_number_candidate;
            }
        }

        panic!("No magic number could be found");
    }

    // Prints magic numbers which can be copied and used for move generation
    pub(crate) fn print_magic_numbers(&mut self) {
        println!("\nBishop magic numbers:");
        for square in Square::ALL_SQUARES {
            println!("0x{:x},", self.generate_magic_number(square, MoveMasks::get_bishop_relevant_bits(square), true));
        }

        println!("\nRook magic numbers:");
        for square in Square::ALL_SQUARES {
            println!("0x{:x},", self.generate_magic_number(square, MoveMasks::get_rook_relevant_bits(square), false));
        }
        
    }
}
