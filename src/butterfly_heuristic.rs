use crate::{bit_move::BitMove, color::Color, square::Square};

const SIDE_COUNT: usize = 2; // White and Black
const SQUARES: usize = 64;
const MAX_SCORE: i16 = 100; // Deliberately lower than any MVV-LVA values

// Butterfly heuristic table: [side][source][target]
static mut BUTTERFLY_HEURISTIC: [[[i16; SQUARES]; SQUARES]; SIDE_COUNT] = [[[0; SQUARES]; SQUARES]; SIDE_COUNT];

pub struct ButterflyHeuristic;

impl ButterflyHeuristic {
    #[inline(always)]
    pub fn get(side: Color, source: Square, target: Square) -> i16 {
        unsafe { BUTTERFLY_HEURISTIC[side][source][target] }
    }

    #[inline(always)]
    pub fn update(side: Color, quiets_searched: &[BitMove], new_best_move: BitMove, bonus: i16) {
        Self::apply_bonus(side, new_best_move, bonus);
        for &quiet_move in quiets_searched {
            Self::apply_bonus(side, quiet_move, -bonus);
        }
    }

    #[inline(always)]
    pub fn apply_bonus(side: Color, butterfly_move: BitMove, bonus: i16) {
        unsafe {
            let butterfly_score = &mut BUTTERFLY_HEURISTIC[side][butterfly_move.source()][butterfly_move.target()];
            
            *butterfly_score =
                (*butterfly_score as f32 + (bonus as f32 - (*butterfly_score * bonus.abs()) as f32 / MAX_SCORE as f32)) as i16;
            
            debug_assert!(*butterfly_score <= MAX_SCORE, "The new butterfly score should never be able to exceed the maximum score");
            debug_assert!(*butterfly_score >= -MAX_SCORE, "The new butterfly score should never be able to go below the inverse maximum score");
        }
    }

    #[inline(always)]
    pub fn reset() {
        unsafe {
            BUTTERFLY_HEURISTIC = [[[0; SQUARES]; SQUARES]; SIDE_COUNT];
        }
    }
}
