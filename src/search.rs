extern crate rand;

use rand::Rng;
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, scope}, time::Duration};

use crate::{bit_move::{BitMove, ScoringMove}, eval::EvalPosition, move_generation::{Legal, MoveGeneration, PseudoLegal}, pl, position::Position, timer::Timer};

struct PrincipalVariation {
    table: [[BitMove; 246]; 246],
    lengths: [usize; 246],
}

impl PrincipalVariation {
    fn get_pv_string(&self, depth: u8) -> String {
        self.table[0].iter().take(depth as usize).map(|&bit_move| bit_move.to_uci_string()).collect::<Vec<String>>().join(" ")
    }

    fn update(&mut self, ply: usize, bit_move: BitMove) {
        self.table[ply][0] = bit_move;
        for i in 0..self.lengths[ply + 1] {
            self.table[ply][i + 1] = self.table[ply + 1][i];
        }
        self.lengths[ply] = self.lengths[ply + 1] + 1;
    }
}

impl Default for PrincipalVariation {
    fn default() -> Self {
        Self { table: [[BitMove::EMPTY; 246]; 246], lengths: [usize::default(); 246] }
    }
}

pub struct Search {
    timer: Timer,
    stop_time: u128,
    pub stop_calculating: Arc<AtomicBool>,
    nodes: u64,
    pv: PrincipalVariation,
    // TODO: killer_moves, etc...
}

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
        if depth == 0 {
            return EvalPosition::eval(position);
        }

        self.nodes += 1;

        if self.stop_calculating.load(Ordering::Relaxed) {
            return ScoringMove::blank(BLANK);
        }
    
        MoveGeneration::generate_moves::<ScoringMove, Legal>(position)
            .into_iter()
            .map(|mut m: ScoringMove| {
                let mut position_copy = position.clone();
                position_copy.make_move(m.bit_move);
                m.score = -self.minimax_best_move(&position_copy, depth - 1).score;
                m
            })
            .max()
            .unwrap_or_else(|| {
                if position.in_check(position.side) {
                    ScoringMove::blank(-CHECKMATE)
                } else {
                    ScoringMove::blank(DRAW)
                }
            })
    }

    #[inline(always)]
    fn quiescence(&mut self, position: &Position, mut alpha: i16, beta: i16) -> ScoringMove {
        self.nodes += 1;

        if self.stop_calculating.load(Ordering::Relaxed) {
            return ScoringMove::blank(BLANK);
        }
        
        let evaluation = EvalPosition::eval(position);

        if evaluation.score >= beta {
            return ScoringMove::blank(beta);
        } else if evaluation.score > alpha {
            alpha = evaluation.score;
        }

        let mut best_move = ScoringMove::blank(alpha);
        let mut moves = MoveGeneration::generate_captures::<ScoringMove, PseudoLegal>(position);

        #[cfg(feature = "move_sort")]
        moves.sort_by_score();

        for scoring_capture in moves.iter_mut() {
            let mut position_copy = position.clone();
            position_copy.make_move(scoring_capture.bit_move);
            if !position_copy.in_check(position_copy.side.opposite()) {
                scoring_capture.score = -self.quiescence(&position_copy, -beta, -alpha).score;
                if scoring_capture.score > alpha {
                    alpha = scoring_capture.score;
                    best_move = *scoring_capture;
                    if alpha >= beta {
                        return best_move;
                    }
                }
            }
        }

        best_move
    }

    #[inline(always)]
    fn negamax_best_move(&mut self, position: &Position, mut alpha: i16, beta: i16, mut depth: u8, ply: usize) -> ScoringMove {
        self.nodes += 1;
        
        if depth == 0 {
            #[cfg(feature = "no_quiescence")]
            return EvalPosition::eval(position);
            
            #[cfg(feature = "quiescence")]
            return self.quiescence(position, alpha, beta);
        }

        if self.stop_calculating.load(Ordering::Relaxed) {
            return ScoringMove::blank(BLANK);
        }

        let in_check = position.in_check(position.side);

        if in_check { depth += 1; }

        // NOTE: Generating legal moves immediately doesn't seem to cause a
        // drop in performance!
        let mut moves = MoveGeneration::generate_moves::<ScoringMove, PseudoLegal>(position);

        #[cfg(feature = "move_sort")]
        moves.sort_by_score();

        let mut moves_has_legal_move = false;

        let mut best_move = ScoringMove::blank(alpha);
        for scoring_move in moves.iter_mut() {
            let mut position_copy = position.clone();
            position_copy.make_move(scoring_move.bit_move);
            if !position_copy.in_check(position_copy.side.opposite()) {
                moves_has_legal_move = true;
                scoring_move.score = -self.negamax_best_move(&position_copy, -beta, -alpha, depth - 1, ply + 1).score;
                if scoring_move.score > alpha {
                    alpha = scoring_move.score;
                    best_move = *scoring_move;
                    self.pv.update(ply, best_move.bit_move);
                    if alpha >= beta {
                        return best_move;
                    }
                }
            }
        }

        if !moves_has_legal_move {
            if in_check {
                return ScoringMove::blank(-CHECKMATE + ply as i16);
            } else {
                return ScoringMove::blank(DRAW);
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
        return self.negamax_best_move(position, START_ALPHA, START_BETA, depth, 0);
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
        let mut candidate_pv_table = self.pv.table;

        for current_depth in 1..=depth {
            if current_depth != 1 && self.timer.get_time_passed_millis() * AVERAGE_AMOUNT_OF_MOVES > self.stop_time {
                pl!("info string ended iterative search early based on time prediction");
                break
            }
            self.nodes = 0;
            let new_best_move = self.best_move(position, current_depth);
            if self.stop_calculating.load(Ordering::Relaxed) {
                self.pv.table = candidate_pv_table; // Revert to previous PV
                break;
            }

            // Copy the new PV table as it's valid
            candidate_pv_table = self.pv.table;
            best_scoring_move = new_best_move;

            pl!(format!(
                "info depth {} score cp {} nodes {} time {} pv {}",
                current_depth,
                best_scoring_move.score,
                self.nodes,
                self.timer.get_time_passed_millis(),
                self.pv.get_pv_string(current_depth),
            ));
        }
        pl!(format!("bestmove {}", best_scoring_move.bit_move.to_uci_string()));
    }

    #[inline(always)]
    pub fn go(&mut self, position: &Position, depth: u8, total_time: u128, increment: u128) {
        self.reset(total_time, increment);
        let stop_flag = Arc::clone(&self.stop_calculating);
        let stop_time = self.stop_time;
        println!("info string searching for best move within {} milliseconds", self.stop_time);

        // NOTE: Scoping the following thread helps prevent an excess amount of threads being created
        // and future calculations being stopped because of old threads.
        scope(|s| {
            s.spawn(move || {
                for _ in 0..stop_time / 10 {
                    thread::sleep(Duration::from_millis(10));
                    if stop_flag.load(Ordering::Relaxed) {
                        return; // Stop the thread early
                    }
                }
                stop_flag.store(true, Ordering::Relaxed);
            });

            #[cfg(feature = "no_iterative_deepening")]
            self.go_no_iterative_deepening(position, depth);

            #[cfg(feature = "iterative_deepening")]
            self.go_iterative_deepening(position, depth);

            self.stop_calculating.store(true, Ordering::Relaxed);
        });
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
            pv: PrincipalVariation::default(),
        }
    }
}
