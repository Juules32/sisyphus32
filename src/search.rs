extern crate rand;

use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::bit_move::BitMove;
use crate::move_generation::{Legal, PseudoLegal};
use crate::{bit_move::ScoringMove, eval::Eval, move_generation::MoveGeneration, pl, position::Position, timer::Timer};

pub struct Search {
    timer: Timer,
    stop_time: u128,
    pub stop_calculating: Arc<AtomicBool>,
    nodes: u64,
    // TODO: pv, killer_moves, etc...
}

const CHECK_STOP_INTERVAL: u64 = 10000;
const BLANK: i16 = 0;
const CHECKMATE: i16 = 10000;
const DRAW: i16 = 0;
const START_ALPHA: i16 = -32001;
const START_BETA: i16 = 32001;
const AVERAGE_AMOUNT_OF_MOVES: u128 = 30;
const TIME_OFFSET: u128 = 100;

impl Search {
    #[inline(always)]
    fn random_best_move(&self, position: &Position, _depth: u8) -> ScoringMove {
        let moves = MoveGeneration::generate_moves::<BitMove, Legal>(position);
        ScoringMove::from(moves[rand::rng().random_range(0..moves.len())])
    }
    
    #[inline(always)]
    fn minimax_best_move(&mut self, position: &Position, depth: u8) -> ScoringMove {
        self.nodes += 1;

        if self.nodes % CHECK_STOP_INTERVAL == 0 && self.timer.get_time_passed_millis() > self.stop_time {
            self.stop_calculating.store(true, Ordering::Relaxed);
        }

        if self.stop_calculating.load(Ordering::Relaxed) {
            return ScoringMove::blank(BLANK)
        }
        
        if depth == 0 {
            return Eval::eval(position);
        }
    
        MoveGeneration::generate_moves::<ScoringMove, PseudoLegal>(position)
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
                    ScoringMove::blank(-CHECKMATE)
                } else {
                    ScoringMove::blank(DRAW)
                }
            })
    }

    #[inline(always)]
    fn negamax_best_move(&mut self, position: &Position, depth: u8, mut alpha: i16, beta: i16) -> ScoringMove {
        self.nodes += 1;

        if self.nodes % CHECK_STOP_INTERVAL == 0 && self.timer.get_time_passed_millis() > self.stop_time {
            self.stop_calculating.store(true, Ordering::Relaxed);
        }

        if self.stop_calculating.load(Ordering::Relaxed) {
            return ScoringMove::blank(BLANK)
        }
        
        if depth == 0 {
            return Eval::eval(position);
        }

        // NOTE: Generating legal moves immediately doesn't seem to cause a
        // drop in performance!
        let mut moves = MoveGeneration::generate_moves::<ScoringMove, Legal>(position);

        if moves.is_empty() {
            if position.in_check() {
                return ScoringMove::blank(-CHECKMATE);
            } else {
                return ScoringMove::blank(DRAW)
            }
        }

        let mut best_move = ScoringMove::blank(alpha);
        for scoring_move in moves.iter_mut() {
            let mut position_copy = position.clone();
            position_copy.make_move(scoring_move.bit_move);
            scoring_move.score = -self.negamax_best_move(&position_copy, depth - 1, -beta, -alpha).score;
            if scoring_move.score > alpha {
                alpha = scoring_move.score;
                if alpha >= beta {
                    return *scoring_move;
                }
                best_move = *scoring_move;
            }
        }

        best_move
    }

    #[inline(always)]
    fn best_move(&mut self, position: &Position, depth: u8) -> ScoringMove {
        #[cfg(feature = "search_random")]
        return self.random_best_move(position, depth);
        
        #[cfg(feature = "search_minimax")]
        return self.minimax_best_move(position, depth);

        #[cfg(feature = "search_negamax")]
        return self.negamax_best_move(position, depth, START_ALPHA, START_BETA);
    }

    fn reset(&mut self, total_time: u128, increment: u128) {
        self.stop_time = Self::calculate_stop_time(total_time, increment);
        self.nodes = 0;
        self.timer.reset();
        self.stop_calculating.store(false, Ordering::Relaxed);
    }

    #[inline(always)]
    fn go_no_iterative_deepening(&mut self, position: &Position, depth: u8) {
        let best_scoring_move = self.best_move(position, depth);
        pl!(format!("info depth {} score cp {} nodes {} time {} pv {}", depth, best_scoring_move.score, self.nodes, self.timer.get_time_passed_millis(), best_scoring_move.bit_move.to_uci_string()));
        pl!(format!("bestmove {}", best_scoring_move.bit_move.to_uci_string()));
    }

    #[inline(always)]
    fn go_iterative_deepening(&mut self, position: &Position, depth: u8) {
        let mut best_scoring_move = ScoringMove::blank(BLANK);
        for current_depth in 1..=depth {
            if current_depth != 1 && self.timer.get_time_passed_millis() * AVERAGE_AMOUNT_OF_MOVES > self.stop_time {
                pl!("info string ended iterative search early based on time prediction");
                break
            }
            self.nodes = 0;
            let new_best_move = self.best_move(position, current_depth);
            if self.stop_calculating.load(Ordering::Relaxed) {
                break
            }
            best_scoring_move = new_best_move;
            pl!(format!(
                "info depth {} score cp {} nodes {} time {} pv {}",
                current_depth,
                best_scoring_move.score,
                self.nodes,
                self.timer.get_time_passed_millis(),
                best_scoring_move.bit_move.to_uci_string()
            ));
        }
        pl!(format!("bestmove {}", best_scoring_move.bit_move.to_uci_string()));
    }

    #[inline(always)]
    pub fn go(&mut self, position: &Position, depth: u8, total_time: u128, increment: u128) {
        self.reset(total_time, increment);

        println!("info string searching for best move within {} milliseconds", self.stop_time);

        #[cfg(feature = "no_iterative_deepening")]
        self.go_no_iterative_deepening(position, depth);

        #[cfg(feature = "iterative_deepening")]
        self.go_iterative_deepening(position, depth);
    }

    #[inline(always)]
    pub fn calculate_stop_time(total_time: u128, increment: u128) -> u128 {
        total_time / AVERAGE_AMOUNT_OF_MOVES + increment - TIME_OFFSET
    }
}

impl Default for Search {
    fn default() -> Search {
        Search {
            timer: Timer::new(),
            stop_time: 0,
            stop_calculating: Arc::new(AtomicBool::new(false)),
            nodes: 0,
        }
    }
}
