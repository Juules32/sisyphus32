extern crate rand;

use rand::Rng;
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, scope}, time::Duration};

use crate::{bit_move::{BitMove, ScoringMove}, butterfly_heuristic::ButterflyHeuristic, eval::EvalPosition, killer_moves::KillerMoves, move_generation::{Legal, MoveGeneration, PseudoLegal}, position::Position, timer::Timer, transposition_table::{TTEntry, TTNodeType, TranspositionTable}};

pub struct Search {
    timer: Timer,
    stop_time: Option<u128>,
    pub stop_calculating: Arc<AtomicBool>,
    nodes: u64,
    // TODO: killer_moves, etc...
}

const BLANK: i16 = 0;
const CHECKMATE: i16 = 10000;
const DRAW: i16 = 0;
const START_ALPHA: i16 = -32001;
const START_BETA: i16 = 32001;
const AVERAGE_AMOUNT_OF_MOVES: u128 = 30;
const AVERAGE_BRANCHING_FACTOR: u128 = 5;
const MAX_PLY: i16 = 242;

impl Search {
    #[inline(always)]
    fn random_best_move(&self, position: &Position, _depth: u16) -> ScoringMove {
        let moves = MoveGeneration::generate_moves::<BitMove, Legal>(position);
        ScoringMove::from(moves[rand::rng().random_range(0..moves.len())])
    }
    
    #[inline(always)]
    fn minimax_best_move(&mut self, position: &Position, depth: u16) -> ScoringMove {
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
    fn quiescence(&mut self, position: &Position, alpha: i16, beta: i16) -> ScoringMove {
        if self.stop_calculating.load(Ordering::Relaxed) {
            return ScoringMove::blank(BLANK);
        }
        
        let evaluation = EvalPosition::eval(position);
        let mut best_move = ScoringMove::blank(alpha);

        if evaluation.score >= beta {
            return ScoringMove::blank(beta);
        } else if evaluation.score > alpha {
            best_move.score = evaluation.score;
        }

        let mut moves = MoveGeneration::generate_captures::<ScoringMove, PseudoLegal>(position);

        #[cfg(feature = "move_sort")]
        moves.sort_by_score();

        for scoring_capture in moves.iter_mut() {
            if let Some(new_position) = position.apply_pseudo_legal_move(scoring_capture.bit_move) {
                self.nodes += 1;
                scoring_capture.score = -self.quiescence(&new_position, -beta, -best_move.score).score;
                if scoring_capture.score > best_move.score {
                    best_move = *scoring_capture;
                    if best_move.score >= beta {
                        return best_move;
                    }
                }
            }
        }

        best_move
    }

    #[inline(always)]
    fn negamax_best_move(&mut self, position: &Position, alpha: i16, beta: i16, mut depth: u16) -> ScoringMove {
        #[cfg(feature = "butterfly_heuristic")]
        let mut quiets_searched: [BitMove; 64] = [BitMove::EMPTY; 64];
        #[cfg(feature = "butterfly_heuristic")]
        let mut quiets_count = 0;

        self.nodes += 1;
        
        if depth == 0 {
            #[cfg(not(feature = "quiescence"))]
            return EvalPosition::eval(position);
            
            #[cfg(feature = "quiescence")]
            return self.quiescence(position, alpha, beta);
        }

        if self.stop_calculating.load(Ordering::Relaxed) {
            return ScoringMove::blank(BLANK);
        }

        #[cfg(feature = "transposition_table")]
        if let Some(tt_entry) = TranspositionTable::probe(position.zobrist_key) {
            // If the stored depth is at least as deep, use it
            if tt_entry.depth >= depth {
                match tt_entry.flag {
                    TTNodeType::Exact => return tt_entry.best_move,
                    TTNodeType::LowerBound => {
                        if tt_entry.best_move.score >= beta {
                            return tt_entry.best_move;
                        }
                    },
                    TTNodeType::UpperBound => {
                        if tt_entry.best_move.score <= alpha {
                            return tt_entry.best_move;
                        }
                    },
                }
            }
        }

        let in_check = position.in_check(position.side);

        #[cfg(feature = "checks_add_depth")]
        if in_check { depth += 1; }

        let mut moves = MoveGeneration::generate_moves::<ScoringMove, PseudoLegal>(position);

        #[cfg(feature = "move_sort")]
        moves.sort_by_score();

        let mut moves_has_legal_move = false;
        let mut best_move = ScoringMove::blank(alpha);
        for scoring_move in moves.iter_mut() {
            if let Some(new_position) = position.apply_pseudo_legal_move(scoring_move.bit_move) {
                let is_capture_or_promotion = scoring_move.bit_move.is_capture_or_promotion(position);
                moves_has_legal_move = true;
                scoring_move.score = -self.negamax_best_move(&new_position, -beta, -best_move.score, depth - 1).score;
                if scoring_move.score > best_move.score {
                    best_move = *scoring_move;
                    if best_move.score >= beta {
                        if !is_capture_or_promotion {
                            #[cfg(feature = "killer_moves")]
                            KillerMoves::update(best_move.bit_move, new_position.ply);
                            
                            #[cfg(feature = "butterfly_heuristic")]
                            ButterflyHeuristic::update(position.side, &quiets_searched[0..quiets_count], best_move.bit_move, depth as i16);
                        }

                        break;
                    }
                }

                #[cfg(feature = "butterfly_heuristic")]
                if scoring_move.bit_move != best_move.bit_move && !is_capture_or_promotion && quiets_count < 64 {
                    quiets_searched[quiets_count] = scoring_move.bit_move;
                    quiets_count += 1;
                }
            }
        }

        if !moves_has_legal_move {
            if in_check {
                best_move = ScoringMove::blank(-CHECKMATE + position.ply as i16);
            } else {
                best_move = ScoringMove::blank(DRAW);
            }
        }

        #[cfg(feature = "transposition_table")]
        {
            let flag = if best_move.score >= beta {
                TTNodeType::LowerBound
            } else if best_move.score <= alpha {
                TTNodeType::UpperBound
            } else {
                TTNodeType::Exact
            };
    
            TranspositionTable::store(
                position.zobrist_key,
                TTEntry {
                    zobrist_key: position.zobrist_key,
                    best_move,
                    depth,
                    flag,
                },
            );
        }

        best_move
    }

    #[inline(always)]
    fn best_move(&mut self, position: &Position, depth: u16) -> ScoringMove {
        #[cfg(feature = "search_random")]
        return self.random_best_move(position, depth);
        
        #[cfg(feature = "search_minimax")]
        return self.minimax_best_move(position, depth);

        #[cfg(feature = "search_negamax")]
        return self.negamax_best_move(position, START_ALPHA, START_BETA, depth);
    }

    fn reset(&mut self, stop_time: Option<u128>) {
        self.stop_time = stop_time;
        self.nodes = 0;
        self.timer.reset();
        self.stop_calculating.store(false, Ordering::Relaxed);
    }

    #[inline(always)]
    fn go_no_iterative_deepening(&mut self, position: &Position, depth: u16) {
        let best_scoring_move = self.best_move(position, depth);
        println!("info depth {} score cp {} nodes {} time {} pv {}", depth, best_scoring_move.score, self.nodes, self.timer.get_time_passed_millis(), best_scoring_move.bit_move.to_uci_string());
        println!("bestmove {}", best_scoring_move.bit_move.to_uci_string());
    }

    #[inline(always)]
    fn go_iterative_deepening(&mut self, position: &Position, depth: u16) {
        let mut best_scoring_move = ScoringMove::blank(BLANK);

        for current_depth in 1..=depth {
            if let Some(time) = self.stop_time {
                if current_depth != 1 && self.timer.get_time_passed_millis() * AVERAGE_BRANCHING_FACTOR > time {
                    println!("info string ended iterative search early based on time prediction");
                    break;
                }
            }
            self.nodes = 0;
            let new_best_move = self.best_move(position, current_depth);
            if self.stop_calculating.load(Ordering::Relaxed) {
                #[cfg(feature = "transposition_table")]
                TranspositionTable::reset();

                println!("info string ended iterative search and reset transposition table");
                break;
            }

            best_scoring_move = new_best_move;
            let found_mate = best_scoring_move.score.abs() > CHECKMATE - MAX_PLY;

            println!(
                "info depth {} score {} nodes {} time {} pv {}",
                current_depth,
                Self::score_or_mate_string(best_scoring_move.score, found_mate),
                self.nodes,
                self.timer.get_time_passed_millis(),
                self.get_pv(position, current_depth, best_scoring_move.bit_move),
            );

            if found_mate {
                println!("info string ended iterative search because mating line was found");
                break;
            }
        }
        println!("bestmove {}", best_scoring_move.bit_move.to_uci_string());
    }

    fn score_or_mate_string(score: i16, found_mate: bool) -> String {
        if found_mate {
            format!("mate {}", ((CHECKMATE - score.abs()) as f32 / 2.0).ceil() as i16 * score.signum())
        } else {
            format!("cp {score}")
        }
    }

    #[inline(always)]
    pub fn go(&mut self, position: &Position, depth: u16, stop_time: Option<u128>) {
        self.reset(stop_time);
        let stop_flag = Arc::clone(&self.stop_calculating);
        print!("info string searching for best move");
        match self.stop_time {
            Some(time) => println!(" within {} milliseconds", time),
            None => println!(),
        }

        // NOTE: Scoping the following thread helps prevent an excess amount of threads being created
        // and future calculations being stopped because of old threads.
        scope(|s| {
            if let Some(time) = self.stop_time {
                s.spawn(move || {
                    for _ in 0..time / 10 {
                        thread::sleep(Duration::from_millis(10));
                        if stop_flag.load(Ordering::Relaxed) {
                            return; // Stop the thread early
                        }
                    }
                    stop_flag.store(true, Ordering::Relaxed);
                });
            }
            
            #[cfg(not(feature = "iterative_deepening"))]
            self.go_no_iterative_deepening(position, depth);

            #[cfg(feature = "iterative_deepening")]
            self.go_iterative_deepening(position, depth);

            self.stop_calculating.store(true, Ordering::Relaxed);
        });
    }

    #[inline(always)]
    pub fn calculate_stop_time(total_time: Option<u128>, increment: u128) -> Option<u128> {
        total_time.map(|time| time / AVERAGE_AMOUNT_OF_MOVES + increment)
    }

    fn get_pv(&self, position: &Position, depth: u16, _best_move: BitMove) -> String {
        #[cfg(feature = "transposition_table")]
        return self.get_pv_from_tt(position, depth);

        #[cfg(not(feature = "transposition_table"))]
        return _best_move.to_uci_string()
    }

    // NOTE: There is a notable chance the pv will be ended early in case a different position
    // happens to have the same table index. The probability scales inversely with the
    // size of the transposition table.
    #[cfg(feature = "transposition_table")]
    fn get_pv_from_tt(&self, position: &Position, depth: u16) -> String {
        let mut pv_moves = Vec::new();
        let mut position_copy = position.clone();
        for _ in 0..depth {
            if let Some(tt_entry) = TranspositionTable::probe(position_copy.zobrist_key) {
                let best_move = tt_entry.best_move;
                if best_move.bit_move == BitMove::EMPTY {
                    break;
                }
                pv_moves.push(best_move.bit_move.to_uci_string());
                position_copy.make_move(best_move.bit_move);
            }
        }
        pv_moves.join(" ")
    }
}

impl Default for Search {
    fn default() -> Search {
        Search {
            timer: Timer::new(),
            stop_time: None,
            stop_calculating: Arc::new(AtomicBool::new(false)),
            nodes: 0,
        }
    }
}
