extern crate rand;

use rand::Rng;

use crate::{bit_move::ScoringMove, eval::Eval, move_generation::MoveGeneration, pl, position::Position, timer::Timer};

pub struct Search {
    timer: Timer,
    stop_time: u128,
    stop_calculating: bool,
    nodes: u64,
    // pv, killer_moves, etc...
}

impl Search {
    pub fn new(stop_time: u128) -> Search {
        Search {
            timer: Timer::new(),
            stop_time,
            stop_calculating: false,
            nodes: 0,
        }
    }

    fn random_best_move(&self, position: &Position, _depth: u8) -> ScoringMove {
        let moves = MoveGeneration::generate_legal_moves(position);
        ScoringMove::from(moves[rand::rng().random_range(0..moves.len())])
    }
    
    fn minimax_best_move(&mut self, position: &Position, depth: u8) -> ScoringMove {
        self.nodes += 1;

        if self.nodes % 5000 == 0 && self.timer.get_time_passed_millis() > self.stop_time {
            self.stop_calculating = true;
        }

        if self.stop_calculating {
            return ScoringMove::blank(12345)
        }
        
        if depth == 0 {
            return Eval::basic(position);
        }
    
        MoveGeneration::generate_pseudo_legal_scoring_moves(position)
            .into_iter()
            .filter_map(|mut m: ScoringMove| {
                let mut position_copy = position.clone();
                if position_copy.make_move(m.bit_move) {
                    m.score = -self.minimax_best_move(&position_copy, depth - 1).score;
                    Some(m)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or_else(|| {
                if position.in_check() {
                    ScoringMove::blank(-10000)
                } else {
                    ScoringMove::blank(0)
                }
            })
    }

    fn best_scoring_move(&mut self, position: &mut Position, depth: u8) -> ScoringMove {
        #[cfg(feature = "search_random")]
        return self.random_best_move(position, depth);
        
        #[cfg(feature = "search_minimax")]
        return self.minimax_best_move(position, depth);
    }
    
    pub fn go(&mut self, position: &mut Position, depth: u8) {
        //TODO: Implement conditional iterative deepening here
        println!("Searching for best move within {} milliseconds", self.stop_time);

        #[cfg(feature = "iterative_deepening")]
        {
            let mut best_scoring_move = ScoringMove::blank(13243);
            for current_depth in 1..=depth {
                self.nodes = 0;
                let new_best_move = self.best_scoring_move(position, current_depth);
                if self.stop_calculating {
                    break
                }
                best_scoring_move = new_best_move;
                pl!(format!("info depth {} score cp {} nodes {} time {} pv {}", current_depth, best_scoring_move.score, self.nodes, self.timer.get_time_passed_millis(), best_scoring_move.bit_move.to_uci_string()));
            }
            pl!(format!("bestmove {}", best_scoring_move.bit_move.to_uci_string()));
        }

        #[cfg(feature = "no_iterative_deepening")]
        {
            let best_scoring_move = self.best_scoring_move(position, depth);
            pl!(format!("info depth {} score cp {} nodes {} time {} pv {}", depth, best_scoring_move.score, self.nodes, self.timer.get_time_passed_millis(), best_scoring_move.bit_move.to_uci_string()));
            pl!(format!("bestmove {}", best_scoring_move.bit_move.to_uci_string()));
        }
    }

    const AVERAGE_AMOUNT_OF_MOVES: u128 = 30;
    const TIME_OFFSET: u128 = 100;

    pub fn calculate_stop_time(total_time: u128, increment: u128) -> u128 {
        total_time / Self::AVERAGE_AMOUNT_OF_MOVES + increment - Self::TIME_OFFSET
    }
}
