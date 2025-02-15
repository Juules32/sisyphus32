use crate::{fen::FenString, pl, position::Position, timer::Timer, move_generation::MoveGeneration};

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
        #[cfg(all(feature = "perft_single_thread", feature = "revert_with_undo_move"))]
        return Self::perft_test_single_thread_undo_move(position, depth, print_result);

        #[cfg(all(feature = "perft_single_thread", feature = "revert_with_clone"))]
        return Self::perft_test_single_thread_clone(position, depth, print_result);

        #[cfg(feature = "perft_parallelize")]
        return Self::perft_test_parallelize(position, depth, print_result);
    }

    #[inline(always)]
    pub fn perft_test_single_thread_undo_move(position: &Position, depth: u8, print_result: bool) -> PerftResult {
        let mut current_nodes = 0_u64;
        let mut cumulative_nodes = 0_u64;
        let timer = Timer::new();

        if print_result { pl!("\n  Performance Test\n"); }

        let mut position_copy = position.clone();

        #[cfg(feature = "revert_with_undo_move")]
        let old_castling_rights = position.castling_rights;
        
        for mv in MoveGeneration::generate_pseudo_legal_moves(position).iter() {
            if position_copy.make_move(*mv) {
                current_nodes += Self::perft_driver_single_thread_undo_move(&position_copy, depth - 1);
            }

            #[cfg(feature = "revert_with_undo_move")]
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

    #[inline(always)]
    pub fn perft_test_single_thread_clone(position: &Position, depth: u8, print_result: bool) -> PerftResult {
        let mut current_nodes = 0_u64;
        let mut cumulative_nodes = 0_u64;
        let timer = Timer::new();

        if print_result { pl!("\n  Performance Test\n"); }

        for mv in MoveGeneration::generate_pseudo_legal_moves(position).iter() {
            let mut position_copy = position.clone();

            if position_copy.make_move(*mv) {
                current_nodes += Self::perft_driver_single_thread_clone(&position_copy, depth - 1);
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

    #[inline(always)]
    pub fn perft_test_parallelize(position: &Position, depth: u8, print_result: bool) -> PerftResult {
        let timer = Timer::new();

        if print_result {
            pl!("\n  Performance Test\n");
        }

        let move_list = MoveGeneration::generate_pseudo_legal_moves(position);

        // Thread-safe clone of position
        let position_arc = Arc::new(position.clone());

        // Computes nodes reached in parallel
        let cumulative_nodes = move_list
            .par_iter()
            .map(|&mv| {
                let mut position_arc_copy = (*position_arc).clone();
                if position_arc_copy.make_move(mv) {
                    let nodes = Self::perft_driver_parallelize(Arc::new(position_arc_copy), depth - 1);
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

    #[inline(always)]
    fn perft_driver_single_thread_undo_move(position: &Position, depth: u8) -> u64 {
        if depth == 0 {
            1
        } else {
            let mut nodes = 0;
            let mut position_copy = position.clone();

            #[cfg(feature = "revert_with_undo_move")]
            let old_castling_rights = position.castling_rights;
            
            for mv in MoveGeneration::generate_pseudo_legal_moves(position).iter() {
                if position_copy.make_move(*mv) {
                    nodes += Self::perft_driver_single_thread_undo_move(&position_copy, depth - 1);
                }

                #[cfg(feature = "revert_with_undo_move")]
                position_copy.undo_move(*mv, old_castling_rights);
            }
            nodes
        }
    }

    #[inline(always)]
    fn perft_driver_single_thread_clone(position: &Position, depth: u8) -> u64 {
        if depth == 0 {
            1
        } else {
            MoveGeneration::generate_pseudo_legal_moves(position)
                .iter()
                .map(|mv| {
                    let mut position_copy = position.clone();
                    if position_copy.make_move(*mv) {
                        Self::perft_driver_single_thread_clone(&position_copy, depth - 1)
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
            MoveGeneration::generate_pseudo_legal_moves(&position_arc)
                .iter()
                .map(|mv| {
                    let mut position_arc_copy = (*position_arc).clone();
                    if position_arc_copy.make_move(*mv) {
                        Self::perft_driver_parallelize(Arc::new(position_arc_copy), depth - 1)
                    } else {
                        0
                    }
                })
                .sum()
        } else {
            // Recursively counts nodes in parallel
            MoveGeneration::generate_pseudo_legal_moves(&position_arc)
                .par_iter()
                .map(|mv| {
                    let mut position_arc_copy = (*position_arc).clone();
                    if position_arc_copy.make_move(*mv) {
                        Self::perft_driver_parallelize(Arc::new(position_arc_copy), depth - 1)
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
    use crate::move_masks;

    use super::*;

    #[test]
    fn short_perft_tests_are_correct() {
        move_masks::init();
        Perft::short_perft_tests();
    }
}
