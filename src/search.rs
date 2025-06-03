use std::{cmp::min, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}};

use crate::{BitMove, EvalPosition, HistoryHeuristic, KillerMoves, Legal, MoveGeneration, Position, PseudoLegal, Score, ScoringMove, TTData, TTNodeType, Timer, TranspositionTable, ZobristKey, MAX_DEPTH, SQUARE_COUNT};

const AVERAGE_AMOUNT_OF_MOVES: usize = 25;
const NULL_MOVE_DEPTH_REDUCTION: usize = 3;
const LMR_MOVE_INDEX_THRESHOLD: usize = 3;
const TABLEBASE_SEARCH_THRESHOLD: u128 = 100;
const EXTENDED_TABLEBASE_SEARCH_THRESHOLD: u128 = 500;
const OPENING_BOOK_SEARCH_THRESHOLD: u128 = 100;
const LMR_DEPTH_THRESHOLD: usize = 3;
const LMR_FACTOR: f32 = 0.75;
const NUM_NODE_CHECK: u64 = 10000;

#[cfg(not(feature = "late_move_reductions"))]
const AVERAGE_BRANCHING_FACTOR: usize = 5;

#[cfg(feature = "late_move_reductions")]
const AVERAGE_BRANCHING_FACTOR: usize = 2;

#[derive(Clone)]
pub struct Search {
    nodes: u64,
    pub(crate) zobrist_key_history: Vec<ZobristKey>,
    timer: Arc<Timer>,
    stop_time: Arc<Option<u128>>,
    stop_calculating: Arc<AtomicBool>,
    pub(crate) in_opening: bool,
    
    #[cfg(feature = "opening_book")]
    opening_book: Arc<crate::OpeningBook>,

    #[cfg(feature = "syzygy_tablebase")]
    tablebase: Arc<Option<crate::SyzygyTablebase>>,
    
    uci_visible: bool,
}

impl Default for Search {
    fn default() -> Search {
        Search {
            timer: Arc::new(Timer::new()),
            stop_time: Arc::new(None),
            stop_calculating: Arc::new(AtomicBool::new(false)),
            nodes: 0,
            zobrist_key_history: Vec::new(),
            in_opening: true,
            
            #[cfg(feature = "opening_book")]
            opening_book: Arc::new(crate::OpeningBook::default()),
            
            #[cfg(feature = "syzygy_tablebase")]
            tablebase: Arc::new(crate::SyzygyTablebase::from_directory("tables/syzygy").ok()),

            uci_visible: false,
        }
    }
}

#[macro_export]
macro_rules! uci_println {
    ($self:expr) => {
        if $self.uci_visible {
            println!();
        }
    };

    ($self:expr, $($arg:tt)*) => {
        if $self.uci_visible {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! uci_print {
    ($self:expr) => {
        if $self.uci_visible {
            print!();
        }
    };

    ($self:expr, $($arg:tt)*) => {
        if $self.uci_visible {
            print!($($arg)*);
        }
    };
}

impl Search {
    #[inline(always)]
    pub fn show_uci_info(&mut self) {
        self.uci_visible = true;
    }

    #[inline(always)]
    pub fn hide_uci_info(&mut self) {
        self.uci_visible = false;
    }

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
    fn move_ordering_best_move(&self, position: &Position) -> ScoringMove {
        let mut moves = MoveGeneration::generate_moves::<ScoringMove, Legal>(position);
        moves.sort_by_score();
        moves.first()
    }
    
    #[inline(always)]
    fn minimax_best_move(&mut self, position: &Position, depth: usize) -> ScoringMove {
        if depth == 0 {
            return ScoringMove::blank(EvalPosition::eval(position));
        }

        self.nodes += 1;

        if self.should_stop_calculating() {
            return ScoringMove::blank(Score::BLANK);
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
                    ScoringMove::blank(-Score::CHECKMATE)
                } else {
                    ScoringMove::blank(Score::STALEMATE)
                }
            })
    }

    #[inline(always)]
    fn perform_stop_calculating_check(&self) {
        if let Some(stop_time) = *self.stop_time {
            if self.nodes % NUM_NODE_CHECK == 0 {
                if self.timer.get_time_passed_millis() >= stop_time {
                    self.begin_stop_calculating();
                }
            }
        }
    }

    #[inline(always)]
    fn quiescence(&mut self, position: &Position, mut alpha: Score, beta: Score) -> ScoringMove {
        self.perform_stop_calculating_check();
        if self.should_stop_calculating() {
            return ScoringMove::blank(Score::BLANK);
        }
        
        let evaluation = EvalPosition::eval(position);

        if evaluation >= beta {
            return ScoringMove::blank(beta);
        } else if evaluation > alpha {
            alpha = evaluation;
        }

        let mut best_move = ScoringMove::blank(alpha);
        let mut moves = MoveGeneration::generate_captures::<ScoringMove, PseudoLegal>(position);

        #[cfg(feature = "sort_moves")]
        moves.sort_by_score();

        for scoring_capture in moves.iter_mut() {
            let mut new_position = position.clone();
            if new_position.apply_pseudo_legal_move(scoring_capture.bit_move) {
                self.nodes += 1;
                scoring_capture.score = -self.quiescence(&new_position, -beta, -alpha).score;
                if scoring_capture.score > alpha {
                    alpha = scoring_capture.score;
                    best_move = *scoring_capture;
                    if alpha >= beta {
                        break;
                    }
                }
            }
        }

        best_move
    }

    #[inline(always)]
    fn negamax_best_move(&mut self, position: &Position, mut alpha: Score, mut beta: Score, mut depth: usize) -> ScoringMove {
        self.nodes += 1;

        if self.zobrist_key_history.contains(&position.zobrist_key) {
            return ScoringMove::blank(Score::REPETITION);
        }
        
        if depth == 0 {
            #[cfg(not(feature = "quiescence"))]
            return ScoringMove::blank(EvalPosition::eval(position));
            
            #[cfg(feature = "quiescence")]
            return self.quiescence(position, alpha, beta);
        }

        self.perform_stop_calculating_check();
        if self.should_stop_calculating() {
            return ScoringMove::blank(Score::BLANK);
        }

        #[cfg(feature = "tt")]
        if let Some(tt_entry) = TranspositionTable::probe(position.zobrist_key) {
            // If the stored depth is at least as deep, use it
            if tt_entry.depth >= depth as u16 {
                match tt_entry.node_type {
                    TTNodeType::Exact => return tt_entry.best_move,
                    TTNodeType::LowerBound => {
                        if tt_entry.best_move.score > alpha {
                            alpha = tt_entry.best_move.score;
                            if alpha >= beta {
                                return tt_entry.best_move;
                            }
                        }
                    },
                    TTNodeType::UpperBound => {
                        if tt_entry.best_move.score < beta {
                            beta = tt_entry.best_move.score;
                            if alpha >= beta {
                                return tt_entry.best_move;
                            }
                        }
                    },
                }
            }
        }

        let in_check = position.in_check(position.side);

        #[cfg(feature = "checks_add_depth")]
        if in_check { depth += 1; }

        #[cfg(feature = "null_move_pruning")]
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

        #[cfg(feature = "sort_moves")]
        moves.sort_by_score();

        #[cfg(feature = "history_heuristic")]
        let mut quiets_searched: [BitMove; SQUARE_COUNT] = [BitMove::EMPTY; SQUARE_COUNT];
        #[cfg(feature = "history_heuristic")]
        let mut quiets_count = 0;

        let mut moves_has_legal_move = false;
        let mut best_move = ScoringMove::blank(alpha);
        self.zobrist_key_history.push(position.zobrist_key);
        let mut move_index = 0;
        for mut scoring_move in moves {
            let mut new_position = position.clone();
            if new_position.apply_pseudo_legal_move(scoring_move.bit_move) {
                let is_capture_or_promotion = scoring_move.bit_move.is_capture_or_promotion(position);
                moves_has_legal_move = true;

                #[cfg(feature = "late_move_reductions")]
                let mut reduced_depth = depth;

                #[cfg(feature = "late_move_reductions")]
                if !is_capture_or_promotion && depth >= LMR_DEPTH_THRESHOLD && move_index >= LMR_MOVE_INDEX_THRESHOLD {
                    // NOTE: If depth was less than zero, the depth would underflow!
                    // NOTE: Usually, we have to check if the new position is part of the PV, but since
                    // our TT returns exact scores early, this isn't needed.
                    reduced_depth = depth - min(depth, (LMR_FACTOR * (move_index as f32).ln() * (depth as f32).ln()) as usize);
                    scoring_move.score = -self.negamax_best_move(&new_position, -beta, -alpha, reduced_depth).score;
                } else {
                    scoring_move.score = -self.negamax_best_move(&new_position, -beta, -alpha, depth - 1).score;
                }

                #[cfg(not(feature = "late_move_reductions"))]
                {
                    scoring_move.score = -self.negamax_best_move(&new_position, -beta, -alpha, depth - 1).score;
                }

                if scoring_move.score.is_checkmate() {
                    scoring_move.score -= scoring_move.score.signum();
                }

                if scoring_move.score > alpha {
                    let mut should_update_alpha = true;

                    #[cfg(feature = "late_move_reductions")]
                    // If a search reduced in depth by lmr is an alpha-cutoff
                    if reduced_depth != depth && scoring_move.score >= beta {
                        // Search again at full depth
                        scoring_move.score = -self.negamax_best_move(&new_position, -beta, -alpha, depth - 1).score;
                        if scoring_move.score.is_checkmate() {
                            scoring_move.score -= scoring_move.score.signum();
                        }
                        
                        // And don't update alpha if the search at full depth actually wasn't an alpha-cutoff
                        if scoring_move.score <= alpha {
                            should_update_alpha = false;
                        }
                    }

                    if should_update_alpha {
                        alpha = scoring_move.score;
                        best_move = scoring_move;
                        if alpha >= beta {
                            if !is_capture_or_promotion {
                                #[cfg(feature = "killer_heuristic")]
                                KillerMoves::update(scoring_move.bit_move, new_position.ply);
                                
                                #[cfg(feature = "history_heuristic")]
                                HistoryHeuristic::update(position.side, &quiets_searched[0..quiets_count], scoring_move.bit_move, depth as i16);
                            }
                            break;
                        }
                    }
                }

                #[cfg(feature = "history_heuristic")]
                if scoring_move.bit_move != best_move.bit_move && !is_capture_or_promotion && quiets_count < SQUARE_COUNT {
                    quiets_searched[quiets_count] = scoring_move.bit_move;
                    quiets_count += 1;
                }

                move_index += 1;
            }
        }
        self.zobrist_key_history.pop();

        if !moves_has_legal_move {
            if in_check {
                best_move = ScoringMove::blank(-Score::CHECKMATE);
            } else {
                best_move = ScoringMove::blank(Score::STALEMATE);
            }
        }

        #[cfg(feature = "tt")]
        {
            let node_type = if best_move.score >= beta {
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
                    depth: depth as u16,
                    node_type,
                },
            );
        }

        best_move
    }

    #[inline(always)]
    fn best_move(&mut self, position: &Position, depth: usize) -> ScoringMove {
        #[cfg(all(not(feature = "minimax"), not(feature = "negamax")))]
        return self.move_ordering_best_move(position);

        #[cfg(feature = "minimax")]
        return self.minimax_best_move(position, depth);

        #[cfg(feature = "negamax")]
        return self.negamax_best_move(position, Score::START_ALPHA, Score::START_BETA, depth);
    }

    fn reset(&mut self, stop_time: Option<u128>) {
        self.stop_time = Arc::new(stop_time);
        self.nodes = 0;
        self.timer = Arc::new(Timer::new());
        self.stop_calculating.store(false, Ordering::Relaxed);
    }

    #[inline(always)]
    fn go_no_iterative_deepening(&mut self, position: &Position, depth: usize) -> ScoringMove {
        let best_move = self.best_move(position, depth);
        uci_println!(self, "info depth {} score cp {} nodes {} time {} pv {}", depth, best_move.score, self.nodes, self.timer.get_time_passed_millis(), best_move.bit_move.to_uci_string());
        best_move
    }

    #[inline(always)]
    fn should_end_search_early(&self) -> bool {
        if let Some(time) = self.stop_time.as_ref() {
            return self.timer.get_time_passed_millis() * AVERAGE_BRANCHING_FACTOR as u128 > *time;
        }
        false
    }

    fn modify_best_move_if_empty(&self, position: &Position, best_move: &mut ScoringMove) {
        if best_move.bit_move == BitMove::EMPTY {
            uci_println!(self, "info string choosing best move based on move ordering");
            *best_move = self.move_ordering_best_move(position);
        }
    }

    #[inline(always)]
    fn go_iterative_deepening(&mut self, position: &Position, depth: usize) -> ScoringMove {
        let mut best_move = ScoringMove::blank(Score::BLANK);

        for current_depth in 1..=depth {
            self.nodes = 0;
            let new_best_move = self.best_move(position, current_depth);

            if self.should_stop_calculating() {
                #[cfg(feature = "tt")]
                TranspositionTable::reset();

                uci_println!(self, "info string ended iterative search and reset transposition table");
                break;
            }

            // NOTE: This check is necessary to mitigate the effects of a rare
            // bug where an empty bitmove is returned from the search!
            if new_best_move.bit_move == BitMove::EMPTY {
                uci_println!(self, "info string found empty best move at depth {current_depth}");
                continue;
            }

            best_move = new_best_move;
            let found_mate = new_best_move.score.is_checkmate();

            self.print_info_depth(position, new_best_move, current_depth, found_mate);

            if self.should_end_search_early() {
                uci_println!(self, "info string ended iterative search early based on time prediction");
                break;
            }
        }

        self.modify_best_move_if_empty(position, &mut best_move);
        best_move
    }

    #[inline(always)]
    fn print_info_depth(&self, position: &Position, scoring_move: ScoringMove, current_depth: usize, found_mate: bool) {
        uci_println!(self, 
            "info depth {:<2} score {:<10} nodes {:<10} time {:<6} pv {}",
            current_depth,
            Self::score_or_mate_string(scoring_move.score, found_mate),
            self.nodes,
            self.timer.get_time_passed_millis(),
            self.get_pv(position, current_depth, scoring_move.bit_move),
        );
    }

    #[inline(always)]
    #[cfg(feature = "parallelize")]
    fn go_lazy_smp(&mut self, position: &Position, depth: usize) -> ScoringMove {
        let best_move = Arc::new(Mutex::new(ScoringMove::blank(Score::BLANK)));
        let ended_early = Arc::new(AtomicBool::new(false));

        crate::GlobalThreadPool::get()
            .scope(|s| {
            for current_depth in 1..=depth {
                let mut self_ref = self.clone();
                let best_move = best_move.clone();
                let ended_early = ended_early.clone();

                s.spawn(move |_| {
                    if self_ref.should_stop_calculating() {
                        return;
                    }

                    let new_best_move = self_ref.best_move(position, current_depth);
                    
                    if self_ref.should_stop_calculating() {
                        return;
                    }
                    
                    if new_best_move.bit_move == BitMove::EMPTY {
                        return;
                    }

                    // NOTE: This prevents a bug where concurrent threads overwrite an already
                    // existing mating line and also help return the search early if a mate has
                    // already been found.
                    if let Ok(mut best_move) = best_move.lock() {
                        if !best_move.score.is_checkmate() {
                            *best_move = new_best_move;
                        } else {
                            return;
                        }
                    }

                    let found_mate = new_best_move.score.is_checkmate();
        
                    self_ref.print_info_depth(position, new_best_move, current_depth, found_mate);

                    if self_ref.should_end_search_early() {
                        self_ref.begin_stop_calculating();
                        ended_early.store(true, Ordering::Relaxed);
                    }
                });
            }
        });

        if self.should_stop_calculating() {
            uci_print!(self, "info string ended iterative search and reset transposition table");
            if ended_early.load(Ordering::Relaxed) {
                uci_println!(self, " based on time prediction");
            } else {
                uci_println!(self, );
            }
            TranspositionTable::reset();
        }

        self.modify_best_move_if_empty(position, &mut best_move.lock().unwrap());
        let best_move = best_move.lock().unwrap();
        *best_move
    }

    fn score_or_mate_string(score: Score, found_mate: bool) -> String {
        if found_mate {
            // format!("mate {}", ((f32::from(Score::CHECKMATE - score.abs())) / 2.0).ceil() as i16 * i16::from(score.signum()))
            format!("cp {score}")
        } else {
            format!("cp {score}")
        }
    }

    #[inline(always)]
    pub fn go(&mut self, position: &Position, depth: Option<usize>, stop_time: Option<u128>) -> ScoringMove {
        self.reset(stop_time);

        #[cfg(feature = "opening_book")]
        if self.in_opening && stop_time.is_none_or(|time| time >= OPENING_BOOK_SEARCH_THRESHOLD) {
            uci_println!(self, "info string searching for opening move");
            if let Some(opening_move) = self.opening_book.get_move(position) {
                uci_println!(self, "info time {}", self.timer.get_time_passed_millis());
                uci_println!(self, "bestmove {}", opening_move.to_uci_string());
                return ScoringMove::from(opening_move);
            } else {
                uci_println!(self, "info string error finding opening move");
                uci_println!(self, "info string disabling opening book");
                self.in_opening = false;
            }
        }

        #[cfg(feature = "syzygy_tablebase")]
        if let Some(tablebase) = self.tablebase.as_ref() {
            let tablebase_max_pieces_u8 = tablebase.get_max_pieces() as u8;
            if position.all_occupancy.count_bits() <= tablebase_max_pieces_u8 + 1 {
                uci_println!(self, "info string searching for tablebase move");
                let mut best_move_option = None;

                if position.all_occupancy.count_bits() <= tablebase_max_pieces_u8 && stop_time.is_none_or(|time| time >= TABLEBASE_SEARCH_THRESHOLD) {
                    if let Some(tablebase_move) = tablebase.best_move(position) {
                        best_move_option = Some(tablebase_move);
                    }
                } else if position.all_occupancy.count_bits() == tablebase_max_pieces_u8 + 1 && stop_time.is_none_or(|time| time >= EXTENDED_TABLEBASE_SEARCH_THRESHOLD) {
                    let moves = MoveGeneration::generate_captures::<ScoringMove, PseudoLegal>(position);
                    for scoring_move in moves {
                        let mut new_position = position.clone();
                        if new_position.apply_pseudo_legal_move(scoring_move.bit_move) {
                            if let Some(tablebase_move) = tablebase.best_move(&new_position) {
                                if tablebase_move.score.is_negative() {
                                    best_move_option = Some(ScoringMove::new(scoring_move.bit_move, Score::CHECKMATE));
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }

                if let Some(best_move) = best_move_option {
                    uci_println!(self, 
                        "info score {} time {}",
                        Self::score_or_mate_string(best_move.score, true),
                        self.timer.get_time_passed_millis(),
                    );
                    uci_println!(self, "bestmove {}", best_move.bit_move.to_uci_string());
                    return best_move;
                } else {
                    uci_println!(self, "info string error finding tablebase move");
                }
            }
        }

        let best_move = self.go_search(position, depth, stop_time);
        uci_println!(self, "bestmove {}", best_move.bit_move.to_uci_string());
        best_move
    }

    #[inline(always)]
    fn go_search(&mut self, position: &Position, depth: Option<usize>, stop_time: Option<u128>) -> ScoringMove {
        uci_print!(self, "info string searching for best move");

        if let Some(stop_time) = stop_time {
            uci_print!(self, " within {stop_time} milliseconds");
        }

        if let Some(depth) = depth {
            uci_print!(self, " with a maximum depth of {depth}");
        }

        uci_println!(self);

        let depth = depth.unwrap_or(MAX_DEPTH);

        #[cfg(not(feature = "iterative_deepening"))]
        { return self.go_no_iterative_deepening(position, depth); }

        #[cfg(not(feature = "parallelize"))]
        { return self.go_iterative_deepening(position, depth); }

        #[cfg(feature = "parallelize")]
        if crate::GlobalThreadPool::should_parallelize() {
            self.go_lazy_smp(position, depth)
        } else {
            self.go_iterative_deepening(position, depth)
        }
    }

    #[inline(always)]
    pub fn calculate_stop_time(total_time: Option<u128>, increment_time: Option<u128>) -> Option<u128> {
        total_time.map(|total_time| total_time / AVERAGE_AMOUNT_OF_MOVES as u128 + increment_time.unwrap_or(0))
    }

    fn get_pv(&self, position: &Position, depth: usize, _best_move: BitMove) -> String {
        #[cfg(feature = "tt")]
        return self.get_pv_from_tt(position, depth);

        #[cfg(not(feature = "tt"))]
        return _best_move.to_uci_string()
    }

    // NOTE: There is a notable chance the pv will be ended early in case a different position
    // happens to have the same table index. The probability scales inversely with the
    // size of the transposition table.
    #[cfg(feature = "tt")]
    fn get_pv_from_tt(&self, position: &Position, depth: usize) -> String {
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

    #[cfg(feature = "syzygy_tablebase")]
    pub fn set_tablebase(&mut self, path: &str) {
        let result = crate::SyzygyTablebase::from_directory(path).ok();
        match result {
            Some(_) => uci_println!(self, "info string loaded tablebase successfully"),
            None => uci_println!(self, "info string error loading tablebase"),
        }
        self.tablebase = Arc::new(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_returns_non_empty_move() {
        assert_ne!(
            Search::default().go(
                &Position::starting_position(),
                Some(2),
                None
            ).bit_move,
            BitMove::EMPTY
        )
    }
}
