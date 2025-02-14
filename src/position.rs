use core::fmt;
use crate::{bit_move::BitMove, bitboard::Bitboard, castling_rights::CastlingRights, color::Color, move_flag::MoveFlag, move_masks, piece::PieceType, square::Square};

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
        Position {
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
        }
    }

    #[inline(always)]
    pub fn set_piece(&mut self, piece: PieceType, sq: Square) {
        self.bbs[piece].set_sq(sq);

        #[cfg(feature = "board_representation_array")]
        { self.pps[sq] = piece; }
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, piece: PieceType, sq: Square) {
        self.bbs[piece].pop_sq(sq);

        #[cfg(feature = "board_representation_array")]
        { self.pps[sq] = PieceType::None; }
    }

    #[inline]
    pub fn make_move(&mut self, bit_move: BitMove) -> bool {
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

        // Moves piece
        self.remove_piece(piece, source);
        self.set_piece(piece, target);

        // Removes captured piece
        if capture != PieceType::None {
            #[cfg(feature = "board_representation_bitboard")]
            self.remove_piece(capture, target);
            
            #[cfg(feature = "board_representation_array")]
            self.bbs[capture].pop_sq(target);
        }

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

        if self.in_check() {
            self.side.switch();
            return false;
        }

        self.side.switch();
        true
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
    pub fn is_square_attacked(&self, square: Square) -> bool {
        let [enemy_pawn, enemy_knight, enemy_bishop, enemy_rook, enemy_queen, enemy_king] = match self.side {
            Color::White => &PieceType::BLACK_PIECES,
            Color::Black => &PieceType::WHITE_PIECES,
        };

        if (move_masks::get_pawn_capture_mask(self.side, square) & self.bbs[*enemy_pawn]).is_not_empty() {
            return true;
        }
        if (move_masks::get_knight_mask(square) & self.bbs[*enemy_knight]).is_not_empty() {
            return true;
        }
        if (move_masks::get_bishop_mask(square, self.ao) & self.bbs[*enemy_bishop]).is_not_empty() {
            return true;
        }
        if (move_masks::get_rook_mask(square, self.ao) & self.bbs[*enemy_rook]).is_not_empty() {
            return true;
        }
        if (move_masks::get_queen_mask(square, self.ao) & self.bbs[*enemy_queen]).is_not_empty() {
            return true;
        }
        if (move_masks::get_king_mask(square) & self.bbs[*enemy_king]).is_not_empty() {
            return true;
        }
        false
    }

    pub fn in_check(&self) -> bool {
        match self.side {
            Color::White => self.is_square_attacked(self.bbs[PieceType::WK].to_sq()),
            Color::Black => self.is_square_attacked(self.bbs[PieceType::BK].to_sq()),
        }
    }

    #[inline(always)]
    #[cfg(feature = "board_representation_bitboard")]
    pub fn get_piece(&self, square: Square) -> PieceType {
        for piece_type in PieceType::ALL_PIECES {
            if self.bbs[piece_type].is_set_sq(square) {
                return piece_type
            }
        }
        PieceType::None
    }

    #[inline(always)]
    #[cfg(feature = "board_representation_array")]
    pub fn get_piece(&self, square: Square) -> PieceType {
        self.pps[square]
    }

    fn to_fen_string(&self) -> String {
        let mut fen_str = String::new();
        let mut curr_width = 0;
        let mut curr_empty = 0;
        for square in Square::ALL_SQUARES {
            curr_width += 1;

            let piece_type = self.get_piece(square);
            match piece_type {
                PieceType::None => curr_empty += 1,
                _ => {
                    if curr_empty != 0 {
                        fen_str.push_str(&curr_empty.to_string());
                        curr_empty = 0;
                    }
                    fen_str.push(piece_type.into())
                }
            }


            if curr_width == 8 {
                if curr_empty != 0 {
                    fen_str.push_str(&curr_empty.to_string());
                }

                if square != *Square::ALL_SQUARES.last().unwrap() {
                    fen_str.push('/');
                }

                curr_empty = 0;
                curr_width = 0;
            }
        }

        fen_str.push(' ');

        fen_str.push(
            match self.side {
                Color::White => 'w',
                Color::Black => 'b',
            }
        );

        fen_str.push(' ');

        fen_str.push_str(&self.castling_rights.to_string());

        fen_str.push(' ');

        fen_str.push_str(
            &match self.en_passant_sq {
                Square::None => "-".to_owned(),
                _ => self.en_passant_sq.to_string(),
            }
        );
        fen_str
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
        s += &format!(
            "
     a b c d e f g h

  FEN:        {}
  Side        {}
  En-passant: {}
  Castling:   {}\n",
            self.to_fen_string(),
            self.side,
            self.en_passant_sq,
            self.castling_rights
        );
        f.pad(&s)
    }
}
