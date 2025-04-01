use core::fmt;

use crate::{bit_move::BitMove, bitboard::Bitboard, castling_rights::CastlingRights, color::Color, consts::{PIECE_TYPE_COUNT, SQUARE_COUNT}, eval_position::EvalPosition, fen::FenString, file::File, move_flag::MoveFlag, move_masks::MoveMasks, piece::Piece, square::Square, zobrist::ZobristKey};

#[derive(Clone)]
pub struct Position {
    #[cfg(feature = "unit_bb_array")]
    pub pps: [Option<Piece>; SQUARE_COUNT],

    pub bbs: [Bitboard; PIECE_TYPE_COUNT],
    pub wo: Bitboard,
    pub bo: Bitboard,
    pub ao: Bitboard,
    pub side: Color,
    pub en_passant_option: Option<Square>,
    pub castling_rights: CastlingRights,
    pub ply: u16,
    pub zobrist_key: ZobristKey,

    #[cfg(feature = "unit_tapered_eval")]
    pub game_phase_score: i16,
}

impl Position {
    #[inline(always)]
    pub fn merge_occupancies(&mut self) {
        self.ao = self.wo | self.bo;
    }

    #[inline(always)]
    pub fn populate_occupancies(&mut self) {
        self.wo = self.bbs[Piece::WP]
                | self.bbs[Piece::WN]
                | self.bbs[Piece::WB]
                | self.bbs[Piece::WR]
                | self.bbs[Piece::WQ]
                | self.bbs[Piece::WK];
        self.bo = self.bbs[Piece::BP]
                | self.bbs[Piece::BN]
                | self.bbs[Piece::BB]
                | self.bbs[Piece::BR]
                | self.bbs[Piece::BQ]
                | self.bbs[Piece::BK];

        self.merge_occupancies();
    }

    pub fn starting_position() -> Position {
        let mut position = Position {
            #[cfg(feature = "unit_bb_array")]
            pps: [
                Some(Piece::BR), Some(Piece::BN), Some(Piece::BB), Some(Piece::BQ), Some(Piece::BK), Some(Piece::BB), Some(Piece::BN), Some(Piece::BR),
                Some(Piece::BP), Some(Piece::BP), Some(Piece::BP), Some(Piece::BP), Some(Piece::BP), Some(Piece::BP), Some(Piece::BP), Some(Piece::BP),
                None,            None,            None,            None,            None,            None,            None,            None,
                None,            None,            None,            None,            None,            None,            None,            None,
                None,            None,            None,            None,            None,            None,            None,            None,
                None,            None,            None,            None,            None,            None,            None,            None,
                Some(Piece::WP), Some(Piece::WP), Some(Piece::WP), Some(Piece::WP), Some(Piece::WP), Some(Piece::WP), Some(Piece::WP), Some(Piece::WP),
                Some(Piece::WR), Some(Piece::WN), Some(Piece::WB), Some(Piece::WQ), Some(Piece::WK), Some(Piece::WB), Some(Piece::WN), Some(Piece::WR),
            ],

            bbs: [
                Bitboard::WP,
                Bitboard::WN,
                Bitboard::WB,
                Bitboard::WR,
                Bitboard::WQ,
                Bitboard::WK,
                Bitboard::BP,
                Bitboard::BN,
                Bitboard::BB,
                Bitboard::BR,
                Bitboard::BQ,
                Bitboard::BK,
            ],
            wo: Bitboard::WHITE_STARTING_PIECES,
            bo: Bitboard::BLACK_STARTING_PIECES,
            ao: Bitboard::ALL_STARTING_PIECES,
            side: Color::White,
            en_passant_option: None,
            castling_rights: CastlingRights::DEFAULT,
            ply: 0,
            zobrist_key: ZobristKey(0),
            
            #[cfg(feature = "unit_tapered_eval")]
            game_phase_score: 0,
        };

        position.zobrist_key = ZobristKey::generate(&position);

        #[cfg(feature = "unit_tapered_eval")]
        { position.game_phase_score = EvalPosition::get_game_phase_score(&position); }

        position
    }

    #[inline(always)]
    pub fn set_piece(&mut self, piece: Piece, sq: Square) {
        self.bbs[piece].set_sq(sq);

        #[cfg(feature = "unit_bb_array")]
        { self.pps[sq] = Some(piece); }
        
        self.zobrist_key.mod_piece(piece, sq);
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, piece: Piece, sq: Square) {
        self.bbs[piece].pop_sq(sq);

        #[cfg(feature = "unit_bb_array")]
        { self.pps[sq] = None; }

        self.zobrist_key.mod_piece(piece, sq);
    }

    #[inline(always)]
    pub fn zobrist_mods(&mut self) {
        self.zobrist_key.mod_side(self.side);
        self.zobrist_key.mod_castling(self.castling_rights);
        self.zobrist_key.mod_en_passant(self.en_passant_option);
    }

    #[inline]
    pub fn make_move(&mut self, bit_move: BitMove) {
        #[cfg(feature = "unit_bb")]
        let (source, target, piece, capture_option, flag_option) = bit_move.decode();

        #[cfg(feature = "unit_bb_array")]
        let (source, target, flag_option) = bit_move.decode();

        #[cfg(feature = "unit_bb_array")]
        let piece = self.get_piece(source);

        #[cfg(feature = "unit_bb_array")]
        let capture_option = self.get_piece_option(target);

        debug_assert_eq!(capture_option, self.get_piece_option(target));
        debug_assert_eq!(piece.color(), self.side);
        debug_assert!(capture_option.is_none_or(|capture| capture.color() == self.side.opposite()));
        debug_assert!(self.bbs[piece].is_set_sq(source));
        debug_assert!(capture_option.is_none_or(|capture| self.bbs[capture].is_set_sq(target)));

        // Modify the zobrist key before making the move
        self.zobrist_mods();

        // Removes captured piece
        // NOTE: Because of the way zobrist hashing is implemented,
        // it is important that the capture is removed before moving the piece.
        if let Some(capture) = capture_option {
            self.remove_piece(capture, target);

            #[cfg(feature = "unit_tapered_eval")]
            { self.game_phase_score -= EvalPosition::get_game_phase_piece_score(capture); }
        }

        // Moves piece
        self.remove_piece(piece, source);
        self.set_piece(piece, target);

        // Resets en-passant square option
        self.en_passant_option = None;

        match flag_option {
            None => (),
            Some(MoveFlag::WDoublePawn) => self.en_passant_option = Some(target.below()),
            Some(MoveFlag::BDoublePawn) => self.en_passant_option = Some(target.above()),
            Some(MoveFlag::WEnPassant) => {
                self.remove_piece(Piece::BP, target.below());
                
                #[cfg(feature = "unit_tapered_eval")]
                { self.game_phase_score -= EvalPosition::get_game_phase_piece_score(Piece::BP); }
            },
            Some(MoveFlag::BEnPassant) => {
                self.remove_piece(Piece::WP, target.above());
                
                #[cfg(feature = "unit_tapered_eval")]
                { self.game_phase_score -= EvalPosition::get_game_phase_piece_score(Piece::WP); }
            },
            Some(MoveFlag::WKCastle) => {
                self.remove_piece(Piece::WR, Square::H1);
                self.set_piece(Piece::WR, Square::F1);
            }
            Some(MoveFlag::WQCastle) => {
                self.remove_piece(Piece::WR, Square::A1);
                self.set_piece(Piece::WR, Square::D1);
            }
            Some(MoveFlag::BKCastle) => {
                self.remove_piece(Piece::BR, Square::H8);
                self.set_piece(Piece::BR, Square::F8);
            }
            Some(MoveFlag::BQCastle) => {
                self.remove_piece(Piece::BR, Square::A8);
                self.set_piece(Piece::BR, Square::D8);
            }
            Some(MoveFlag::PromoQ) => {
                self.remove_piece(piece, target);
                self.set_piece(
                    match self.side {
                        Color::White => Piece::WQ,
                        Color::Black => Piece::BQ,
                    },
                    target,
                );

                #[cfg(feature = "unit_tapered_eval")]
                {
                    self.game_phase_score -= EvalPosition::get_game_phase_piece_score(piece);
                    self.game_phase_score += EvalPosition::get_game_phase_piece_score(Piece::WQ);
                }
            }
            Some(MoveFlag::PromoR) => {
                self.remove_piece(piece, target);
                self.set_piece(
                    match self.side {
                        Color::White => Piece::WR,
                        Color::Black => Piece::BR,
                    },
                    target,
                );

                #[cfg(feature = "unit_tapered_eval")]
                {
                    self.game_phase_score -= EvalPosition::get_game_phase_piece_score(piece);
                    self.game_phase_score += EvalPosition::get_game_phase_piece_score(Piece::WR);
                }
            }
            Some(MoveFlag::PromoN) => {
                self.remove_piece(piece, target);
                self.set_piece(
                    match self.side {
                        Color::White => Piece::WN,
                        Color::Black => Piece::BN,
                    },
                    target,
                );

                #[cfg(feature = "unit_tapered_eval")]
                {
                    self.game_phase_score -= EvalPosition::get_game_phase_piece_score(piece);
                    self.game_phase_score += EvalPosition::get_game_phase_piece_score(Piece::WR);
                }
            }
            Some(MoveFlag::PromoB) => {
                self.remove_piece(piece, target);
                self.set_piece(
                    match self.side {
                        Color::White => Piece::WB,
                        Color::Black => Piece::BB,
                    },
                    target,
                );

                #[cfg(feature = "unit_tapered_eval")]
                {
                    self.game_phase_score -= EvalPosition::get_game_phase_piece_score(piece);
                    self.game_phase_score += EvalPosition::get_game_phase_piece_score(Piece::WR);
                }
            }
        };

        self.castling_rights.update(source, target);
        self.populate_occupancies();
        self.side.switch();

        // Modify the zobrist key after making the move
        self.zobrist_mods();
        debug_assert_eq!(self.zobrist_key, ZobristKey::generate(self), "{}", self);
    }

    #[inline]
    #[cfg(feature = "unit_revert_undo")]
    pub fn undo_move(&mut self, bit_move: BitMove, old_castling_rights: CastlingRights) {
        let (source, target, piece, capture_option, flag_option) = bit_move.decode();

        // Switches side first to make it easier to conceptualize
        self.side.switch();

        debug_assert_eq!(piece.color(), self.side);
        debug_assert!(capture_option.is_none_or(|capture| capture.color() == self.side.opposite()));

        self.set_piece(piece, source);
        self.remove_piece(piece, target);

        if let Some(capture) = capture_option {
            self.set_piece(capture, target);
        }

        self.en_passant_option = None;

        match flag_option {
            None | Some(MoveFlag::WDoublePawn) | Some(MoveFlag::BDoublePawn) => (),
            Some(MoveFlag::WEnPassant) => {
                self.en_passant_option = Some(target);
                self.set_piece(Piece::BP, target.below())
            }
            Some(MoveFlag::BEnPassant) => {
                self.en_passant_option = Some(target);
                self.set_piece(Piece::WP, target.above())
            }
            Some(MoveFlag::WKCastle) => {
                self.set_piece(Piece::WR, Square::H1);
                self.remove_piece(Piece::WR, Square::F1);
            }
            Some(MoveFlag::WQCastle) => {
                self.set_piece(Piece::WR, Square::A1);
                self.remove_piece(Piece::WR, Square::D1);
            }
            Some(MoveFlag::BKCastle) => {
                self.set_piece(Piece::BR, Square::H8);
                self.remove_piece(Piece::BR, Square::F8);
            }
            Some(MoveFlag::BQCastle) => {
                self.set_piece(Piece::BR, Square::A8);
                self.remove_piece(Piece::BR, Square::D8);
            }
            Some(MoveFlag::PromoQ) => {
                self.remove_piece(
                    match self.side {
                        Color::White => Piece::WQ,
                        Color::Black => Piece::BQ,
                    },
                    target,
                );
            }
            Some(MoveFlag::PromoR) => {
                self.remove_piece(
                    match self.side {
                        Color::White => Piece::WR,
                        Color::Black => Piece::BR,
                    },
                    target,
                );
            }
            Some(MoveFlag::PromoN) => {
                self.remove_piece(
                    match self.side {
                        Color::White => Piece::WN,
                        Color::Black => Piece::BN,
                    },
                    target,
                );
            }
            Some(MoveFlag::PromoB) => {
                self.remove_piece(
                    match self.side {
                        Color::White => Piece::WB,
                        Color::Black => Piece::BB,
                    },
                    target,
                );
            }
        };

        self.castling_rights = old_castling_rights;
        self.populate_occupancies();
    }

    // NOTE: In this function, self is supposed to be a clone of the current position state.
    #[inline(always)]
    pub fn apply_pseudo_legal_move(&mut self, bit_move: BitMove) -> bool {
        self.make_move(bit_move);
        if !self.in_check(self.side.opposite()) {
            self.ply += 1;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn is_square_attacked(&self, defending_side: Color, square: Square) -> bool {
        let &[enemy_pawn, enemy_knight, enemy_bishop, enemy_rook, enemy_queen, enemy_king] = match defending_side {
            Color::White => &Piece::BLACK_PIECES,
            Color::Black => &Piece::WHITE_PIECES,
        };

        (MoveMasks::get_pawn_capture_mask(defending_side, square) & self.bbs[enemy_pawn]).is_not_empty() ||
        (MoveMasks::get_knight_mask(square) & self.bbs[enemy_knight]).is_not_empty() ||
        (MoveMasks::get_bishop_mask(square, self.ao) & self.bbs[enemy_bishop]).is_not_empty() ||
        (MoveMasks::get_rook_mask(square, self.ao) & self.bbs[enemy_rook]).is_not_empty() ||
        (MoveMasks::get_queen_mask(square, self.ao) & self.bbs[enemy_queen]).is_not_empty() ||
        (MoveMasks::get_king_mask(square) & self.bbs[enemy_king]).is_not_empty()
    }

    pub fn in_check(&self, defending_side: Color) -> bool {
        match defending_side {
            Color::White => self.is_square_attacked(defending_side, Square::from(self.bbs[Piece::WK])),
            Color::Black => self.is_square_attacked(defending_side, Square::from(self.bbs[Piece::BK])),
        }
    }

    #[inline(always)]
    #[cfg(feature = "unit_bb")]
    pub fn get_piece(&self, square: Square) -> Piece {
        for piece in Piece::ALL_PIECES {
            if self.bbs[piece].is_set_sq(square) {
                return piece;
            }
        }
        panic!("Couldn't find some piece on {}", square);
    }

    #[inline(always)]
    #[cfg(feature = "unit_bb")]
    pub fn get_piece_option(&self, square: Square) -> Option<Piece> {
        for piece in Piece::ALL_PIECES {
            if self.bbs[piece].is_set_sq(square) {
                return Some(piece);
            }
        }
        None
    }

    #[inline(always)]
    #[cfg(feature = "unit_bb_array")]
    pub fn get_piece(&self, square: Square) -> Piece {
        self.pps[square].unwrap()
    }

    #[inline(always)]
    #[cfg(feature = "unit_bb_array")]
    pub fn get_piece_option(&self, square: Square) -> Option<Piece> {
        self.pps[square]
    }
}

impl Default for Position {
    fn default() -> Position {
        Position {
            #[cfg(feature = "unit_bb_array")]
            pps: [None; SQUARE_COUNT],

            bbs: [Bitboard::EMPTY; PIECE_TYPE_COUNT],
            wo: Bitboard::EMPTY,
            bo: Bitboard::EMPTY,
            ao: Bitboard::EMPTY,
            side: Color::White,
            en_passant_option: None,
            castling_rights: CastlingRights::NONE,
            ply: 0,
            zobrist_key: ZobristKey(0),

            #[cfg(feature = "unit_tapered_eval")]
            game_phase_score: 0,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");
        for sq in Square::ALL_SQUARES {
            if sq.file() == File::FA {
                s += &format!("  {}  ", sq.rank());
            }
            
            match self.get_piece_option(sq) {
                None => s += ". ",
                Some(piece) => s += &format!("{} ", piece),
            }

            if sq.file() == File::FH {
                s += "\n";
            }
        }
        s += "\n     a b c d e f g h\n";

        s += &format!("
          FEN: {}
         Side: {}
   En-passant: {:?}
     Castling: {}
  Zobrist Key: {:#x}\n",
            FenString::from(self),
            self.side,
            self.en_passant_option,
            self.castling_rights,
            self.zobrist_key.0,
        );
        
        f.pad(&s)
    }
}
