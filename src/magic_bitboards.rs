use crate::{bitboard::Bitboard, move_init, square::Square};

#[allow(dead_code)]
pub struct MagicBitboardGenerator {
    pub seed: u32
}

#[allow(dead_code)]
impl MagicBitboardGenerator {
    fn generate_u32(&mut self) -> u32 {
        self.seed ^= self.seed << 13; 
        self.seed ^= self.seed >> 17; 
        self.seed ^= self.seed << 5;

        self.seed
    }

    fn generate_u64(&mut self) -> u64 {
        let [a, b, c, d] = std::array::from_fn(|_| (self.generate_u32() & 0xFFFF) as u64);
        a | (b << 16) | (c << 32) | (d << 48)
    }

    fn generate_sparse_u64(&mut self) -> u64 {
        self.generate_u64() & self.generate_u64() & self.generate_u64()
    }

    fn generate_magic_bitboard_candidate(&mut self) -> Bitboard {
        Bitboard(self.generate_sparse_u64())
    }

    pub fn generate_magic_bitboard(&mut self, square: Square, num_relevant_bits: u8, is_bishop: bool) -> Bitboard {
        let mut occupancies = [Bitboard::EMPTY; 4096];
        let mut moves = [Bitboard::EMPTY; 4096];
        let mask = unsafe { if is_bishop { move_init::BISHOP_MASKS[square] } else { move_init::ROOK_MASKS[square] } };
        let max_occupancy_index = 1 << num_relevant_bits;

        for i in 0..max_occupancy_index {
            occupancies[i] = move_init::generate_occupancy_permutation(i as u32, num_relevant_bits, mask);
            
            if is_bishop {
                moves[i] = move_init::generate_bishop_moves_on_the_fly(square, occupancies[i]);
            }
            else {
                moves[i] = move_init::generate_rook_moves_on_the_fly(square, occupancies[i]);
            }
        }

        for _ in 0..10000000 {
            let magic_bitboard_candidate = self.generate_magic_bitboard_candidate();
            
            // Skip inappropriate magic bitboards
            if Bitboard(mask.0.wrapping_mul(magic_bitboard_candidate.0) & 0xFF00000000000000).count_bits() < 6 {
                continue;
            }

            let mut used_moves = [Bitboard::EMPTY; 4096];

            let mut failed = false;
            for i in 0..max_occupancy_index {
                if failed { break };

                let magic_index = ((occupancies[i].0.wrapping_mul(magic_bitboard_candidate.0)) >> (64 - num_relevant_bits)) as usize;

                if used_moves[magic_index].is_empty() {
                    used_moves[magic_index] = moves[i];
                }

                else if used_moves[magic_index] != moves[i] {
                    failed = true;
                }
            }

            if !failed {
                return magic_bitboard_candidate;
            }
        }

        panic!("No magic bitboard could be found");
    }

    // Outputs magic bitboards which can be copied and used for move generation
    pub fn print_magic_bitboards(&mut self) {

        println!("\nRook magic bitboards:");
        for square in Square::ALL_SQUARES {
            println!("0x{:x},", self.generate_magic_bitboard(square, move_init::ROOK_RELEVANT_BITS[square], false).0);
        }
        
        println!("\nBishop magic bitboards:");
        for square in Square::ALL_SQUARES {
            println!("0x{:x},", self.generate_magic_bitboard(square, move_init::BISHOP_RELEVANT_BITS[square], true).0);
        }
    }
}
