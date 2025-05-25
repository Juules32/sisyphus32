use ctor::ctor;

use crate::{EvalPosition, MoveMasks, TranspositionTable, ZobristKey};

#[cfg(not(target_arch = "wasm32"))]
#[ctor]
unsafe fn ctor() {
    init();
}

pub unsafe fn init() {
    MoveMasks::init_move_masks();
    EvalPosition::init_positional_masks();
    ZobristKey::init_zobrist_keys();
    TranspositionTable::init();
}
