use crate::{bit_move::BitMove, fen::FenString, move_generation::{MoveGeneration, PseudoLegal}, position::Position, timer::Timer};

use {std::sync::Arc, rayon::iter::{IntoParallelRefIterator, ParallelIterator}};

pub struct PerftResult {
    depth: u8,
    nodes: u64,
    time: u128,
}

struct PerftPosition {
    name: &'static str,
    fen: FenString,
    depth: u8,
    target_nodes: u64
}

pub struct Perft { }

impl Perft {
    #[inline(always)]
    pub fn perft_test(position: &Position, depth: u8, print_result: bool) -> PerftResult {
        #[cfg(all(not(feature = "unit_parallel_perft"), feature = "unit_revert_undo"))]
        return Self::perft_test_single_thread_undo_move(position, depth, print_result);

        #[cfg(all(not(feature = "unit_parallel_perft"), feature = "unit_revert_clone"))]
        return Self::perft_test_single_thread_clone(position, depth, print_result);

        #[cfg(feature = "unit_parallel_perft")]
        return Self::perft_test_parallelize(position, depth, print_result);
    }

    #[inline(always)]
    pub fn perft_test_single_thread_undo_move(position: &Position, depth: u8, print_result: bool) -> PerftResult {
        let mut current_nodes = 0_u64;
        let mut cumulative_nodes = 0_u64;
        let timer = Timer::new();

        if print_result { println!("\n  Performance Test\n"); }

        let mut position_copy = position.clone();

        #[cfg(feature = "unit_revert_undo")]
        let old_castling_rights = position.castling_rights;
        
        for bit_move in MoveGeneration::generate_moves::<BitMove, PseudoLegal>(position) {
            position_copy.make_move(bit_move);
            if !position_copy.in_check(position_copy.side.opposite()) {
                current_nodes += Self::perft_driver_single_thread_undo_move(&position_copy, depth - 1);

                if print_result {
                    println!("  Move: {:<5} Nodes: {}", bit_move.to_uci_string(), current_nodes);
                }

                cumulative_nodes += current_nodes;
                current_nodes = 0;
            }

            #[cfg(feature = "unit_revert_undo")]
            position_copy.undo_move(bit_move, old_castling_rights);
        }

        let perft_result = PerftResult {
            depth,
            nodes: cumulative_nodes,
            time: timer.get_time_passed_millis(),
        };

        if print_result {
            println!("
    Depth: {}
    Nodes: {}
    Time: {} milliseconds\n",
                perft_result.depth,
                perft_result.nodes,
                perft_result.time
            );
        }

        perft_result
    }

    #[inline(always)]
    pub fn perft_test_single_thread_clone(position: &Position, depth: u8, print_result: bool) -> PerftResult {
        let mut current_nodes = 0_u64;
        let mut cumulative_nodes = 0_u64;
        let timer = Timer::new();

        if print_result { println!("\n  Performance Test\n"); }

        for bit_move in MoveGeneration::generate_moves::<BitMove, PseudoLegal>(position) {
            if let Some(new_position) = position.apply_pseudo_legal_move(bit_move) {
                current_nodes += Self::perft_driver_single_thread_clone(&new_position, depth - 1);
                
                if print_result {
                    println!("  Move: {:<5} Nodes: {}", bit_move.to_uci_string(), current_nodes);
                }
    
                cumulative_nodes += current_nodes;
                current_nodes = 0;
            }
        }

        let perft_result = PerftResult {
            depth,
            nodes: cumulative_nodes,
            time: timer.get_time_passed_millis(),
        };

        if print_result {
            println!("
    Depth: {}
    Nodes: {}
     Time: {} milliseconds\n",
                perft_result.depth,
                perft_result.nodes,
                perft_result.time
            );
        }

        perft_result
    }

    #[inline(always)]
    pub fn perft_test_parallelize(position: &Position, depth: u8, print_result: bool) -> PerftResult {
        let timer = Timer::new();

        if print_result {
            println!("\n  Performance Test\n");
        }

        // Thread-safe clone of position
        let position_arc = Arc::new(position.clone());

        // Computes nodes reached in parallel
        let cumulative_nodes = MoveGeneration::generate_moves::<BitMove, PseudoLegal>(position)
            .par_iter()
            .map(|&bit_move| {
                if let Some(new_position) = (*position_arc).apply_pseudo_legal_move(bit_move) {
                    let nodes = Self::perft_driver_parallelize(Arc::new(new_position), depth - 1);
                    if print_result {
                        println!("  Move: {:<5} Nodes: {}", bit_move.to_uci_string(), nodes);
                    }
                    nodes
                } else {
                    0
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .sum();

        if print_result {
            println!(
                "
    Depth: {}
    Nodes: {}
     Time: {} milliseconds\n",
                depth,
                cumulative_nodes,
                timer.get_time_passed_millis()
            );
        }

        PerftResult {
            depth,
            nodes: cumulative_nodes,
            time: timer.get_time_passed_millis(),
        }
    }

    #[inline(always)]
    fn perft_driver_single_thread_undo_move(position: &Position, depth: u8) -> u64 {
        if depth == 0 {
            1
        } else {
            let mut nodes = 0;
            let mut position_copy = position.clone();

            #[cfg(feature = "unit_revert_undo")]
            let old_castling_rights = position.castling_rights;
            
            for bit_move in MoveGeneration::generate_moves::<BitMove, PseudoLegal>(position) {
                position_copy.make_move(bit_move);
                if !position_copy.in_check(position_copy.side.opposite()) {
                    nodes += Self::perft_driver_single_thread_undo_move(&position_copy, depth - 1);
                }

                #[cfg(feature = "unit_revert_undo")]
                position_copy.undo_move(bit_move, old_castling_rights);
            }
            nodes
        }
    }

    #[inline(always)]
    fn perft_driver_single_thread_clone(position: &Position, depth: u8) -> u64 {
        if depth == 0 {
            1
        } else {
            MoveGeneration::generate_moves::<BitMove, PseudoLegal>(position)
                .iter()
                .map(|&bit_move| {
                    if let Some(new_position) = position.apply_pseudo_legal_move(bit_move) {
                        Self::perft_driver_single_thread_clone(&new_position, depth - 1)
                    } else {
                        0
                    }
                })
                .sum()
        }
    }

    #[inline(always)]
    fn perft_driver_parallelize(position_arc: std::sync::Arc<Position>, depth: u8) -> u64 {
        if depth == 0 {
            1
        } else if depth <= 2 {
            // Recursively counts nodes sequentially
            MoveGeneration::generate_moves::<BitMove, PseudoLegal>(&position_arc)
                .iter()
                .map(|&bit_move| {
                    if let Some(new_position) = (*position_arc).apply_pseudo_legal_move(bit_move) {
                        Self::perft_driver_parallelize(Arc::new(new_position), depth - 1)
                    } else {
                        0
                    }
                })
                .sum()
        } else {
            // Recursively counts nodes in parallel
            MoveGeneration::generate_moves::<BitMove, PseudoLegal>(&position_arc)
                .par_iter()
                .map(|&bit_move| {
                    if let Some(new_position) = (*position_arc).apply_pseudo_legal_move(bit_move) {
                        Self::perft_driver_parallelize(Arc::new(new_position), depth - 1)
                    } else {
                        0
                    }
                })
                .sum()
        }
    }

    fn perft_tests(perft_positions: [PerftPosition; 5]) {
        let mut performances: Vec<u128> = vec![];

        println!("\n    Printing performance test results:");
        println!("  |-----------------------------------------------------------------|");
        println!("  | {:<18} | {:<6} | {:<10} | {:<6} | {:<11} |", "Position", "Depth", "Nodes", "Time", "Performance");
        println!("  |-----------------------------------------------------------------|");

        for perft_position in perft_positions {
            let position = perft_position.fen.parse().expect("FEN parser could not parse given position!");
            let perft_result = Self::perft_test(&position, perft_position.depth, false);
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
        Self::perft_tests([
            PerftPosition {
                name: "Starting Position",
                fen: FenString::starting(),
                depth: 6,
                target_nodes: 119_060_324
            },
            PerftPosition {
                name: "Kiwipete Position",
                fen: FenString::kiwipete(),
                depth: 5,
                target_nodes: 193_690_690
            },
            PerftPosition {
                name: "Rook Position",
                fen: FenString::rook(),
                depth: 7,
                target_nodes: 178_633_661
            },
            PerftPosition {
                name: "Tricky Position",
                fen: FenString::tricky(),
                depth: 6,
                target_nodes: 706_045_033
            },
            PerftPosition {
                name: "Tricky Position 2",
                fen: FenString::tricky2(),
                depth: 5,
                target_nodes: 89_941_194
            },
        ]);
    }

    pub fn medium_perft_tests() {
        Self::perft_tests([
            PerftPosition {
                name: "Starting Position",
                fen: FenString::starting(),
                depth: 5,
                target_nodes: 4_865_609
            },
            PerftPosition {
                name: "Kiwipete Position",
                fen: FenString::kiwipete(),
                depth: 4,
                target_nodes: 4_085_603
            },
            PerftPosition {
                name: "Rook Position",
                fen: FenString::rook(),
                depth: 6,
                target_nodes: 11_030_083
            },
            PerftPosition {
                name: "Tricky Position",
                fen: FenString::tricky(),
                depth: 5,
                target_nodes: 15_833_292
            },
            PerftPosition {
                name: "Tricky Position 2",
                fen: FenString::tricky2(),
                depth: 4,
                target_nodes: 2_103_487
            },
        ]);
    }

    pub fn short_perft_tests() {
        Self::perft_tests([
            PerftPosition {
                name: "Starting Position",
                fen: FenString::starting(),
                depth: 4,
                target_nodes: 197_281
            },
            PerftPosition {
                name: "Kiwipete Position",
                fen: FenString::kiwipete(),
                depth: 3,
                target_nodes: 97_862
            },
            PerftPosition {
                name: "Rook Position",
                fen: FenString::rook(),
                depth: 5,
                target_nodes: 674_624
            },
            PerftPosition {
                name: "Tricky Position",
                fen: FenString::tricky(),
                depth: 4,
                target_nodes: 422_333
            },
            PerftPosition {
                name: "Tricky Position 2",
                fen: FenString::tricky2(),
                depth: 3,
                target_nodes: 62_379
            },
        ]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_perft_tests_are_correct() {
        Perft::short_perft_tests();
    }
}
