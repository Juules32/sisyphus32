use crate::{bit_move::{BitMove, ScoringMove}, butterfly_heuristic::ButterflyHeuristic, color::Color, killer_moves::KillerMoves, piece::PieceType, position::Position, square::Square, transposition_table::{TTNodeType, TranspositionTable}};

const PIECE_SCORES: [i16; 12] = [100, 300, 320, 500, 900, 10000, -100, -300, -320, -500, -900, -10000];

const WP_POSITION_SCORES: [i16; 64] = [
     90,  90,  90,  90,  90,  90,  90,  90, 
     30,  30,  30,  40,  40,  30,  30,  30,
     20,  20,  25,  30,  30,  25,  20,  20,
     10,  10,  10,  20,  20,  10,  10,  10,
      5,   5,  10,  20,  20,   5,   5,   5,
      0,   0,   0,   5,   5,  -5,   0,   0, 
      0,   0,   0, -10, -10,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const WN_POSITION_SCORES: [i16; 64] = [
    -15,  -5,   0,   0,   0,   0,  -5, -15, 
     -5,   0,   0,  10,  10,   0,   0,  -5,
     -5,   5,  20,  20,  20,  20,   5,  -5,
     -5,  10,  20,  30,  30,  20,  10,  -5,
     -5,  10,  20,  30,  30,  20,  10,  -5,
     -5,   5,  20,  10,  10,  20,   5,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
    -10, -10,   0,   0,   0,   0, -10, -10,
];

const WB_POSITION_SCORES: [i16; 64] = [
     -5,   0,   0,   0,   0,   0,   0,  -5, 
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   5,   5,   0,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0, 
      0,  10,   0,   5,   5,   0,  10,   0,
      0,  30,   0,   0,   0,   0,  30,   0,
      0,   0, -10,   0,   0, -10,   0,   0,
];

const WR_POSITION_SCORES: [i16; 64] = [
     50,  50,  50,  50,  50,  50,  50,  50, 
     50,  50,  50,  50,  50,  50,  50,  50,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
];

const WQ_POSITION_SCORES: [i16; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const WK_POSITION_SCORES: [i16; 64] = [
     -5,   0,   0,   0,   0,   0,   0,  -5, 
      0,   0,   5,   5,   5,   5,   0,   0,
      0,   5,   5,  10,  10,   5,   5,   0,
      0,   5,  10,  20,  20,  10,   5,   0,
      0,   5,  10,  20,  20,  10,   5,   0,
      0,   0,   5,  10,  10,   5,   0,   0,
      0,   5,   5,  -5,  -5,  -5,   5,   0,
      0,   5,   5,  -5, -15,  -5,  10,   0,
];

const BP_POSITION_SCORES: [i16; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,  10,  10,   0,   0,   0,
      0,   0,   0,  -5,  -5,   5,   0,   0, 
     -5,  -5, -10, -20, -20,  -5,  -5,  -5,
    -10, -10, -10, -20, -20, -10, -10, -10,
    -20, -20, -25, -30, -30, -25, -20, -20,
    -30, -30, -30, -40, -40, -30, -30, -30,
    -90, -90, -90, -90, -90, -90, -90, -90, 
];

const BN_POSITION_SCORES: [i16; 64] = [
     10,  10,   0,   0,   0,   0,  10,  10,
      5,   0,   0,   0,   0,   0,   0,   5,
      5,  -5, -20, -10, -10, -20,  -5,   5,
      5, -10, -20, -30, -30, -20, -10,   5,
      5, -10, -20, -30, -30, -20, -10,   5,
      5,  -5, -20, -20, -20, -20,  -5,   5,
      5,   0,   0, -10, -10,   0,   0,   5,
     15,   5,   0,   0,   0,   0,   5,  15, 
];

const BB_POSITION_SCORES: [i16; 64] = [
      0,   0,  10,   0,   0,  10,   0,   0,
      0, -30,   0,   0,   0,   0, -30,   0,
      0, -10,   0,  -5,  -5,   0, -10,   0,
      0,   0, -10, -20, -20, -10,   0,   0, 
      0,   0, -10, -20, -20, -10,   0,   0,
      0,   0,   0,  -5,  -5,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      5,   0,   0,   0,   0,   0,   0,   5, 
];

const BR_POSITION_SCORES: [i16; 64] = [
      0,   0, -10, -20, -20, -10,   0,   0,
      0,   0, -10, -20, -20, -10,   0,   0,
      0,   0, -10, -20, -20, -10,   0,   0,
      0,   0, -10, -20, -20, -10,   0,   0,
      0,   0, -10, -20, -20, -10,   0,   0,
      0,   0, -10, -20, -20, -10,   0,   0,
    -50, -50, -50, -50, -50, -50, -50, -50,
    -50, -50, -50, -50, -50, -50, -50, -50, 
];

const BQ_POSITION_SCORES: [i16; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const BK_POSITION_SCORES: [i16; 64] = [
      0,  -5,  -5,   5,  15,   5, -10,   0,
      0,  -5,  -5,   5,   5,   5,  -5,   0,
      0,   0,  -5, -10, -10,  -5,   0,   0,
      0,  -5, -10, -20, -20, -10,  -5,   0,
      0,  -5, -10, -20, -20, -10,  -5,   0,
      0,  -5,  -5, -10, -10,  -5,  -5,   0,
      0,   0,  -5,  -5,  -5,  -5,   0,   0,
      5,   0,   0,   0,   0,   0,   0,   5, 
];

const PIECE_POSITION_SCORES: [&[i16; 64]; 12] = [
    &WP_POSITION_SCORES, &WN_POSITION_SCORES, &WB_POSITION_SCORES, &WR_POSITION_SCORES, &WQ_POSITION_SCORES, &WK_POSITION_SCORES,
    &BP_POSITION_SCORES, &BN_POSITION_SCORES, &BB_POSITION_SCORES, &BR_POSITION_SCORES, &BQ_POSITION_SCORES, &BK_POSITION_SCORES,
];

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

pub struct EvalPosition { }

impl EvalPosition {
    #[inline(always)]
    pub fn eval(position: &Position) -> ScoringMove {
        ScoringMove::blank(Square::ALL_SQUARES.iter().fold(0, |mut acc, &sq| {
            match position.get_piece(sq) {
                PieceType::None => acc,
                piece => {
                    acc += PIECE_SCORES[piece as usize];
                    
                    #[cfg(feature = "eval_piece_positions")]
                    { acc += PIECE_POSITION_SCORES[piece as usize][sq]; }

                    acc
                }
            }
        }) * match position.side {
            Color::White => 1,
            Color::Black => -1
        })
    }
}

pub struct EvalMove { }

impl EvalMove {
    #[inline(always)]
    pub fn eval(position: &Position, bit_move: BitMove) -> i16 {
        let mut score = if position.get_piece(bit_move.target()) == PieceType::None {
            0
        } else {
            MVV_LVA[position.get_piece(bit_move.source()) as usize][position.get_piece(bit_move.target()) as usize]
        };

        #[cfg(feature = "eval_transposition_table")]
        {
            if let Some(entry) = TranspositionTable::probe(position.zobrist_key) {
                if entry.best_move.bit_move == bit_move {
                    match entry.flag {
                        TTNodeType::Exact => score += 50,
                        TTNodeType::LowerBound => score += 30,
                        TTNodeType::UpperBound => score += 20,
                    }
                }
            }
        }

        #[cfg(feature = "killer_moves")]
        {
            if KillerMoves::get_primary(position.ply) == bit_move {
                score += 100;
            } else if KillerMoves::get_secondary(position.ply) == bit_move {
                score += 50;
            }
        }

        #[cfg(feature = "butterfly_heuristic")]
        {
            score += ButterflyHeuristic::get(position.side, bit_move.source(), bit_move.target());
        }
        score
    }
}
