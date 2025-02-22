use crate::bit_move::BitMove;

const MAX_PLY: usize = 64;

static mut PRIMARY_KILLER_MOVES: [BitMove; MAX_PLY] = [BitMove::EMPTY; MAX_PLY];
static mut SECONDARY_KILLER_MOVES: [BitMove; MAX_PLY] = [BitMove::EMPTY; MAX_PLY];

pub struct KillerMoves;

impl KillerMoves {
    #[inline(always)]
    pub fn get_primary(ply: u16) -> BitMove {
        unsafe { PRIMARY_KILLER_MOVES[ply as usize] }
    }

    #[inline(always)]
    pub fn get_secondary(ply: u16) -> BitMove {
        unsafe { PRIMARY_KILLER_MOVES[ply as usize] }
    }

    #[inline(always)]
    pub fn update(bit_move: BitMove, ply: u16) {
        unsafe {
            SECONDARY_KILLER_MOVES[ply as usize] = PRIMARY_KILLER_MOVES[ply as usize];
            PRIMARY_KILLER_MOVES[ply as usize] = bit_move;
        }
    }

    #[inline(always)]
    pub fn reset() {
        unsafe {
            PRIMARY_KILLER_MOVES = [BitMove::EMPTY; MAX_PLY];
            SECONDARY_KILLER_MOVES = [BitMove::EMPTY; MAX_PLY];
        }
    }
}
