extern crate rand;

use rand::Rng;
use rayon::ThreadPool;
use std::{cmp::max, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread, time::Duration};

use crate::{bit_move::{BitMove, ScoringMove}, butterfly_heuristic::ButterflyHeuristic, eval_position::EvalPosition, killer_moves::KillerMoves, move_generation::{Legal, MoveGeneration, PseudoLegal}, position::Position, syzygy::SyzygyTablebase, timer::Timer, transposition_table::{TTData, TTNodeType, TranspositionTable}, zobrist::ZobristKey};

const BLANK: i16 = 0;
pub const CHECKMATE: i16 = 10000;
pub const TABLEBASE_MOVE: i16 = 20000;
pub const DRAW: i16 = 0;
const DRAW_BY_STALEMATE: i16 = 0;
const DRAW_BY_REPETITION: i16 = 0;
const START_ALPHA: i16 = -32001;
const START_BETA: i16 = 32001;
const AVERAGE_AMOUNT_OF_MOVES: u128 = 25;
pub const MAX_DEPTH: u16 = 64;
const NULL_MOVE_DEPTH_REDUCTION: u16 = 3;

#[cfg(not(feature = "unit_late_move_reductions"))]
const AVERAGE_BRANCHING_FACTOR: u128 = 5;

#[cfg(feature = "unit_late_move_reductions")]
const AVERAGE_BRANCHING_FACTOR: u128 = 2;

#[derive(Clone)]
pub struct Search {
    nodes: u64,
    pub zobrist_key_history: Vec<ZobristKey>,
    timer: Arc<Timer>,
    stop_time: Arc<Option<u128>>,
    stop_calculating: Arc<AtomicBool>,
    threadpool: Arc<ThreadPool>,
    tablebase: Arc<Option<SyzygyTablebase>>,
}

impl Search {
    #[inline(always)]
    pub fn begin_stop_calculating(&self) {
        self.stop_calculating.store(true, Ordering::Relaxed);
    }

    #[inline(always)]
    fn should_stop_calculating(&self) -> bool {
        self.stop_calculating.load(Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn get_stop_calculating(&self) -> Arc<AtomicBool> {
        self.stop_calculating.clone()
    }
    
    #[inline(always)]
    fn random_best_move(&self, position: &Position) -> ScoringMove {
        let moves = MoveGeneration::generate_moves::<BitMove, Legal>(position);
        ScoringMove::from(moves[rand::rng().random_range(0..moves.len())])
    }
    
    #[inline(always)]
    fn minimax_best_move(&mut self, position: &Position, depth: u16) -> ScoringMove {
        if depth == 0 {
            return ScoringMove::blank(EvalPosition::eval(position));
        }

        self.nodes += 1;

        if self.should_stop_calculating() {
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
                    ScoringMove::blank(DRAW_BY_STALEMATE)
                }
            })
    }

    #[inline(always)]
    fn quiescence(&mut self, position: &Position, alpha: i16, beta: i16) -> ScoringMove {
        if self.should_stop_calculating() {
            return ScoringMove::blank(BLANK);
        }
        
        let evaluation = EvalPosition::eval(position);

        let mut best_move = ScoringMove::blank(alpha);

        if evaluation >= beta {
            return ScoringMove::blank(beta);
        } else if evaluation > alpha {
            best_move.score = evaluation;
        }

        let mut moves = MoveGeneration::generate_captures::<ScoringMove, PseudoLegal>(position);

        #[cfg(feature = "unit_sort_moves")]
        moves.sort_by_score();

        for scoring_capture in moves.iter_mut() {
            let mut new_position = position.clone();
            if new_position.apply_pseudo_legal_move(scoring_capture.bit_move) {
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
        self.nodes += 1;

        #[cfg(feature = "unit_syzygy_tablebase")]
        if let Some(tablebase) = self.tablebase.as_ref() {
            if position.ao.count_bits() <= tablebase.get_max_pieces() {
                if let Some(scoring_move) = tablebase.best_move(position) {
                    return scoring_move;
                }
            }
        }

        if self.zobrist_key_history.contains(&position.zobrist_key) {
            return ScoringMove::blank(DRAW_BY_REPETITION);
        }
        
        if depth == 0 {
            #[cfg(not(feature = "unit_quiescence"))]
            return ScoringMove::blank(EvalPosition::eval(position));
            
            #[cfg(feature = "unit_quiescence")]
            return self.quiescence(position, alpha, beta);
        }

        if self.should_stop_calculating() {
            return ScoringMove::blank(BLANK);
        }

        #[cfg(feature = "unit_tt")]
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

        #[cfg(feature = "unit_checks_add_depth")]
        if in_check { depth += 1; }

        #[cfg(feature = "unit_null_move_pruning")]
        if depth > NULL_MOVE_DEPTH_REDUCTION && !in_check && position.ply > 0 {
            let mut position_copy = position.clone();
            position_copy.zobrist_mods();
            position_copy.side.switch();
            position_copy.en_passant_option = None;
            position_copy.zobrist_mods();
            let null_move_score = -self.negamax_best_move(&position_copy, -beta, -beta + 1, depth - NULL_MOVE_DEPTH_REDUCTION).score;
            if null_move_score >= beta {
                return ScoringMove::blank(beta);
            }
        }

        let mut moves = MoveGeneration::generate_moves::<ScoringMove, PseudoLegal>(position);

        #[cfg(feature = "unit_sort_moves")]
        moves.sort_by_score();

        #[cfg(feature = "unit_butterfly_heuristic")]
        let mut quiets_searched: [BitMove; 64] = [BitMove::EMPTY; 64];
        #[cfg(feature = "unit_butterfly_heuristic")]
        let mut quiets_count = 0;

        let mut moves_has_legal_move = false;
        let mut best_move = ScoringMove::blank(alpha);
        self.zobrist_key_history.push(position.zobrist_key);
        let original_depth = depth;
        let mut move_index = 0;
        for scoring_move in moves.iter_mut() {
            let mut new_position = position.clone();
            if new_position.apply_pseudo_legal_move(scoring_move.bit_move) {
                let is_capture_or_promotion = scoring_move.bit_move.is_capture_or_promotion(position);
                moves_has_legal_move = true;

                #[cfg(feature = "unit_late_move_reductions")]
                if !is_capture_or_promotion && original_depth >= 3 && move_index >= 3 {
                    // NOTE: If depth was less than one, the recursive call would underflow depth!
                    // NOTE: Usually, we have to check if the new position is part of the PV, but since
                    // our TT returns exact scores early, this isn't needed.
                    depth = max(1, original_depth - (0.75 * (move_index as f32).ln() * (original_depth as f32).ln()) as u16);
                }

                scoring_move.score = -self.negamax_best_move(&new_position, -beta, -best_move.score, depth - 1).score;
                if scoring_move.score > best_move.score {
                    let mut should_update_best_move = true;

                    #[cfg(feature = "unit_late_move_reductions")]
                    if depth != original_depth && scoring_move.score >= beta {
                        scoring_move.score = -self.negamax_best_move(&new_position, -beta, -best_move.score, original_depth - 1).score;
                        if scoring_move.score <= best_move.score {
                            should_update_best_move = false;
                        }
                    }

                    if should_update_best_move {
                        best_move = *scoring_move;
                        if best_move.score >= beta {
                            if !is_capture_or_promotion {
                                #[cfg(feature = "unit_killer_heuristic")]
                                KillerMoves::update(best_move.bit_move, new_position.ply);
                                
                                #[cfg(feature = "unit_butterfly_heuristic")]
                                ButterflyHeuristic::update(position.side, &quiets_searched[0..quiets_count], best_move.bit_move, depth as i16);
                            }
                            break;
                        }
                    }                    
                }

                #[cfg(feature = "unit_butterfly_heuristic")]
                if scoring_move.bit_move != best_move.bit_move && !is_capture_or_promotion && quiets_count < 64 {
                    quiets_searched[quiets_count] = scoring_move.bit_move;
                    quiets_count += 1;
                }

                move_index += 1;
            }
        }
        self.zobrist_key_history.pop();

        if !moves_has_legal_move {
            if in_check {
                best_move = ScoringMove::blank(-CHECKMATE + position.ply as i16);
            } else {
                best_move = ScoringMove::blank(DRAW_BY_STALEMATE);
            }
        }

        #[cfg(feature = "unit_tt")]
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
                TTData {
                    best_move,
                    depth: original_depth,
                    flag,
                },
            );
        }

        best_move
    }

    #[inline(always)]
    fn best_move(&mut self, position: &Position, depth: u16) -> ScoringMove {
        #[cfg(all(not(feature = "unit_minimax"), not(feature = "unit_negamax")))]
        return self.random_best_move(position);

        #[cfg(feature = "unit_minimax")]
        return self.minimax_best_move(position, depth);

        #[cfg(feature = "unit_negamax")]
        return self.negamax_best_move(position, START_ALPHA, START_BETA, depth);
    }

    fn reset(&mut self, stop_time: Option<u128>) {
        self.stop_time = Arc::new(stop_time);
        self.nodes = 0;
        self.timer = Arc::new(Timer::new());
        self.stop_calculating.store(false, Ordering::Relaxed);
    }

    #[inline(always)]
    fn go_no_iterative_deepening(&mut self, position: &Position, depth: u16) {
        let best_scoring_move = self.best_move(position, depth);
        println!("info depth {} score cp {} nodes {} time {} pv {}", depth, best_scoring_move.score, self.nodes, self.timer.get_time_passed_millis(), best_scoring_move.bit_move.to_uci_string());
        println!("bestmove {}", best_scoring_move.bit_move.to_uci_string());
    }

    #[inline(always)]
    fn should_end_search_early(&self) -> bool {
        if let Some(time) = self.stop_time.as_ref() {
            return self.timer.get_time_passed_millis() * AVERAGE_BRANCHING_FACTOR > *time;
        }
        false
    }

    fn modify_best_scoring_move_if_empty(&self, position: &Position, best_scoring_move: &mut ScoringMove) {
        if best_scoring_move.bit_move == BitMove::EMPTY {
            println!("info string search yielded no move, choosing random move instead");
            *best_scoring_move = self.random_best_move(position);
        }
    }

    #[inline(always)]
    fn go_iterative_deepening(&mut self, position: &Position, depth: u16) {
        let mut best_scoring_move = ScoringMove::blank(BLANK);

        for current_depth in 1..=depth {
            self.nodes = 0;
            let new_best_move = self.best_move(position, current_depth);
            if self.should_stop_calculating() {
                #[cfg(feature = "unit_tt")]
                TranspositionTable::reset();

                println!("info string ended iterative search and reset transposition table");
                break;
            }

            best_scoring_move = new_best_move;
            let found_mate = new_best_move.is_checkmate();
            let found_tablebase_move = new_best_move.is_tablebase_move();

            println!(
                "info depth {} score {} nodes {} time {} pv {}",
                current_depth,
                Self::score_or_mate_string(best_scoring_move.score, found_mate),
                self.nodes,
                self.timer.get_time_passed_millis(),
                self.get_pv(position, current_depth, best_scoring_move.bit_move),
            );

            if found_tablebase_move {
                println!("info string ended iterative search because tablebase move was found");
                break;
            }

            if self.should_end_search_early() {
                println!("info string ended iterative search early based on time prediction");
                break;
            }
        }

        self.modify_best_scoring_move_if_empty(position, &mut best_scoring_move);
        println!("bestmove {}", best_scoring_move.bit_move.to_uci_string());
    }

    #[inline(always)]
    fn go_lazy_smp(&mut self, position: &Position, depth: u16) {
        let best_scoring_move = Arc::new(Mutex::new(ScoringMove::blank(BLANK)));
        let ended_early = Arc::new(AtomicBool::new(false));

        self.threadpool
            .scope(|s| {
            for current_depth in 1..=depth {
                let mut self_ref = self.clone();
                let best_scoring_move = best_scoring_move.clone();
                let ended_early = ended_early.clone();

                s.spawn(move |_| {
                    if self_ref.should_stop_calculating() {
                        return;
                    }

                    let new_best_move = self_ref.best_move(position, current_depth);
                    
                    if self_ref.should_stop_calculating() {
                        return;
                    }

                    // NOTE: This prevents a bug where concurrent threads overwrite an already
                    // existing mating line and also help return the search early if a mate has
                    // already been found.
                    if let Ok(mut best_move) = best_scoring_move.lock() {
                        if !best_move.is_checkmate() {
                            *best_move = new_best_move;
                        } else {
                            return;
                        }
                    }

                    let found_mate = new_best_move.is_checkmate();
                    let found_tablebase_move = new_best_move.is_tablebase_move();
        
                    println!(
                        "info depth {} score {} nodes {} time {} pv {}",
                        current_depth,
                        Self::score_or_mate_string(new_best_move.score, found_mate),
                        self_ref.nodes,
                        self_ref.timer.get_time_passed_millis(),
                        self_ref.get_pv(position, current_depth, new_best_move.bit_move),
                    );

                    if found_tablebase_move {
                        println!("info string ended iterative search because tablebase move was found");
                        self_ref.begin_stop_calculating();
                        return;
                    }

                    if self_ref.should_end_search_early() {
                        self_ref.begin_stop_calculating();
                        ended_early.store(true, Ordering::Relaxed);
                        return;
                    }
                });
            }
        });

        if self.should_stop_calculating() {
            print!("info string ended iterative search and reset transposition table");
            if ended_early.load(Ordering::Relaxed) {
                println!(" based on time prediction");
            } else {
                println!();
            }
            TranspositionTable::reset();
        }

        self.modify_best_scoring_move_if_empty(position, &mut best_scoring_move.lock().unwrap());
        println!("bestmove {}", best_scoring_move.lock().unwrap().bit_move.to_uci_string());
    }

    fn score_or_mate_string(score: i16, found_mate: bool) -> String {
        if found_mate {
            format!("mate {}", ((CHECKMATE - score.abs()) as f32 / 2.0).ceil() as i16 * score.signum())
        } else {
            format!("cp {score}")
        }
    }

    #[inline(always)]
    pub fn go(&mut self, position: &Position, depth: Option<u16>, stop_time: Option<u128>) {
        self.reset(stop_time);
        let stop_flag = self.stop_calculating.clone();
        print!("info string searching for best move");

        if let Some(stop_time) = stop_time {
            print!(" within {} milliseconds", stop_time);
        }

        if let Some(depth) = depth {
            print!(" with a maximum depth of {}", depth);
        }

        println!();

        let depth = depth.unwrap_or(255);

        // NOTE: Scoping the following thread helps prevent an excess amount of threads being created
        // and future calculations being stopped because of old threads.
        thread::scope(|s| {
            if let Some(time) = stop_time {
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
            
            #[cfg(not(feature = "unit_iterative_deepening"))]
            self.go_no_iterative_deepening(position, depth);

            #[cfg(all(not(feature = "unit_lazy_smp"), feature = "unit_iterative_deepening"))]
            self.go_iterative_deepening(position, depth);

            #[cfg(feature = "unit_lazy_smp")]
            if self.threadpool.current_num_threads() >= 3 {
                self.go_lazy_smp(position, depth);
            } else {
                self.go_iterative_deepening(position, depth);
            }

            self.begin_stop_calculating();
        });
    }

    #[inline(always)]
    pub fn calculate_stop_time(total_time: Option<u128>, increment_time: Option<u128>) -> Option<u128> {
        total_time.map(|total_time| total_time / AVERAGE_AMOUNT_OF_MOVES + increment_time.unwrap_or(0))
    }

    fn get_pv(&self, position: &Position, depth: u16, _best_move: BitMove) -> String {
        #[cfg(feature = "unit_tt")]
        return self.get_pv_from_tt(position, depth);

        #[cfg(not(feature = "unit_tt"))]
        return _best_move.to_uci_string()
    }

    // NOTE: There is a notable chance the pv will be ended early in case a different position
    // happens to have the same table index. The probability scales inversely with the
    // size of the transposition table.
    #[cfg(feature = "unit_tt")]
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

    pub fn set_threadpool(&mut self, num_threads: usize) {
        self.threadpool = Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .unwrap()
        );
    }

    pub fn set_tablebase(&mut self, path: &str) {
        self.tablebase = Arc::new(Some(SyzygyTablebase::from_directory(path).unwrap()));
    }
}

impl Default for Search {
    fn default() -> Search {
        Search {
            timer: Arc::new(Timer::new()),
            stop_time: Arc::new(None),
            stop_calculating: Arc::new(AtomicBool::new(false)),
            nodes: 0,
            zobrist_key_history: Vec::new(),
            threadpool: Arc::new(
                rayon::ThreadPoolBuilder::new()
                    .num_threads(rayon::current_num_threads()) // NOTE: Defaults to the number of CPU cores
                    .build()
                    .unwrap()
            ),
            tablebase: Arc::new(SyzygyTablebase::from_directory("tables/syzygy").ok()),
        }
    }
}
