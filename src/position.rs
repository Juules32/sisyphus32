use core::fmt;
use std::collections::HashSet;
use crate::{bit_move::{BitMove, Move, ScoringMove}, bitboard::Bitboard, castling_rights::CastlingRights, color::Color, fen, move_flag::MoveFlag, move_list::MoveList, move_masks, piece::{self, PieceType}, rank::Rank, square::Square};

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
        self.side.switch();
        self.populate_occupancies();

        if self.is_square_attacked(
            if self.side == Color::White {
                self.bbs[PieceType::BK].to_sq()
            } else {
                self.bbs[PieceType::WK].to_sq()
            },
            self.side.opposite(),
            if self.side == Color::White {
                &PieceType::WHITE_PIECES
            } else {
                &PieceType::BLACK_PIECES
            },
        ) {
            return false;
        }

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
    pub fn is_square_attacked(
        &self,
        square: Square,
        defending_side: Color,
        [enemy_pawn, enemy_knight, enemy_bishop, enemy_rook, enemy_queen, enemy_king]: &[PieceType; 6]
    ) -> bool {
        if (move_masks::get_pawn_capture_mask(defending_side, square) & self.bbs[*enemy_pawn]).is_not_empty() {
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
            Color::White => self.is_square_attacked(self.bbs[PieceType::WK].to_sq(), Color::White, &PieceType::BLACK_PIECES),
            Color::Black => self.is_square_attacked(self.bbs[PieceType::BK].to_sq(), Color::Black, &PieceType::WHITE_PIECES),
        }
    }
    
    // Based on side, relevant pieces and occupancies can be selected
    #[inline]
    pub fn generate_moves<T: Move>(&self, add: fn(&Position, &mut MoveList<T>, BitMove)) -> MoveList<T> {
        let mut move_list = MoveList::default();
        
        let side = self.side;
        let en_passant_sq = self.en_passant_sq;
        let inv_all_occupancies = !self.ao;
        
        let ([pawn, knight, bishop, rook, queen, king], enemy_pieces) = match side {
            Color::White => (PieceType::WHITE_PIECES, PieceType::BLACK_PIECES),
            Color::Black => (PieceType::BLACK_PIECES, PieceType::WHITE_PIECES)
        };

        let (inv_own_occupancies, enemy_occupancies) = match side {
            Color::White => (!self.wo, self.bo),
            Color::Black => (!self.bo, self.wo)
        };
        
        let (pawn_promotion_rank, pawn_starting_rank, en_passant_rank, pawn_double_push_rank) = match side {
            Color::White => (Rank::R7, Rank::R2, Rank::R5, Rank::R4),
            Color::Black => (Rank::R2, Rank::R7, Rank::R4, Rank::R5)
        };
        
        let (double_pawn_flag, en_passant_flag, king_side_castling_flag, queen_side_castling_flag) = match side {
            Color::White => (MoveFlag::WDoublePawn, MoveFlag::WEnPassant, MoveFlag::WKCastle, MoveFlag::WQCastle),
            Color::Black => (MoveFlag::BDoublePawn, MoveFlag::BEnPassant, MoveFlag::BKCastle, MoveFlag::BQCastle)
        };

        let (king_side_castling_mask, queen_side_castling_mask) = match side {
            Color::White => (Bitboard::W_KING_SIDE_MASK, Bitboard::W_QUEEN_SIDE_MASK),
            Color::Black => (Bitboard::B_KING_SIDE_MASK, Bitboard::B_QUEEN_SIDE_MASK)
        };

        let (king_side_castling_right, queen_side_castling_right) = match side {
            Color::White => (self.castling_rights.wk(), self.castling_rights.wq()),
            Color::Black => (self.castling_rights.bk(), self.castling_rights.bq())
        };

        let (castling_square_c, castling_square_d, castling_square_e, castling_square_f, castling_square_g) = match side {
            Color::White => (Square::C1, Square::D1, Square::E1, Square::F1, Square::G1),
            Color::Black => (Square::C8, Square::D8, Square::E8, Square::F8, Square::G8)
        };

        {
            /*------------------------------*\ 
                        Pawn moves
            \*------------------------------*/
            let mut pawn_bb = self.bbs[pawn];
            while pawn_bb.is_not_empty() {
                let source = pawn_bb.pop_lsb();
                let source_rank = source.rank();

                // Captures
                let mut capture_mask = move_masks::get_pawn_capture_mask(side, source) & enemy_occupancies;
                while capture_mask.is_not_empty() {
                    let target = capture_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = self.get_target_piece(enemy_pieces, target);

                    if source_rank == pawn_promotion_rank {
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoN));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoN));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoB));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoB));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoR));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoR));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoQ));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoQ));
                    } else {
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::None));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                    }
                }

                // Quiet moves
                let mut quiet_mask = move_masks::get_pawn_quiet_mask(side, source) & inv_all_occupancies;
                while quiet_mask.is_not_empty() {
                    let target = quiet_mask.pop_lsb();
                    
                    if source_rank == pawn_starting_rank && target.rank() == pawn_double_push_rank {
                        // Making sure both squares in front of the pawn are empty
                        if (move_masks::get_pawn_quiet_mask(side, source) & self.ao).is_empty() {
                            
                            #[cfg(feature = "board_representation_bitboard")]
                            add(self, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, double_pawn_flag));

                            #[cfg(feature = "board_representation_array")]
                                add(self, &mut move_list, BitMove::encode(source, target, double_pawn_flag));
                        } 
                    } else if source_rank == pawn_promotion_rank {
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoN));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoN));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoB));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoB));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoR));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoR));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoQ));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoQ));
                    } else {
                        #[cfg(feature = "board_representation_bitboard")]
                        add(self, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::None));

                        #[cfg(feature = "board_representation_array")]
                        add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                    }
                }
                
                // En-passant (could maybe be combined with captures?)
                if en_passant_sq != Square::None && source_rank == en_passant_rank {
                    let mut en_passant_mask = move_masks::get_pawn_capture_mask(side, source);
                    while en_passant_mask.is_not_empty() {
                        let target = en_passant_mask.pop_lsb();
                        if target == en_passant_sq {
                            #[cfg(feature = "board_representation_bitboard")]
                            add(self, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, en_passant_flag));

                            #[cfg(feature = "board_representation_array")]
                            add(self, &mut move_list, BitMove::encode(source, target, en_passant_flag));
                        }
                    }
                }
            }
        }

        {
            /*------------------------------*\ 
                    Knight moves
            \*------------------------------*/
            let mut knight_bb = self.bbs[knight];
            while knight_bb.is_not_empty() {
                let source = knight_bb.pop_lsb();
                
                let mut move_mask = move_masks::get_knight_mask(source) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = self.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(self, &mut move_list, BitMove::encode(source, target, knight, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        King moves
            \*------------------------------*/
            let mut king_bb = self.bbs[king];
            let source = king_bb.pop_lsb();
            let mut move_mask = move_masks::get_king_mask(source) & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();

                #[cfg(feature = "board_representation_bitboard")]
                let target_piece = self.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                
                #[cfg(feature = "board_representation_bitboard")]
                add(self, &mut move_list, BitMove::encode(source, target, king, target_piece, MoveFlag::None));

                #[cfg(feature = "board_representation_array")]
                add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
            }

            // Kingside Castling
            #[allow(clippy::collapsible_if)]
            if king_side_castling_right && (self.ao & king_side_castling_mask).is_empty() {
                if !self.is_square_attacked(castling_square_e, self.side, &enemy_pieces) &&
                !self.is_square_attacked(castling_square_f, self.side, &enemy_pieces) &&
                !self.is_square_attacked(castling_square_g, self.side, &enemy_pieces)
                {
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(self, &mut move_list, BitMove::encode(source, castling_square_g, king, PieceType::None, king_side_castling_flag));

                    #[cfg(feature = "board_representation_array")]
                    add(self, &mut move_list, BitMove::encode(source, castling_square_g, king_side_castling_flag));
                }
            }

            // Queenside Castling
            #[allow(clippy::collapsible_if)]
            if queen_side_castling_right && (self.ao & queen_side_castling_mask).is_empty() {
                if !self.is_square_attacked(castling_square_e, self.side, &enemy_pieces) &&
                !self.is_square_attacked(castling_square_d, self.side, &enemy_pieces) &&
                !self.is_square_attacked(castling_square_c, self.side, &enemy_pieces)
                {
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(self, &mut move_list, BitMove::encode(source, castling_square_c, king, PieceType::None, queen_side_castling_flag));

                    #[cfg(feature = "board_representation_array")]
                    add(self, &mut move_list, BitMove::encode(source, castling_square_c, queen_side_castling_flag));
                }
            }
        }

        {
            /*------------------------------*\ 
                    Bishop moves
            \*------------------------------*/
            let mut bishop_bb = self.bbs[bishop];
            while bishop_bb.is_not_empty() {
                let source = bishop_bb.pop_lsb();
                let mut move_mask = move_masks::get_bishop_mask(source, self.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = self.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(self, &mut move_list, BitMove::encode(source, target, bishop, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        Rook moves
            \*------------------------------*/
            let mut rook_bb = self.bbs[rook];
            while rook_bb.is_not_empty() {
                let source = rook_bb.pop_lsb();
                let mut move_mask = move_masks::get_rook_mask(source, self.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = self.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(self, &mut move_list, BitMove::encode(source, target, rook, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                    Queen moves
            \*------------------------------*/
            let mut queen_bb = self.bbs[queen];
            while queen_bb.is_not_empty() {
                let source = queen_bb.pop_lsb();
                let mut move_mask = move_masks::get_queen_mask(source, self.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = self.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(self, &mut move_list, BitMove::encode(source, target, queen, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    add(self, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }
        
        // Checks that all moves are unique
        debug_assert!({
            let mut seen: HashSet<T> = HashSet::new();
            move_list.iter().all(|&m| seen.insert(m))
        });
        
        move_list
    }

    #[inline]
    pub fn generate_pseudo_legal_moves(&self) -> MoveList<BitMove> {
        self.generate_moves::<BitMove>(|_position, move_list, bit_move| {
            move_list.add(bit_move);
        })
    }

    // NOTE: This function is inefficient for perft and move ordering.
    // generate_pseudo_legal_moves() is faster in those cases.
    #[inline]
    pub fn generate_legal_moves(&self) -> MoveList<BitMove> {
        self.generate_moves::<BitMove>(|position, move_list, bit_move| {
            let mut position_copy = position.clone();
            if position_copy.make_move(bit_move) {
                move_list.add(bit_move);
            }
        })
    }

    #[inline]
    pub fn generate_pseudo_legal_scoring_moves(&self) -> MoveList<ScoringMove> {
        self.generate_moves::<ScoringMove>(|_position, move_list, bit_move| {
            move_list.add(ScoringMove::from(bit_move));
        })
    }

    #[inline]
    pub fn generate_legal_scoring_moves(&self) -> MoveList<ScoringMove> {
        self.generate_moves::<ScoringMove>(|position, move_list, bit_move| {
            let mut position_copy = position.clone();
            if position_copy.make_move(bit_move) {
                move_list.add(ScoringMove::from(bit_move));
            }
        })
    }

    #[inline(always)]
    #[cfg(feature = "board_representation_bitboard")]
    fn get_piece(&self, square: Square) -> PieceType {
        for piece_type in PieceType::ALL_PIECES {
            if self.bbs[piece_type].is_set_sq(square) {
                return piece_type
            }
        }
        PieceType::None
    }

    #[inline(always)]
    #[cfg(feature = "board_representation_array")]
    fn get_piece(&self, square: Square) -> PieceType {
        self.pps[square]
    }

    #[inline(always)]
    pub fn get_target_piece(&self, enemy_piece_types: [PieceType; 6], target: Square) -> PieceType {
        for piece_type in enemy_piece_types {
            if self.bbs[piece_type].is_set_sq(target) {
                return piece_type;
            }
        }

        panic!("There seems to be something wrong with the occupancy bitboards!")
    }

    #[inline(always)]
    pub fn get_target_piece_if_any(&self, enemy_piece_types: [PieceType; 6], enemy_occupancies: Bitboard, target: Square) -> PieceType {
        if (enemy_occupancies & target.to_bb()).is_empty() {
            return PieceType::None;
        }
        
        self.get_target_piece(enemy_piece_types, target)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_pseudo_legal_moves_returns_unique_moves() {
        let move_list = Position::starting_position().generate_pseudo_legal_moves();
        let mut seen = HashSet::new();
        assert!(move_list.iter().all(|&m| seen.insert(m)));
    }

    #[test]
    fn false_test() {
        assert!(false)
    }
}
