pub const VERSIONS: &[&str] = &[
    "random",
    "minimax",
    "negamax",
    "eval_pps",
    "sort_moves",
    "iterative_deepening",
    "quiescence",
    "quiescence_en_passant",
    "checks_add_depth",
    "killer_heuristic",
    "butterfly_heuristic",
    "tt",
    "eval_tt",
    "tt_two_tier",
    "null_move_pruning",
    "late_move_reductions",
];

pub const BASE_FEATURES: &[&str] = &[
    "base_basic",
    "base_magic_bitboard",
    "base_clone",
    "base_parallel",
    "base_array",
    "base_parallel_array",
];
