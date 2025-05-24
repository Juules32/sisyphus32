use ctor::ctor;

use crate::{eval_position::EvalPosition, move_masks::MoveMasks, transposition_table::TranspositionTable, zobrist::ZobristKey};

#[cfg(not(target_arch = "wasm32"))]
#[ctor]
unsafe fn ctor() {
    init();
}

unsafe fn init() {
    MoveMasks::init_move_masks();
    EvalPosition::init_positional_masks();
    ZobristKey::init_zobrist_keys();
    TranspositionTable::init();
}
