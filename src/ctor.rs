use ctor::ctor;

use crate::{eval_position::EvalPosition, move_masks::MoveMasks, transposition_table::TranspositionTable, zobrist::ZobristKey};

#[ctor]
unsafe fn init() {
    MoveMasks::init_move_masks();
    EvalPosition::init_positional_masks();
    ZobristKey::init_zobrist_keys();
    TranspositionTable::init();
}
