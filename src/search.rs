extern crate rand;
use rand::Rng;

use crate::{bit_move::ScoringMove, eval::Eval, move_generation::MoveGeneration, pl, position::Position, timer::Timer};

pub struct Search {
    timer: Timer,
    stop_time: u128,
}

impl Search {
    pub fn new(stop_time: u128) -> Search {
        Search {
            timer: Timer::new(),
            stop_time,
        }
    }

    fn random_best_move(&self, position: &Position, _depth: u8) -> ScoringMove {
        let moves = MoveGeneration::generate_legal_moves(position);
        ScoringMove::from(moves[rand::rng().random_range(0..moves.len())])
    }
    
    fn minimax_best_move(&self, position: &Position, depth: u8) -> ScoringMove {
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

    fn best_scoring_move(&self, position: &mut Position, depth: u8) -> ScoringMove {
        #[cfg(feature = "search_random")]
        return self.random_best_move(position, depth);
        
        #[cfg(feature = "search_minimax")]
        return self.minimax_best_move(position, depth);
    }
    
    pub fn go(&self, position: &mut Position, depth: u8) {
        //TODO: Implement conditional iterative deepening here
        let best_scoring_move = self.best_scoring_move(position, depth);
        pl!(format!("info depth {} score cp {} time {} pv {}", depth, best_scoring_move.score, self.timer.get_time_passed_millis(), best_scoring_move.bit_move.to_uci_string()));
        pl!(format!("bestmove {}", best_scoring_move.bit_move.to_uci_string()));
    }

}
