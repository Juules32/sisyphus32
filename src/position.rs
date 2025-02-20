use core::fmt;
use crate::{bit_move::BitMove, bitboard::Bitboard, castling_rights::CastlingRights, color::Color, fen::FenString, move_flag::MoveFlag, move_masks::MoveMasks, piece::PieceType, square::Square, zobrist::ZobristKey};

#[derive(Clone)]
pub struct Position {
    #[cfg(feature = "board_representation_array")]
    pub pps: [PieceType; 64],

    pub bbs: [Bitboard; 12],
    pub wo: Bitboard,
    pub bo: Bitboard,
    pub ao: Bitboard,
    pub side: Color,
    pub en_passant_sq: Square,
    pub castling_rights: CastlingRights,

    #[cfg(feature = "transposition_table")]
    pub zobrist_key: ZobristKey,
}

impl Position {
    #[inline(always)]
    pub fn merge_occupancies(&mut self) {
        self.ao = self.wo | self.bo;
    }

    #[inline(always)]
    pub fn populate_occupancies(&mut self) {
        self.wo = self.bbs[PieceType::WP]
                | self.bbs[PieceType::WN]
                | self.bbs[PieceType::WB]
                | self.bbs[PieceType::WR]
                | self.bbs[PieceType::WQ]
                | self.bbs[PieceType::WK];
        self.bo = self.bbs[PieceType::BP]
                | self.bbs[PieceType::BN]
                | self.bbs[PieceType::BB]
                | self.bbs[PieceType::BR]
                | self.bbs[PieceType::BQ]
                | self.bbs[PieceType::BK];

        self.merge_occupancies();
    }

    pub fn starting_position() -> Position {
        let mut position = Position {
            #[cfg(feature = "board_representation_array")]
            pps: [
                PieceType::BR, PieceType::BN, PieceType::BB, PieceType::BQ, PieceType::BK, PieceType::BB, PieceType::BN, PieceType::BR,
                PieceType::BP, PieceType::BP, PieceType::BP, PieceType::BP, PieceType::BP, PieceType::BP, PieceType::BP, PieceType::BP,
                PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None,
                PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None,
                PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None,
                PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None, PieceType::None,
                PieceType::WP, PieceType::WP, PieceType::WP, PieceType::WP, PieceType::WP, PieceType::WP, PieceType::WP, PieceType::WP,
                PieceType::WR, PieceType::WN, PieceType::WB, PieceType::WQ, PieceType::WK, PieceType::WB, PieceType::WN, PieceType::WR,
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
            en_passant_sq: Square::None,
            castling_rights: CastlingRights::DEFAULT,

            #[cfg(feature = "transposition_table")]
            zobrist_key: ZobristKey(0),
        };

        #[cfg(feature = "transposition_table")]
        { position.zobrist_key = ZobristKey::generate(&position); }

        position
    }

    #[inline(always)]
    pub fn set_piece(&mut self, piece: PieceType, sq: Square) {
        self.bbs[piece].set_sq(sq);

        #[cfg(feature = "board_representation_array")]
        { self.pps[sq] = piece; }
        
        #[cfg(feature = "transposition_table")]
        { self.zobrist_key.mod_piece(piece, sq); }
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, piece: PieceType, sq: Square) {
        self.bbs[piece].pop_sq(sq);

        #[cfg(feature = "board_representation_array")]
        { self.pps[sq] = PieceType::None; }

        #[cfg(feature = "transposition_table")]
        { self.zobrist_key.mod_piece(piece, sq); }
    }

    #[inline(always)]
    #[cfg(feature = "transposition_table")]
    pub fn zobrist_mods(&mut self) {
        self.zobrist_key.mod_side(self.side);
        self.zobrist_key.mod_castling(self.castling_rights);
        self.zobrist_key.mod_en_passant(self.en_passant_sq);
    }

    #[inline]
    pub fn make_move(&mut self, bit_move: BitMove) {
        #[cfg(feature = "board_representation_bitboard")]
        let (source, target, piece, capture, flag) = bit_move.decode();

        #[cfg(feature = "board_representation_array")]
        let (source, target, flag) = bit_move.decode();

        #[cfg(feature = "board_representation_array")]
        let piece = self.pps[source];

        #[cfg(feature = "board_representation_array")]
        let capture = self.pps[target];

        debug_assert_eq!(piece.color(), self.side);
        debug_assert!(capture == PieceType::None || capture.color() == self.side.opposite());
        debug_assert!(self.bbs[piece].is_set_sq(source));
        debug_assert!(capture == PieceType::None || self.bbs[capture].is_set_sq(target));

        // Modify the zobrist hash before making the move
        #[cfg(feature = "transposition_table")]
        self.zobrist_mods();


        // Removes captured piece
        // NOTE: Because of the way zobrist hashing is implemented,
        // it is important that the capture is removed before moving the piece.
        if capture != PieceType::None {
            self.remove_piece(capture, target);
        }

        // Moves piece
        self.remove_piece(piece, source);
        self.set_piece(piece, target);

        // Resets en-passant square
        self.en_passant_sq = Square::None;

        match flag {
            MoveFlag::None => (),
            MoveFlag::WDoublePawn => self.en_passant_sq = target.below(),
            MoveFlag::BDoublePawn => self.en_passant_sq = target.above(),
            MoveFlag::WEnPassant => self.remove_piece(PieceType::BP, target.below()),
            MoveFlag::BEnPassant => self.remove_piece(PieceType::WP, target.above()),
            MoveFlag::WKCastle => {
                self.remove_piece(PieceType::WR, Square::H1);
                self.set_piece(PieceType::WR, Square::F1);
            }
            MoveFlag::WQCastle => {
                self.remove_piece(PieceType::WR, Square::A1);
                self.set_piece(PieceType::WR, Square::D1);
            }
            MoveFlag::BKCastle => {
                self.remove_piece(PieceType::BR, Square::H8);
                self.set_piece(PieceType::BR, Square::F8);
            }
            MoveFlag::BQCastle => {
                self.remove_piece(PieceType::BR, Square::A8);
                self.set_piece(PieceType::BR, Square::D8);
            }
            MoveFlag::PromoQ => {
                self.remove_piece(piece, target);
                self.set_piece(
                    match self.side {
                        Color::White => PieceType::WQ,
                        Color::Black => PieceType::BQ,
                    },
                    target,
                );
            }
            MoveFlag::PromoR => {
                self.remove_piece(piece, target);
                self.set_piece(
                    match self.side {
                        Color::White => PieceType::WR,
                        Color::Black => PieceType::BR,
                    },
                    target,
                );
            }
            MoveFlag::PromoN => {
                self.remove_piece(piece, target);
                self.set_piece(
                    match self.side {
                        Color::White => PieceType::WN,
                        Color::Black => PieceType::BN,
                    },
                    target,
                );
            }
            MoveFlag::PromoB => {
                self.remove_piece(piece, target);
                self.set_piece(
                    match self.side {
                        Color::White => PieceType::WB,
                        Color::Black => PieceType::BB,
                    },
                    target,
                );
            }
        };

        self.castling_rights.update(source, target);
        self.populate_occupancies();
        self.side.switch();

        // Modify the zobrist hash after making the move
        #[cfg(feature = "transposition_table")]
        self.zobrist_mods();

        debug_assert_eq!(self.zobrist_key, ZobristKey::generate(self), "{}", self);
    }

    #[inline]
    #[cfg(feature = "revert_with_undo_move")]
    pub fn undo_move(&mut self, bit_move: BitMove, old_castling_rights: CastlingRights) {
        let (source, target, piece, capture, flag) = bit_move.decode();

        // Switches side first to make it easier to conceptualize
        self.side.switch();

        debug_assert_eq!(piece.color(), self.side);
        debug_assert!(capture == PieceType::None || capture.color() == self.side.opposite());

        self.set_piece(piece, source);
        self.remove_piece(piece, target);

        if capture != PieceType::None {
            self.set_piece(capture, target);
        }

        self.en_passant_sq = Square::None;

        match flag {
            MoveFlag::None | MoveFlag::WDoublePawn | MoveFlag::BDoublePawn => (),
            MoveFlag::WEnPassant => {
                self.en_passant_sq = target;
                self.set_piece(PieceType::BP, target.below())
            }
            MoveFlag::BEnPassant => {
                self.en_passant_sq = target;
                self.set_piece(PieceType::WP, target.above())
            }
            MoveFlag::WKCastle => {
                self.set_piece(PieceType::WR, Square::H1);
                self.remove_piece(PieceType::WR, Square::F1);
            }
            MoveFlag::WQCastle => {
                self.set_piece(PieceType::WR, Square::A1);
                self.remove_piece(PieceType::WR, Square::D1);
            }
            MoveFlag::BKCastle => {
                self.set_piece(PieceType::BR, Square::H8);
                self.remove_piece(PieceType::BR, Square::F8);
            }
            MoveFlag::BQCastle => {
                self.set_piece(PieceType::BR, Square::A8);
                self.remove_piece(PieceType::BR, Square::D8);
            }
            MoveFlag::PromoQ => {
                self.remove_piece(
                    match self.side {
                        Color::White => PieceType::WQ,
                        Color::Black => PieceType::BQ,
                    },
                    target,
                );
            }
            MoveFlag::PromoR => {
                self.remove_piece(
                    match self.side {
                        Color::White => PieceType::WR,
                        Color::Black => PieceType::BR,
                    },
                    target,
                );
            }
            MoveFlag::PromoN => {
                self.remove_piece(
                    match self.side {
                        Color::White => PieceType::WN,
                        Color::Black => PieceType::BN,
                    },
                    target,
                );
            }
            MoveFlag::PromoB => {
                self.remove_piece(
                    match self.side {
                        Color::White => PieceType::WB,
                        Color::Black => PieceType::BB,
                    },
                    target,
                );
            }
        };

        self.castling_rights = old_castling_rights;
        self.populate_occupancies();
    }

    #[inline(always)]
    pub fn is_square_attacked(&self, defending_side: Color, square: Square) -> bool {
        let &[enemy_pawn, enemy_knight, enemy_bishop, enemy_rook, enemy_queen, enemy_king] = match defending_side {
            Color::White => &PieceType::BLACK_PIECES,
            Color::Black => &PieceType::WHITE_PIECES,
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
            Color::White => self.is_square_attacked(defending_side, self.bbs[PieceType::WK].to_sq()),
            Color::Black => self.is_square_attacked(defending_side, self.bbs[PieceType::BK].to_sq()),
        }
    }

    #[inline(always)]
    #[cfg(feature = "board_representation_bitboard")]
    pub fn get_piece(&self, square: Square) -> PieceType {
        for piece_type in PieceType::ALL_PIECES {
            if self.bbs[piece_type].is_set_sq(square) {
                return piece_type;
            }
        }
        PieceType::None
    }

    #[inline(always)]
    #[cfg(feature = "board_representation_array")]
    pub fn get_piece(&self, square: Square) -> PieceType {
        self.pps[square]
    }
}

impl Default for Position {
    fn default() -> Position {
        Position {
            #[cfg(feature = "board_representation_array")]
            pps: [PieceType::None; 64],

            bbs: [Bitboard::EMPTY; 12],
            wo: Bitboard::EMPTY,
            bo: Bitboard::EMPTY,
            ao: Bitboard::EMPTY,
            side: Color::White,
            en_passant_sq: Square::None,
            castling_rights: CastlingRights::NONE,

            #[cfg(feature = "transposition_table")]
            zobrist_key: ZobristKey(0),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");
        for rank in 0..8_u8 {
            s += &format!("  {}  ", 8 - rank);
            for file in 0..8_u8 {
                let mut is_occupied = false;
                let sq = Square::from(rank * 8 + file);
                for piece_type in PieceType::ALL_PIECES {
                    if Bitboard::is_set_sq(&self.bbs[piece_type], sq) {
                        s += &format!("{} ", piece_type);
                        is_occupied = true;
                    }
                }
                if !is_occupied {
                    s += ". ";
                }
            }
            s += "\n";
        }
        s += &format!("\n     a b c d e f g h\n");

        #[cfg(feature = "transposition_table")]
        {
            s += &format!("
  Zobrist Key: {:#x}",
                self.zobrist_key.0
            );
        }

        s += &format!("
          FEN: {}
         Side: {}
   En-passant: {}
     Castling: {}\n",
            FenString::from(self),
            self.side,
            self.en_passant_sq,
            self.castling_rights,
        );
        
        f.pad(&s)
    }
}
