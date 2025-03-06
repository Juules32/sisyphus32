use ctor::ctor;

use crate::{eval_position::EvalPosition, move_masks::MoveMasks, zobrist::ZobristKey};

#[ctor]
unsafe fn init() {
    EvalPosition::init_positional_masks();
    MoveMasks::init_move_masks();
    ZobristKey::init_zobrist_keys();
}
