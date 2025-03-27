use std::mem;

use crate::{bit_move::BitMove, search::MAX_DEPTH};

static mut PRIMARY_KILLER_MOVES: [BitMove; MAX_DEPTH as usize] = unsafe { mem::zeroed() };
static mut SECONDARY_KILLER_MOVES: [BitMove; MAX_DEPTH as usize] = unsafe { mem::zeroed() };

pub struct KillerMoves;

impl KillerMoves {
    #[inline(always)]
    pub fn get_primary(ply: u16) -> Option<BitMove> {
        if ply < MAX_DEPTH {
            unsafe { Some(PRIMARY_KILLER_MOVES[ply as usize]) }
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_secondary(ply: u16) -> Option<BitMove> {
        if ply < MAX_DEPTH {
            unsafe { Some(SECONDARY_KILLER_MOVES[ply as usize]) }
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn update(bit_move: BitMove, ply: u16) {
        if ply < MAX_DEPTH {
            unsafe {
                SECONDARY_KILLER_MOVES[ply as usize] = PRIMARY_KILLER_MOVES[ply as usize];
                PRIMARY_KILLER_MOVES[ply as usize] = bit_move;
            }
        }
    }

    #[inline(always)]
    pub fn reset() {
        unsafe {
            PRIMARY_KILLER_MOVES = mem::zeroed();
            SECONDARY_KILLER_MOVES = mem::zeroed();
        }
    }
}
