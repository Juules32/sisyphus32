extern crate rand;
use rand::Rng;

use crate::{bit_move::BitMove, pl, position::Position};

fn random_best_move(position: &mut Position, _depth: u8) -> BitMove {
    let moves = position.generate_legal_moves();
    moves[rand::rng().random_range(0..moves.len())]
}

fn best_move(position: &mut Position, depth: u8) -> BitMove {
    // TODO: Implement conditional search methods (minimax, alpha-beta, negamax)
    random_best_move(position, depth)
}

pub fn go(position: &mut Position, depth: u8) {
    //TODO: Implement conditional iterative deepening here
    let best_move = best_move(position, depth);

    pl!(format!("bestmove {}", best_move.to_uci_string()));
}
