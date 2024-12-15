use crate::{bit_move::BitMove, fen, move_gen, pl, position::Position, timer::Timer};
use std::sync::Arc;
use rayon::prelude::*;
use crate::{position::Position, fen, move_masks, pl, timer::Timer};

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

static MAIN_PERFT_POSITIONS: [PerftPosition; 5] = [
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

static SHORT_PERFT_POSITIONS: [PerftPosition; 5] = [
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

pub fn perft_test(position: &mut Position, depth: u8, print_result: bool) -> PerftResult {
    let timer = Timer::new();

    if print_result {
        pl!("\n  Performance Test\n");
    }

    let move_list = move_gen::generate_moves(position);

    // Thread-safe clone of position
    let position_arc = Arc::new(position.clone());

    // Computes nodes reached in parallel
    let legal_moves_and_nodes: Vec<(BitMove, u64)> = move_list
        .par_iter()
        .filter_map(|&mv| {
            let mut pos_clone = (*position_arc).clone();
            if pos_clone.make_move(mv) {
                let nodes = perft_driver(Arc::new(pos_clone), depth - 1);
                Some((mv, nodes))
            } else {
                None
            }
        })
        .collect();

    let cumulative_nodes: u64 = legal_moves_and_nodes.iter().map(|(_, nodes)| *nodes).sum();

    if print_result {
        for (mv, nodes) in &legal_moves_and_nodes {
            pl!(format!("  Move: {:<5} Nodes: {}", mv.to_uci_string(), nodes));
        }

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

#[inline(always)]
fn perft_driver(position_arc: Arc<Position>, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let move_list = move_gen::generate_moves(&position_arc);

    // Recursively counts nodes in parallel
    move_list
        .par_iter()
        .map(|mv| {
            let mut pos_clone = (*position_arc).clone();
            if pos_clone.make_move(*mv) {
                perft_driver(Arc::new(pos_clone), depth - 1)
            } else {
                0
            }
        })
        .sum()
}

fn perft_tests(perft_positions: &[PerftPosition; 5]) {
    let mut performances: Vec<u128> = vec![];

    println!("\n    Printing performance test results:");
    println!("  |-----------------------------------------------------------------|");
    println!("  | {:<18} | {:<6} | {:<10} | {:<6} | {:<11} |", "Position", "Depth", "Nodes", "Time", "Performance");
    println!("  |-----------------------------------------------------------------|");

    for perft_position in perft_positions {
        let mut position = fen::parse(perft_position.fen).expect("FEN parser could not parse given position!");
        let perft_result = perft_test(&mut position, perft_position.depth, false);
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

pub fn main_perft_tests() {
    perft_tests(&MAIN_PERFT_POSITIONS);
}

pub fn short_perft_tests() {
    perft_tests(&SHORT_PERFT_POSITIONS);
}
