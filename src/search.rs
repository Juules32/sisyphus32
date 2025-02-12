extern crate rand;
use rand::Rng;

use crate::{bit_move::ScoringMove, eval::Eval, pl, position::Position, timer::Timer};

pub struct Search { }

impl Search {

    fn random_best_move(position: &Position, _depth: u8) -> ScoringMove {
        let moves = position.generate_legal_moves();
        ScoringMove::from(moves[rand::rng().random_range(0..moves.len())])
    }
    
    fn minimax_best_move(position: &Position, depth: u8) -> ScoringMove {
        if depth == 0 {
        return Eval::basic(position);
    }
    
    position
    .generate_pseudo_legal_scoring_moves()
    .into_iter()
    .filter_map(|mut m: ScoringMove| {
        let mut position_copy = position.clone();
        if position_copy.make_move(m.bit_move) {
            m.score = -Self::minimax_best_move(&position_copy, depth - 1).score;
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

    fn best_scoring_move(position: &mut Position, depth: u8) -> ScoringMove {
        #[cfg(feature = "search_random")]
        return random_best_move(position, depth);
        
        #[cfg(feature = "search_minimax")]
        return Self::minimax_best_move(position, depth);
    }
    
    pub fn go(position: &mut Position, depth: u8) {
        //TODO: Implement conditional iterative deepening here
        let timer = Timer::new();
        let best_scoring_move = Self::best_scoring_move(position, depth);
        pl!(format!("info depth {} score cp {} time {} pv {}", depth, best_scoring_move.score, timer.get_time_passed_millis(), best_scoring_move.bit_move.to_uci_string()));
        pl!(format!("bestmove {}", best_scoring_move.bit_move.to_uci_string()));
    }

}
