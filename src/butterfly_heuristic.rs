use std::mem;

use crate::{bit_move::BitMove, color::Color, consts::{PLAYER_COUNT, SQUARE_COUNT}, square::Square};

const MAX_SCORE: i16 = 1000;

// Butterfly heuristic table: [side][source][target]
static mut BUTTERFLY_HEURISTIC: [[[i16; SQUARE_COUNT]; SQUARE_COUNT]; PLAYER_COUNT] = unsafe { mem::zeroed() };

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
            let clamped_bonus = bonus.clamp(-MAX_SCORE, MAX_SCORE);
            let butterfly_score = &mut BUTTERFLY_HEURISTIC[side][butterfly_move.source()][butterfly_move.target()];
            
            *butterfly_score =
                (*butterfly_score as f32 + (clamped_bonus as f32 - (*butterfly_score * clamped_bonus.abs()) as f32 / MAX_SCORE as f32)) as i16;
            
            debug_assert!(*butterfly_score <= MAX_SCORE, "The new butterfly score should never be able to exceed the maximum score");
            debug_assert!(*butterfly_score >= -MAX_SCORE, "The new butterfly score should never be able to go below the inverse maximum score");
        }
    }

    #[inline(always)]
    pub fn reset() {
        unsafe {
            BUTTERFLY_HEURISTIC = mem::zeroed();
        }
    }
}
