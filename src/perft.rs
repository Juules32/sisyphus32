use crate::{fen, pl, position::Position, timer::Timer};

pub struct PerftResult {
    depth: u8,
    nodes: u64,
    time: u128,
}

struct PerftPosition {
    name: &'static str,
    fen: &'static str,
    depth: u8,
    target_nodes: u64
}

static LONG_PERFT_POSITIONS: [PerftPosition; 5] = [
    PerftPosition {
        name: "Starting Position",
        fen: fen::STARTING_POSITION,
        depth: 6,
        target_nodes: 119_060_324
    },
    PerftPosition {
        name: "Kiwipete Position",
        fen: fen::KIWIPETE_POSITION,
        depth: 5,
        target_nodes: 193_690_690
    },
    PerftPosition {
        name: "Rook Position",
        fen: fen::ROOK_POSITION,
        depth: 7,
        target_nodes: 178_633_661
    },
    PerftPosition {
        name: "Tricky Position",
        fen: fen::TRICKY_POSITION,
        depth: 6,
        target_nodes: 706_045_033
    },
    PerftPosition {
        name: "Tricky Position 2",
        fen: fen::TRICKY_POSITION_2,
        depth: 5,
        target_nodes: 89_941_194
    },
];

static MEDIUM_PERFT_POSITIONS: [PerftPosition; 5] = [
    PerftPosition {
        name: "Starting Position",
        fen: fen::STARTING_POSITION,
        depth: 5,
        target_nodes: 4_865_609
    },
    PerftPosition {
        name: "Kiwipete Position",
        fen: fen::KIWIPETE_POSITION,
        depth: 4,
        target_nodes: 4_085_603
    },
    PerftPosition {
        name: "Rook Position",
        fen: fen::ROOK_POSITION,
        depth: 6,
        target_nodes: 11_030_083
    },
    PerftPosition {
        name: "Tricky Position",
        fen: fen::TRICKY_POSITION,
        depth: 5,
        target_nodes: 15_833_292
    },
    PerftPosition {
        name: "Tricky Position 2",
        fen: fen::TRICKY_POSITION_2,
        depth: 4,
        target_nodes: 2_103_487
    },
];

static SHORT_PERFT_POSITIONS: [PerftPosition; 5] = [
    PerftPosition {
        name: "Starting Position",
        fen: fen::STARTING_POSITION,
        depth: 4,
        target_nodes: 197_281
    },
    PerftPosition {
        name: "Kiwipete Position",
        fen: fen::KIWIPETE_POSITION,
        depth: 3,
        target_nodes: 97_862
    },
    PerftPosition {
        name: "Rook Position",
        fen: fen::ROOK_POSITION,
        depth: 5,
        target_nodes: 674_624
    },
    PerftPosition {
        name: "Tricky Position",
        fen: fen::TRICKY_POSITION,
        depth: 4,
        target_nodes: 422_333
    },
    PerftPosition {
        name: "Tricky Position 2",
        fen: fen::TRICKY_POSITION_2,
        depth: 3,
        target_nodes: 62_379
    },
];

#[cfg(all(feature = "perft_single_thread", feature = "revert_with_undo_move"))]
pub fn perft_test(position: &Position, depth: u8, print_result: bool) -> PerftResult {
    let mut current_nodes = 0_u64;
    let mut cumulative_nodes = 0_u64;
    let timer = Timer::new();

    if print_result { pl!("\n  Performance Test\n"); }

    let mut position_copy = position.clone();
    let old_castling_rights = position.castling_rights;
    
    for mv in position.generate_pseudo_legal_moves().iter() {
        if position_copy.make_move(*mv) {
            current_nodes += perft_driver(&position_copy, depth - 1);
        }
        position_copy.undo_move(*mv, old_castling_rights);

        if print_result {
            pl!(format!("  Move: {:<5} Nodes: {}", mv.to_uci_string(), current_nodes));
        }

        cumulative_nodes += current_nodes;
        current_nodes = 0;
    }

    let perft_result = PerftResult {
        depth,
        nodes: cumulative_nodes,
        time: timer.get_time_passed_millis(),
    };

    if print_result {
        pl!(format!("
Depth: {}
Nodes: {}
 Time: {} milliseconds\n",
            perft_result.depth,
            perft_result.nodes,
            perft_result.time
        ));
    }

    perft_result
}

#[cfg(all(feature = "perft_single_thread", feature = "revert_with_clone"))]
pub fn perft_test(position: &Position, depth: u8, print_result: bool) -> PerftResult {
    let mut current_nodes = 0_u64;
    let mut cumulative_nodes = 0_u64;
    let timer = Timer::new();

    if print_result { pl!("\n  Performance Test\n"); }

    for mv in position.generate_pseudo_legal_moves().iter() {
        let mut position_copy = position.clone();

        if position_copy.make_move(*mv) {
            current_nodes += perft_driver(&position_copy, depth - 1);
        }

        if print_result {
            pl!(format!("  Move: {:<5} Nodes: {}", mv.to_uci_string(), current_nodes));
        }

        cumulative_nodes += current_nodes;
        current_nodes = 0;
    }

    let perft_result = PerftResult {
        depth,
        nodes: cumulative_nodes,
        time: timer.get_time_passed_millis(),
    };

    if print_result {
        pl!(format!("
Depth: {}
Nodes: {}
 Time: {} milliseconds\n",
            perft_result.depth,
            perft_result.nodes,
            perft_result.time
        ));
    }

    perft_result
}

#[cfg(feature = "perft_parallelize")]
pub fn perft_test(position: &Position, depth: u8, print_result: bool) -> PerftResult {
    use {std::sync::Arc, rayon::iter::{IntoParallelRefIterator, ParallelIterator}};

    let timer = Timer::new();

    if print_result {
        pl!("\n  Performance Test\n");
    }

    let move_list = position.generate_pseudo_legal_moves();

    // Thread-safe clone of position
    let position_arc = Arc::new(position.clone());

    // Computes nodes reached in parallel
    let cumulative_nodes = move_list
        .par_iter()
        .map(|&mv| {
            let mut position_arc_copy = (*position_arc).clone();
            if position_arc_copy.make_move(mv) {
                let nodes = perft_driver(Arc::new(position_arc_copy), depth - 1);
                if print_result {
                    pl!(format!("  Move: {:<5} Nodes: {}", mv.to_uci_string(), nodes));
                }
                nodes
            } else {
                0
            }
        })
        .collect::<Vec<_>>().into_iter().sum();

    if print_result {
        pl!(format!(
            "
 Depth: {}
 Nodes: {}
  Time: {} milliseconds\n",
            depth,
            cumulative_nodes,
            timer.get_time_passed_millis()
        ));
    }

    PerftResult {
        depth,
        nodes: cumulative_nodes,
        time: timer.get_time_passed_millis(),
    }
}

#[cfg(all(feature = "perft_single_thread", feature = "revert_with_undo_move"))]
#[inline(always)]
fn perft_driver(position: &Position, depth: u8) -> u64 {
    if depth == 0 {
        1
    } else {
        let mut nodes = 0;
        let mut position_copy = position.clone();
        let old_castling_rights = position.castling_rights;
        
        for mv in position.generate_pseudo_legal_moves().iter() {
            if position_copy.make_move(*mv) {
                nodes += perft_driver(&position_copy, depth - 1);
            }
            position_copy.undo_move(*mv, old_castling_rights);
        }
        nodes
    }
}

#[cfg(all(feature = "perft_single_thread", feature = "revert_with_clone"))]
#[inline(always)]
fn perft_driver(position: &Position, depth: u8) -> u64 {
    if depth == 0 {
        1
    } else {
        position
            .generate_pseudo_legal_moves()
            .iter()
            .map(|mv| {
                let mut position_copy = position.clone();
                if position_copy.make_move(*mv) {
                    perft_driver(&position_copy, depth - 1)
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(feature = "perft_parallelize")]
#[inline(always)]
fn perft_driver(position_arc: std::sync::Arc<Position>, depth: u8) -> u64 {
    use {std::sync::Arc, rayon::iter::{IntoParallelRefIterator, ParallelIterator}};

    if depth == 0 {
        1
    } else if depth <= 2 {
        // Recursively counts nodes sequentially
        position_arc.generate_pseudo_legal_moves()
            .iter()
            .map(|mv| {
                let mut position_arc_copy = (*position_arc).clone();
                if position_arc_copy.make_move(*mv) {
                    perft_driver(Arc::new(position_arc_copy), depth - 1)
                } else {
                    0
                }
            })
            .sum()
    } else {
        // Recursively counts nodes in parallel
        position_arc.generate_pseudo_legal_moves()
            .par_iter()
            .map(|mv| {
                let mut position_arc_copy = (*position_arc).clone();
                if position_arc_copy.make_move(*mv) {
                    perft_driver(Arc::new(position_arc_copy), depth - 1)
                } else {
                    0
                }
            })
            .sum()
    }
}

fn perft_tests(perft_positions: &[PerftPosition; 5]) {
    let mut performances: Vec<u128> = vec![];

    println!("\n    Printing performance test results:");
    println!("  |-----------------------------------------------------------------|");
    println!("  | {:<18} | {:<6} | {:<10} | {:<6} | {:<11} |", "Position", "Depth", "Nodes", "Time", "Performance");
    println!("  |-----------------------------------------------------------------|");

    for perft_position in perft_positions {
        let position = fen::parse(perft_position.fen).expect("FEN parser could not parse given position!");
        let perft_result = perft_test(&position, perft_position.depth, false);
        if perft_result.nodes != perft_position.target_nodes {
            panic!("Perft test of {} did not get the target nodes!", perft_position.name);
        }
        let performance = perft_result.nodes as u128 / perft_result.time;
        performances.push(performance);
        println!("  | {:<18} | {:<6} | {:<10} | {:<6} | {:<11} |", perft_position.name, perft_position.depth, perft_result.nodes, perft_result.time, performance);
    }

    let score = performances.iter().sum::<u128>() / performances.len() as u128;

    println!("  |-----------------------------------------------------------------|");
    println!("  | Overall score: {:<13}                                    |", score);
    println!("  |-----------------------------------------------------------------|");
}

pub fn long_perft_tests() {
    perft_tests(&LONG_PERFT_POSITIONS);
}

pub fn medium_perft_tests() {
    perft_tests(&MEDIUM_PERFT_POSITIONS);
}

pub fn short_perft_tests() {
    perft_tests(&SHORT_PERFT_POSITIONS);
}

#[cfg(test)]
mod tests {
    use crate::move_masks;

    use super::*;

    #[test]
    fn short_perft_tests_are_correct() {
        move_masks::init();
        short_perft_tests();
    }
}
