use crate::board_state::BoardState;

// Contains search_position, quiescence, perft tests, etc.
pub struct Engine {
    board_state: BoardState,
    // nodes: Nodes // Use to measure how many nodes were traversed, how many captures etc.
    // timer: Timer // Use to benchmark and limit search time
    // flags: Flags // Use for debugging, verbose etc.
}
