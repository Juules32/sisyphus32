use core::fmt;

use crate::{BitMove, Bitboard, CastlingRights, Color, EvalPosition, FenString, File, MoveFlag, MoveMasks, Piece, Square, ZobristKey, PIECE_TYPE_COUNT, SQUARE_COUNT};

#[derive(Clone)]
pub struct Position {
    #[cfg(feature = "unit_bb_array")]
    pub(crate) pps: [Option<Piece>; SQUARE_COUNT],

    pub(crate) bitboards: [Bitboard; PIECE_TYPE_COUNT],
    pub(crate) white_occupancy: Bitboard,
    pub(crate) black_occupancy: Bitboard,
    pub(crate) all_occupancy: Bitboard,
    pub side: Color,
    pub en_passant_option: Option<Square>,
    pub castling_rights: CastlingRights,
    pub zobrist_key: ZobristKey,
    pub(crate) ply: u16,

    #[cfg(feature = "unit_tapered_eval")]
    pub(crate) game_phase_score: i16,
}

impl Default for Position {
    fn default() -> Position {
        Position {
            #[cfg(feature = "unit_bb_array")]
            pps: [None; SQUARE_COUNT],

            bitboards: [Bitboard::EMPTY; PIECE_TYPE_COUNT],
            white_occupancy: Bitboard::EMPTY,
            black_occupancy: Bitboard::EMPTY,
            all_occupancy: Bitboard::EMPTY,
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

impl Position {
    #[inline(always)]
    pub(crate) fn merge_occupancies(&mut self) {
        self.all_occupancy = self.white_occupancy | self.black_occupancy;
    }

    #[inline(always)]
    pub(crate) fn populate_occupancies(&mut self) {
        self.white_occupancy = self.bitboards[Piece::WP]
                | self.bitboards[Piece::WN]
                | self.bitboards[Piece::WB]
                | self.bitboards[Piece::WR]
                | self.bitboards[Piece::WQ]
                | self.bitboards[Piece::WK];
        self.black_occupancy = self.bitboards[Piece::BP]
                | self.bitboards[Piece::BN]
                | self.bitboards[Piece::BB]
                | self.bitboards[Piece::BR]
                | self.bitboards[Piece::BQ]
                | self.bitboards[Piece::BK];

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

            bitboards: [
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
            white_occupancy: Bitboard::WHITE_STARTING_PIECES,
            black_occupancy: Bitboard::BLACK_STARTING_PIECES,
            all_occupancy: Bitboard::ALL_STARTING_PIECES,
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
        self.bitboards[piece].set_sq(sq);

        #[cfg(feature = "unit_bb_array")]
        { self.pps[sq] = Some(piece); }
        
        self.zobrist_key.mod_piece(piece, sq);
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, piece: Piece, sq: Square) {
        self.bitboards[piece].pop_sq(sq);

        #[cfg(feature = "unit_bb_array")]
        { self.pps[sq] = None; }

        self.zobrist_key.mod_piece(piece, sq);
    }

    #[inline(always)]
    pub(crate) fn zobrist_mods(&mut self) {
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
        debug_assert!(self.bitboards[piece].is_set_sq(source));
        debug_assert!(capture_option.is_none_or(|capture| self.bitboards[capture].is_set_sq(target)));

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
        debug_assert_eq!(self.zobrist_key, ZobristKey::generate(self), "{self}");
    }

    #[inline]
    #[cfg(feature = "unit_revert_undo")]
    pub(crate) fn undo_move(&mut self, bit_move: BitMove, old_castling_rights: CastlingRights) {
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
    pub(crate) fn apply_pseudo_legal_move(&mut self, bit_move: BitMove) -> bool {
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

        (MoveMasks::get_pawn_capture_mask(defending_side, square) & self.bitboards[enemy_pawn]).is_not_empty() ||
        (MoveMasks::get_knight_mask(square) & self.bitboards[enemy_knight]).is_not_empty() ||
        (MoveMasks::get_bishop_mask(square, self.all_occupancy) & self.bitboards[enemy_bishop]).is_not_empty() ||
        (MoveMasks::get_rook_mask(square, self.all_occupancy) & self.bitboards[enemy_rook]).is_not_empty() ||
        (MoveMasks::get_queen_mask(square, self.all_occupancy) & self.bitboards[enemy_queen]).is_not_empty() ||
        (MoveMasks::get_king_mask(square) & self.bitboards[enemy_king]).is_not_empty()
    }

    pub fn in_check(&self, defending_side: Color) -> bool {
        match defending_side {
            Color::White => self.is_square_attacked(defending_side, Square::from(self.bitboards[Piece::WK])),
            Color::Black => self.is_square_attacked(defending_side, Square::from(self.bitboards[Piece::BK])),
        }
    }

    #[inline(always)]
    #[cfg(feature = "unit_bb")]
    pub fn get_piece(&self, square: Square) -> Piece {
        for piece in Piece::ALL_PIECES {
            if self.bitboards[piece].is_set_sq(square) {
                return piece;
            }
        }
        panic!("Couldn't find some piece on {}", square);
    }

    #[inline(always)]
    #[cfg(feature = "unit_bb")]
    pub fn get_piece_option(&self, square: Square) -> Option<Piece> {
        for piece in Piece::ALL_PIECES {
            if self.bitboards[piece].is_set_sq(square) {
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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");
        for sq in Square::ALL_SQUARES {
            if sq.file() == File::FA {
                s += &format!("  {}  ", sq.rank());
            }
            
            match self.get_piece_option(sq) {
                None => s += ". ",
                Some(piece) => s += &format!("{piece} "),
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
