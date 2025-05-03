use std::mem;

use crate::{bitboard::Bitboard, color::Color, consts::{FILE_COUNT, PIECE_TYPE_COUNT, SQUARE_COUNT}, file::File, piece::Piece, position::Position, score::Score, square::Square};

#[allow(unused_imports)]
use crate::move_masks::MoveMasks;

const BASE_PIECE_SCORES: [i16; PIECE_TYPE_COUNT] = [100, 300, 320, 500, 900, 10000, 100, 300, 320, 500, 900, 10000];

const OPENING_PIECE_SCORES: [i16; PIECE_TYPE_COUNT] = [82, 337, 365, 477, 1025, 12000, 82, 337, 365, 477, 1025, 12000];

const ENDGAME_PIECE_SCORES: [i16; PIECE_TYPE_COUNT] = [94, 281, 297, 512,  936, 12000, 94, 281, 297, 512, 936, 12000];

const BASE_PAWN_POSITION_SCORES: [i16; SQUARE_COUNT] = [
     90,  90,  90,  90,  90,  90,  90,  90, 
     30,  30,  30,  40,  40,  30,  30,  30,
     20,  20,  25,  30,  30,  25,  20,  20,
     10,  10,  10,  20,  20,  10,  10,  10,
      5,   5,  10,  20,  20,   5,   5,   5,
      0,   0,   0,   5,   5,  -5,   0,   0, 
      0,   0,   0, -10, -10,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const BASE_KNIGHT_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -15,  -5,   0,   0,   0,   0,  -5, -15, 
     -5,   0,   0,  10,  10,   0,   0,  -5,
     -5,   5,  20,  20,  20,  20,   5,  -5,
     -5,  10,  20,  30,  30,  20,  10,  -5,
     -5,  10,  20,  30,  30,  20,  10,  -5,
     -5,   5,  20,  10,  10,  20,   5,  -5,
     -5,   0,   0,   0,   0,   0,   0,  -5,
    -10, -10,   0,   0,   0,   0, -10, -10,
];

const BASE_BISHOP_POSITION_SCORES: [i16; SQUARE_COUNT] = [
     -5,   0,   0,   0,   0,   0,   0,  -5, 
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   5,   5,   0,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0, 
      0,  10,   0,   5,   5,   0,  10,   0,
      0,  30,   0,   0,   0,   0,  30,   0,
      0,   0, -10,   0,   0, -10,   0,   0,
];

const BASE_ROOK_POSITION_SCORES: [i16; SQUARE_COUNT] = [
     50,  50,  50,  50,  50,  50,  50,  50, 
     50,  50,  50,  50,  50,  50,  50,  50,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
];

const BASE_QUEEN_POSITION_SCORES: [i16; SQUARE_COUNT] = [
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const BASE_KING_POSITION_SCORES: [i16; SQUARE_COUNT] = [
     -5,   0,   0,   0,   0,   0,   0,  -5, 
      0,   0,   5,   5,   5,   5,   0,   0,
      0,   5,   5,  10,  10,   5,   5,   0,
      0,   5,  10,  20,  20,  10,   5,   0,
      0,   5,  10,  20,  20,  10,   5,   0,
      0,   0,   5,  10,  10,   5,   0,   0,
      0,   5,   5,  -5,  -5,  -5,   5,   0,
      0,   5,   5,  -5, -15,  -5,  10,   0,
];

const BASE_PIECE_POSITION_SCORES: [&[i16; SQUARE_COUNT]; PIECE_TYPE_COUNT] = [
    &BASE_PAWN_POSITION_SCORES, &BASE_KNIGHT_POSITION_SCORES, &BASE_BISHOP_POSITION_SCORES, &BASE_ROOK_POSITION_SCORES, &BASE_QUEEN_POSITION_SCORES, &BASE_KING_POSITION_SCORES,
    &BASE_PAWN_POSITION_SCORES, &BASE_KNIGHT_POSITION_SCORES, &BASE_BISHOP_POSITION_SCORES, &BASE_ROOK_POSITION_SCORES, &BASE_QUEEN_POSITION_SCORES, &BASE_KING_POSITION_SCORES,
];

const OPENING_PAWN_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    0,   0,   0,   0,   0,   0,  0,   0,
    98, 134,  61,  95,  68, 126, 34, -11,
    -6,   7,  26,  31,  65,  56, 25, -20,
    -14,  13,   6,  21,  23,  12, 17, -23,
    -27,  -2,  -5,  12,  17,   6, 10, -25,
    -26,  -4,  -4, -10,   3,   3, 33, -12,
    -35,  -1, -20, -23, -15,  24, 38, -22,
    0,   0,   0,   0,   0,   0,  0,   0,
];
    
const OPENING_KNIGHT_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -167, -89, -34, -49,  61, -97, -15, -107,
    -73, -41,  72,  36,  23,  62,   7,  -17,
    -47,  60,  37,  65,  84, 129,  73,   44,
    -9,  17,  19,  53,  37,  69,  18,   22,
    -13,   4,  16,  13,  28,  19,  21,   -8,
    -23,  -9,  12,  10,  19,  17,  25,  -16,
    -29, -53, -12,  -3,  -1,  18, -14,  -19,
    -105, -21, -58, -33, -17, -28, -19,  -23,
];

const OPENING_BISHOP_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -29,   4, -82, -37, -25, -42,   7,  -8,
    -26,  16, -18, -13,  30,  59,  18, -47,
    -16,  37,  43,  40,  35,  50,  37,  -2,
    -4,   5,  19,  50,  37,  37,   7,  -2,
    -6,  13,  13,  26,  34,  12,  10,   4,
    0,  15,  15,  15,  14,  27,  18,  10,
    4,  15,  16,   0,   7,  21,  33,   1,
    -33,  -3, -14, -21, -13, -12, -39, -21,
];

const OPENING_ROOK_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    32,  42,  32,  51, 63,  9,  31,  43,
    27,  32,  58,  62, 80, 67,  26,  44,
    -5,  19,  26,  36, 17, 45,  61,  16,
    -24, -11,   7,  26, 24, 35,  -8, -20,
    -36, -26, -12,  -1,  9, -7,   6, -23,
    -45, -25, -16, -17,  3,  0,  -5, -33,
    -44, -16, -20,  -9, -1, 11,  -6, -71,
    -19, -13,   1,  17, 16,  7, -37, -26,
];

const OPENING_QUEEN_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -28,   0,  29,  12,  59,  44,  43,  45,
    -24, -39,  -5,   1, -16,  57,  28,  54,
    -13, -17,   7,   8,  29,  56,  47,  57,
    -27, -27, -16, -16,  -1,  17,  -2,   1,
    -9, -26,  -9, -10,  -2,  -4,   3,  -3,
    -14,   2, -11,  -2,  -5,   2,  14,   5,
    -35,  -8,  11,   2,   8,  15,  -3,   1,
    -1, -18,  -9,  10, -15, -25, -31, -50,
];

const OPENING_KING_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -65,  23,  16, -15, -56, -34,   2,  13,
    29,  -1, -20,  -7,  -8,  -4, -38, -29,
    -9,  24,   2, -16, -20,   6,  22, -22,
    -17, -20, -12, -27, -30, -25, -14, -36,
    -49,  -1, -27, -39, -46, -44, -33, -51,
    -14, -14, -22, -46, -44, -30, -15, -27,
    1,   7,  -8, -64, -43, -16,   9,   8,
    -15,  36,  12, -54,   8, -28,  24,  14,
];

const OPENING_PIECE_POSITION_SCORES: [&[i16; SQUARE_COUNT]; PIECE_TYPE_COUNT] = [
    &OPENING_PAWN_POSITION_SCORES, &OPENING_KNIGHT_POSITION_SCORES, &OPENING_BISHOP_POSITION_SCORES, &OPENING_ROOK_POSITION_SCORES, &OPENING_QUEEN_POSITION_SCORES, &OPENING_KING_POSITION_SCORES,
    &OPENING_PAWN_POSITION_SCORES, &OPENING_KNIGHT_POSITION_SCORES, &OPENING_BISHOP_POSITION_SCORES, &OPENING_ROOK_POSITION_SCORES, &OPENING_QUEEN_POSITION_SCORES, &OPENING_KING_POSITION_SCORES,
];

const ENDGAME_PAWN_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    178, 173, 158, 134, 147, 132, 165, 187,
    94, 100,  85,  67,  56,  53,  82,  84,
    32,  24,  13,   5,  -2,   4,  17,  17,
    13,   9,  -3,  -7,  -7,  -8,   3,  -1,
    4,   7,  -6,   1,   0,  -5,  -1,  -8,
    13,   8,   8,  10,  13,   0,   2,  -7,
    0,   0,   0,   0,   0,   0,   0,   0,
];

const ENDGAME_KNIGHT_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -58, -38, -13, -28, -31, -27, -63, -99,
    -25,  -8, -25,  -2,  -9, -25, -24, -52,
    -24, -20,  10,   9,  -1,  -9, -19, -41,
    -17,   3,  22,  22,  22,  11,   8, -18,
    -18,  -6,  16,  25,  16,  17,   4, -18,
    -23,  -3,  -1,  15,  10,  -3, -20, -22,
    -42, -20, -10,  -5,  -2, -20, -23, -44,
    -29, -51, -23, -15, -22, -18, -50, -64,
];

const ENDGAME_BISHOP_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -14, -21, -11,  -8, -7,  -9, -17, -24,
    -8,  -4,   7, -12, -3, -13,  -4, -14,
    2,  -8,   0,  -1, -2,   6,   0,   4,
    -3,   9,  12,   9, 14,  10,   3,   2,
    -6,   3,  13,  19,  7,  10,  -3,  -9,
    -12,  -3,   8,  10, 13,   3,  -7, -15,
    -14, -18,  -7,  -1,  4,  -9, -15, -27,
    -23,  -9, -23,  -5, -9, -16,  -5, -17,
];

const ENDGAME_ROOK_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    13, 10, 18, 15, 12,  12,   8,   5,
    11, 13, 13, 11, -3,   3,   8,   3,
    7,  7,  7,  5,  4,  -3,  -5,  -3,
    4,  3, 13,  1,  2,   1,  -1,   2,
    3,  5,  8,  4, -5,  -6,  -8, -11,
    -4,  0, -5, -1, -7, -12,  -8, -16,
    -6, -6,  0,  2, -9,  -9, -11,  -3,
    -9,  2,  3, -1, -5, -13,   4, -20,
];

const ENDGAME_QUEEN_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -9,  22,  22,  27,  27,  19,  10,  20,
    -17,  20,  32,  41,  58,  25,  30,   0,
    -20,   6,   9,  49,  47,  35,  19,   9,
    3,  22,  24,  45,  57,  40,  57,  36,
    -18,  28,  19,  47,  31,  34,  39,  23,
    -16, -27,  15,   6,   9,  17,  10,   5,
    -22, -23, -30, -16, -16, -23, -36, -32,
    -33, -28, -22, -43,  -5, -32, -20, -41,
];

const ENDGAME_KING_POSITION_SCORES: [i16; SQUARE_COUNT] = [
    -74, -35, -18, -18, -11,  15,   4, -17,
    -12,  17,  14,  17,  17,  38,  23,  11,
    10,  17,  23,  15,  20,  45,  44,  13,
    -8,  22,  24,  27,  26,  33,  26,   3,
    -18,  -4,  21,  24,  27,  23,   9, -11,
    -19,  -3,  11,  21,  23,  16,   7,  -9,
    -27, -11,   4,  13,  14,   4,  -5, -17,
    -53, -34, -21, -11, -28, -14, -24, -43
];

const ENDGAME_PIECE_POSITION_SCORES: [&[i16; SQUARE_COUNT]; PIECE_TYPE_COUNT] = [
    &ENDGAME_PAWN_POSITION_SCORES, &ENDGAME_KNIGHT_POSITION_SCORES, &ENDGAME_BISHOP_POSITION_SCORES, &ENDGAME_ROOK_POSITION_SCORES, &ENDGAME_QUEEN_POSITION_SCORES, &ENDGAME_KING_POSITION_SCORES,
    &ENDGAME_PAWN_POSITION_SCORES, &ENDGAME_KNIGHT_POSITION_SCORES, &ENDGAME_BISHOP_POSITION_SCORES, &ENDGAME_ROOK_POSITION_SCORES, &ENDGAME_QUEEN_POSITION_SCORES, &ENDGAME_KING_POSITION_SCORES,
];

const OPENING_PHASE_CUTOFF: i16 = 6192;
const ENDGAME_PHASE_CUTOFF: i16 = 518;

const FLIPPED_SQUARE_INDEX: [usize; SQUARE_COUNT] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,
];

const DOUBLED_PAWN_SCORE: i16 = -30;
const ISOLATED_PAWN_SCORE: i16 = -15;
const PASSED_PAWN_SCORES: [i16; FILE_COUNT] = [0, 10, 30, 50, 75, 100, 150, 200];
const SEMI_OPEN_FILE_SCORE: i16 = 10;
const OPEN_FILE_SCORE: i16 = 15;
const KING_ON_SEMI_OPEN_FILE_SCORE: i16 = -30;
const KING_ADJACENCY_SCORE: i16 = 15;
const PSEUDO_PIN_SCORE: i16 = 20;
const BISHOP_PAIR_SCORE: i16 = 20;
const CHECK_SCORE: i16 = 30;
const ATTACK_UNITS_SCORE: i16 = 10;

static mut FILE_MASKS: [Bitboard; SQUARE_COUNT] = unsafe { mem::zeroed() };
static mut RANK_MASKS: [Bitboard; SQUARE_COUNT] = unsafe { mem::zeroed() };
static mut ISOLATED_MASKS: [Bitboard; SQUARE_COUNT] = unsafe { mem::zeroed() };
static mut WHITE_PASSED_MASKS: [Bitboard; SQUARE_COUNT] = unsafe { mem::zeroed() };
static mut BLACK_PASSED_MASKS: [Bitboard; SQUARE_COUNT] = unsafe { mem::zeroed() };

#[derive(Clone, Copy, Debug)]
enum GamePhase {
    Opening,
    Middlegame,
    Endgame,
}

pub struct EvalPosition;

impl EvalPosition {

    /// # Safety
    ///
    /// This function is safe, as it is called before any other function with ctor.
    pub unsafe fn init_positional_masks() {
        Self::init_file_masks();
        Self::init_rank_masks();
        Self::init_isolated_masks();
        Self::init_passed_masks();
    }

    unsafe fn init_file_masks() {
        for square in Square::ALL_SQUARES {
            FILE_MASKS[square] = Bitboard::ALL_FILES[square.file() as usize];
        }
    }

    unsafe fn init_rank_masks() {
        for square in Square::ALL_SQUARES {
            RANK_MASKS[square] = Bitboard::ALL_RANKS[square.rank() as usize];
        }
    }

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
    pub fn get_base_piece_position_score(piece: Piece, square: Square, color: Color) -> i16 {
        BASE_PIECE_POSITION_SCORES[piece][Self::get_positional_index(square, color)]
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
    pub fn get_game_phase_score(position: &Position) -> i16 {
        let mut game_phase_score = 0;

        for piece in Piece::ALL_PIECES_EXPECT_PAWNS_AND_KINGS {
            game_phase_score += position.bitboards[piece].count_bits() as i16 * OPENING_PIECE_SCORES[piece];
        }

        game_phase_score
    }

    #[inline(always)]
    pub fn get_game_phase_piece_score(piece: Piece) -> i16 {
        OPENING_PIECE_SCORES[piece]
    }

    #[inline(always)]
    fn get_game_phase(game_phase_score: i16) -> GamePhase {
        if game_phase_score > OPENING_PHASE_CUTOFF {
            GamePhase::Opening
        } else if game_phase_score < ENDGAME_PHASE_CUTOFF {
            GamePhase::Endgame
        } else {
            GamePhase::Middlegame
        }
    }

    #[inline(always)]
    fn get_positional_index(square: Square, color: Color) -> usize {
        match color {
            Color::White => square as usize,
            Color::Black => FLIPPED_SQUARE_INDEX[square as usize],
        }
    }

    #[inline(always)]
    fn get_tapered_score(game_phase: GamePhase, game_phase_score: i16, opening_score: i16, endgame_score: i16) -> i16 {
        match game_phase {
            // NOTE: The i32 casting is necessary to avoid scoring overflows
            GamePhase::Middlegame => ((
                opening_score as i32 * game_phase_score as i32 +
                endgame_score as i32 * (OPENING_PHASE_CUTOFF as i32 - game_phase_score as i32)
            ) / OPENING_PHASE_CUTOFF as i32) as i16,
            GamePhase::Opening => opening_score,
            GamePhase::Endgame => endgame_score,
        }
    }

    #[inline(always)]
    pub fn eval(position: &Position) -> Score {
        let mut score = Score::ZERO;
        let mut opening_score = 0;
        let mut endgame_score = 0;
        let mut ao_copy = position.all_occupancy;

        #[cfg(feature = "unit_tapered_eval")]
        let game_phase = Self::get_game_phase(position.game_phase_score);

        while ao_copy != Bitboard::EMPTY {
            let sq = ao_copy.pop_lsb();
            let piece = position.get_piece(sq);
            let piece_color_modifier = match piece.color() {
                Color::White => 1,
                Color::Black => -1,
            };

            let positional_index = Self::get_positional_index(sq, piece.color());

            #[allow(unused_mut)]
            let mut piece_score = 0;

            #[cfg(not(feature = "unit_tapered_eval"))]
            {
                piece_score += BASE_PIECE_SCORES[piece];
                #[cfg(feature = "unit_pst")]
                { piece_score += BASE_PIECE_POSITION_SCORES[piece][positional_index]; }
            }

            #[cfg(feature = "unit_tapered_eval")]
            {
                opening_score += OPENING_PIECE_SCORES[piece] * piece_color_modifier;
                endgame_score += ENDGAME_PIECE_SCORES[piece] * piece_color_modifier;

                #[cfg(feature = "unit_pst")]
                {
                    opening_score += OPENING_PIECE_POSITION_SCORES[piece][positional_index] * piece_color_modifier;
                    endgame_score += ENDGAME_PIECE_POSITION_SCORES[piece][positional_index] * piece_color_modifier;
                }
            }

            #[cfg(feature = "unit_positional_eval")]
            if piece == Piece::WP || piece == Piece::BP {
                if (position.bitboards[piece] & Self::get_file_mask(sq)).count_bits() > 1 {
                    piece_score += DOUBLED_PAWN_SCORE;
                }
                
                if (position.bitboards[piece] & Self::get_isolated_mask(sq)).is_empty() {
                    piece_score += ISOLATED_PAWN_SCORE;
                }

                if piece == Piece::WP {
                    if (position.bitboards[Piece::BP] & Self::get_white_passed_mask(sq)).is_empty() {
                        piece_score += PASSED_PAWN_SCORES[7 - sq.rank() as usize];
                    }
                } else {
                    if (position.bitboards[Piece::WP] & Self::get_black_passed_mask(sq)).is_empty() {
                        piece_score += PASSED_PAWN_SCORES[sq.rank() as usize];
                    }
                }
            } else if piece == Piece::WR || piece == Piece::BR {
                if piece == Piece::WR {
                    if (position.bitboards[Piece::WP] & Self::get_file_mask(sq)).is_empty() {
                        piece_score += SEMI_OPEN_FILE_SCORE;
                    }
                } else {
                    if (position.bitboards[Piece::BP] & Self::get_file_mask(sq)).is_empty() {
                        piece_score += SEMI_OPEN_FILE_SCORE;
                    }
                }

                if ((position.bitboards[Piece::WP] | position.bitboards[Piece::BP]) & Self::get_file_mask(sq)).is_empty() {
                    piece_score += OPEN_FILE_SCORE;
                }
            } else if piece == Piece::WK || piece == Piece::BK {
                if piece == Piece::WK {
                    if (position.bitboards[Piece::WP] & Self::get_file_mask(sq)).is_empty() {
                        piece_score += KING_ON_SEMI_OPEN_FILE_SCORE;
                    }

                    piece_score += (position.white_occupancy & MoveMasks::get_king_mask(sq)).count_bits() as i16 * KING_ADJACENCY_SCORE;
                    piece_score -= (position.black_occupancy & MoveMasks::get_king_mask(sq)).count_bits() as i16 * KING_ADJACENCY_SCORE;
                } else {
                    if (position.bitboards[Piece::BP] & Self::get_file_mask(sq)).is_empty() {
                        piece_score += KING_ON_SEMI_OPEN_FILE_SCORE;
                    }

                    piece_score += (position.black_occupancy & MoveMasks::get_king_mask(sq)).count_bits() as i16 * KING_ADJACENCY_SCORE;
                    piece_score -= (position.white_occupancy & MoveMasks::get_king_mask(sq)).count_bits() as i16 * KING_ADJACENCY_SCORE;
                }
            }

            score += piece_score * piece_color_modifier;
        }
        
        #[cfg(feature = "unit_tapered_eval")]
        { score += Self::get_tapered_score(game_phase, position.game_phase_score, opening_score, endgame_score); }

        #[cfg(feature = "unit_pseudo_pins")]
        {
            let wk_square = Square::from(position.bitboards[Piece::WK]);
            let bk_square = Square::from(position.bitboards[Piece::BK]);

            score -= (MoveMasks::get_bishop_mask_empty_occupancy(wk_square) & position.bitboards[Piece::BB]).count_bits() as i16 * PSEUDO_PIN_SCORE;
            score -= (MoveMasks::get_rook_mask_empty_occupancy(wk_square) & position.bitboards[Piece::BR]).count_bits() as i16 * PSEUDO_PIN_SCORE;
            score -= (MoveMasks::get_queen_mask_empty_occupancy(wk_square) & position.bitboards[Piece::BQ]).count_bits() as i16 * PSEUDO_PIN_SCORE;
            score += (MoveMasks::get_bishop_mask_empty_occupancy(bk_square) & position.bitboards[Piece::WB]).count_bits() as i16 * PSEUDO_PIN_SCORE;
            score += (MoveMasks::get_rook_mask_empty_occupancy(bk_square) & position.bitboards[Piece::WR]).count_bits() as i16 * PSEUDO_PIN_SCORE;
            score += (MoveMasks::get_queen_mask_empty_occupancy(bk_square) & position.bitboards[Piece::WQ]).count_bits() as i16 * PSEUDO_PIN_SCORE;
        }

        if position.bitboards[Piece::WB].count_bits() >= 2 {
            score += BISHOP_PAIR_SCORE;
        }

        if position.bitboards[Piece::BB].count_bits() >= 2 {
            score -= BISHOP_PAIR_SCORE;
        }

        if position.in_check(position.side) {
            score -= CHECK_SCORE;
        }

        match position.side {
            Color::White => score,
            Color::Black => -score,
        }
    }
}
