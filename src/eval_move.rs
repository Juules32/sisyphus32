use crate::{bit_move::BitMove, butterfly_heuristic::ButterflyHeuristic, color::Color, killer_moves::KillerMoves, move_masks::MoveMasks, piece::Piece, position::Position, transposition_table::{TTNodeType, TranspositionTable}};

// Most valuable victim - least valuable attacker [attacker][victim]
const MVV_LVA: [[i16; 12]; 12] = [
    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600],
    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600],
];

pub struct EvalMove { }

impl EvalMove {
    #[inline(always)]
    pub fn eval(position: &Position, bit_move: BitMove) -> i16 {
        let mut score = 0;
        let source = bit_move.source();
        let target = bit_move.target();
        let piece = position.get_piece(source);
        let capture_option = position.try_get_piece(target);

        if capture_option.is_some() {
            score += MVV_LVA[piece][capture_option.unwrap()];

            #[cfg(feature = "unit_capture_with_check_eval")]
            {
                let enemy_king_bb = match position.side {
                    Color::White => position.bbs[Piece::BK],
                    Color::Black => position.bbs[Piece::WK],
                };
                if (MoveMasks::get_piece_mask(piece, target, position.ao) & enemy_king_bb).is_not_empty() {
                    score += 300
                }
            }
        };

        #[cfg(feature = "unit_eval_tt")]
        {
            if let Some(entry) = TranspositionTable::probe(position.zobrist_key) {
                if entry.best_move.bit_move == bit_move {
                    match entry.flag {
                        TTNodeType::Exact => score += 10000,
                        TTNodeType::LowerBound => score += 4000,
                        TTNodeType::UpperBound => score += 3000,
                    }
                }
            }
        }

        #[cfg(feature = "unit_killer_heuristic")]
        {
            if KillerMoves::get_primary(position.ply) == Some(bit_move) {
                score += 2000;
            } else if KillerMoves::get_secondary(position.ply) == Some(bit_move) {
                score += 1000;
            }
        }

        #[cfg(feature = "unit_butterfly_heuristic")]
        {
            score += ButterflyHeuristic::get(position.side, source, target);
        }

        score
    }
}
