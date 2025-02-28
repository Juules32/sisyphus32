use std::mem;

use ctor::ctor;

use crate::{bit_move::BitMove, bitboard::Bitboard, butterfly_heuristic::ButterflyHeuristic, color::Color, file::File, killer_moves::KillerMoves, piece::PieceType, position::Position, square::Square, transposition_table::{TTNodeType, TranspositionTable}};

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

static mut FILE_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
static mut RANK_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
static mut ISOLATED_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
static mut WHITE_PASSED_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
static mut BLACK_PASSED_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };

const DOUBLED_PAWN_SCORE: i16 = -30;
const ISOLATED_PAWN_SCORE: i16 = -15;
const PASSED_PAWN_SCORES: [i16; 8] = [0, 10, 30, 50, 75, 100, 150, 200];
const SEMI_OPEN_FILE_SCORE: i16 = 10;
const OPEN_FILE_SCORE: i16 = 15;
const KING_ON_SEMI_OPEN_FILE_SCORE: i16 = -30;

pub struct EvalPosition { }

impl EvalPosition {
    #[inline(always)]
    unsafe fn init_file_masks() {
        for square in Square::ALL_SQUARES {
            FILE_MASKS[square] = Bitboard::ALL_FILES[square.file() as usize];
        }
    }

    #[inline(always)]
    unsafe fn init_rank_masks() {
        for square in Square::ALL_SQUARES {
            RANK_MASKS[square] = Bitboard::ALL_RANKS[square.rank() as usize];
        }
    }

    #[inline(always)]
    unsafe fn init_isolated_masks() {
        for square in Square::ALL_SQUARES {
            if square.file() != File::FA {
                ISOLATED_MASKS[square] |= Bitboard::ALL_FILES[square.file() as usize - 1];
            }

            if square.file() != File::FH {
                ISOLATED_MASKS[square] |= Bitboard::ALL_FILES[square.file() as usize + 1];
            }
        }
    }

    #[inline(always)]
    unsafe fn init_passed_masks() {
        for square in Square::ALL_SQUARES {
            for color in [Color::White, Color::Black] {
                #[allow(static_mut_refs)]
                let passed_masks_ref = match color {
                    Color::White => &mut WHITE_PASSED_MASKS,
                    Color::Black => &mut BLACK_PASSED_MASKS,
                };
                
                (*passed_masks_ref)[square] |= Bitboard::ALL_FILES[square.file() as usize];

                if square.file() != File::FA {
                    (*passed_masks_ref)[square] |= Bitboard::ALL_FILES[square.file() as usize - 1];
                }

                if square.file() != File::FH {
                    (*passed_masks_ref)[square] |= Bitboard::ALL_FILES[square.file() as usize + 1];
                }

                // NOTE: The rank slices depend on the rank order in the enum
                match color {
                    Color::White => {
                        for &rank_bb in Bitboard::ALL_RANKS[square.rank() as usize..].iter() {
                            (*passed_masks_ref)[square] &= !rank_bb;
                        }
                    },
                    Color::Black => {
                        for &rank_bb in Bitboard::ALL_RANKS[..=square.rank() as usize].iter() {
                            (*passed_masks_ref)[square] &= !rank_bb;
                        }
                    },
                }
            }
        }
    }

    #[inline(always)]
    fn get_file_mask(square: Square) -> Bitboard {
        unsafe { FILE_MASKS[square] }
    }

    #[inline(always)]
    fn get_isolated_mask(square: Square) -> Bitboard {
        unsafe { ISOLATED_MASKS[square] }
    }

    #[inline(always)]
    fn get_white_passed_mask(square: Square) -> Bitboard {
        unsafe { WHITE_PASSED_MASKS[square] }
    }
    
    #[inline(always)]
    fn get_black_passed_mask(square: Square) -> Bitboard {
        unsafe { BLACK_PASSED_MASKS[square] }
    }

    #[inline(always)]
    pub fn eval(position: &Position) -> i16 {
        let mut score = 0;
        let mut ao = position.ao;

        while ao != Bitboard::EMPTY {

            let sq = ao.pop_lsb();
            let piece = position.get_piece(sq);
            score += PIECE_SCORES[piece as usize];
            #[cfg(feature = "unit_eval_pps")]
            { score += PIECE_POSITION_SCORES[piece as usize][sq]; }

            #[cfg(feature = "unit_positional_eval")]
            if piece == PieceType::WP || piece == PieceType::BP {
                let mut pawn_score = 0;
                if (position.bbs[piece] & Self::get_file_mask(sq)).count_bits() > 1 {
                    pawn_score += DOUBLED_PAWN_SCORE;
                }
                
                if (position.bbs[piece] & Self::get_isolated_mask(sq)).is_empty() {
                    pawn_score += ISOLATED_PAWN_SCORE;
                }

                if piece == PieceType::WP {
                    if (position.bbs[PieceType::BP] & Self::get_white_passed_mask(sq)).is_empty() {
                        pawn_score += PASSED_PAWN_SCORES[7 - sq.rank() as usize];
                    }
                } else {
                    if (position.bbs[PieceType::WP] & Self::get_black_passed_mask(sq)).is_empty() {
                        pawn_score += PASSED_PAWN_SCORES[sq.rank() as usize];
                    }
                }

                if piece == PieceType::BP { pawn_score *= -1; }
                score += pawn_score
            } else if piece == PieceType::WR || piece == PieceType::BR {
                let mut rook_score = 0;

                if piece == PieceType::WR {
                    if (position.bbs[PieceType::WP] & Self::get_file_mask(sq)).is_empty() {
                        rook_score += SEMI_OPEN_FILE_SCORE;
                    }
                } else {
                    if (position.bbs[PieceType::BP] & Self::get_file_mask(sq)).is_empty() {
                        rook_score += SEMI_OPEN_FILE_SCORE;
                    }
                }

                if ((position.bbs[PieceType::WP] | position.bbs[PieceType::BP]) & Self::get_file_mask(sq)).is_empty() {
                    rook_score += OPEN_FILE_SCORE;
                }

                if piece == PieceType::BR { rook_score *= -1; }
                score += rook_score
            } else if piece == PieceType::WK || piece == PieceType::BK {
                let mut king_score = 0;

                if piece == PieceType::WK {
                    if (position.bbs[PieceType::WP] & Self::get_file_mask(sq)).is_empty() {
                        king_score += KING_ON_SEMI_OPEN_FILE_SCORE;
                    }
                } else {
                    if (position.bbs[PieceType::BP] & Self::get_file_mask(sq)).is_empty() {
                        king_score += KING_ON_SEMI_OPEN_FILE_SCORE;
                    }
                }

                if piece == PieceType::BR { king_score *= -1; }
                score += king_score
            }
        }

        score * match position.side {
            Color::White => 1,
            Color::Black => -1
        }
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

        #[cfg(feature = "unit_eval_tt")]
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

        #[cfg(feature = "unit_killer_heuristic")]
        {
            if KillerMoves::get_primary(position.ply) == Some(bit_move) {
                score += 100;
            } else if KillerMoves::get_secondary(position.ply) == Some(bit_move) {
                score += 50;
            }
        }

        #[cfg(feature = "unit_butterfly_heuristic")]
        {
            score += ButterflyHeuristic::get(position.side, bit_move.source(), bit_move.target());
        }
        score
    }
}

#[ctor]
pub unsafe fn init_all_masks() {
    EvalPosition::init_file_masks();
    EvalPosition::init_rank_masks();
    EvalPosition::init_isolated_masks();
    EvalPosition::init_passed_masks();
}
