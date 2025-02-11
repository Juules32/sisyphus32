extern crate rand;
use rand::Rng;

use crate::{bit_move::{BitMove, Move, ScoringMove}, eval, pl, position::Position, timer::Timer};

fn random_best_move(position: &Position, _depth: u8) -> ScoringMove {
    let moves = position.generate_legal_moves();
    ScoringMove::from(moves[rand::rng().random_range(0..moves.len())])
}

fn minimax_best_move(position: &Position, depth: u8) -> ScoringMove {
    if depth == 0 {
        return eval::basic(position);
    }

    position
        // TODO: Use pseudo-legal moves
        .generate_legal_scoring_moves()
        .into_iter()
        .map(|mut m: ScoringMove| {
            let mut position_copy = position.clone();
            position_copy.make_move(m.bit_move);
            m.score = -minimax_best_move(&position_copy, depth - 1).score;
            m
        })
        .max()
        .unwrap_or_else(|| match position.in_check() {
            true => ScoringMove::blank(-10000),
            false => ScoringMove::blank(0),
        })
}

fn best_scoring_move(position: &mut Position, depth: u8) -> ScoringMove {
    #[cfg(feature = "search_random")]
    return random_best_move(position, depth);
    
    #[cfg(feature = "search_minimax")]
    return minimax_best_move(position, depth);
}

pub fn go(position: &mut Position, depth: u8) {
    //TODO: Implement conditional iterative deepening here
    let timer = Timer::new();
    let best_scoring_move = best_scoring_move(position, depth);
    pl!(format!("info depth {} score cp {} time {} pv {}", depth, best_scoring_move.score, timer.get_time_passed_millis(), best_scoring_move.bit_move.to_uci_string()));
    pl!(format!("bestmove {}", best_scoring_move.bit_move.to_uci_string()));
}
