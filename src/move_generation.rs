use crate::{bit_move::{BitMove, Move}, bitboard::Bitboard, color::Color, move_flag::MoveFlag, move_list::MoveList, move_masks::MoveMasks, piece::PieceType, position::Position, rank::Rank, square::Square};

pub struct MoveGeneration { }

impl MoveGeneration {
    #[inline]
    pub fn generate_moves<T: Move, F: Filter>(position: &Position) -> MoveList<T> {
        let mut move_list = MoveList::new();
        
        let side = position.side;
        let en_passant_sq = position.en_passant_sq;
        let inv_all_occupancies = !position.ao;
        
        let ([pawn, knight, bishop, rook, queen, king], inv_own_occupancies) = match side {
            Color::White => (PieceType::WHITE_PIECES, !position.wo),
            Color::Black => (PieceType::BLACK_PIECES, !position.bo),
        };

        {
            /*------------------------------*\ 
                        Pawn moves
            \*------------------------------*/
            let (pawn_promotion_rank, pawn_starting_rank, en_passant_rank, pawn_double_push_rank, double_pawn_flag, en_passant_flag, enemy_occupancies) = match side {
                Color::White => (Rank::R7, Rank::R2, Rank::R5, Rank::R4, MoveFlag::WDoublePawn, MoveFlag::WEnPassant, position.bo),
                Color::Black => (Rank::R2, Rank::R7, Rank::R4, Rank::R5, MoveFlag::BDoublePawn, MoveFlag::BEnPassant, position.wo),
            };

            let mut pawn_bb = position.bbs[pawn];
            while pawn_bb.is_not_empty() {
                let source = pawn_bb.pop_lsb();
                let source_rank = source.rank();

                // Captures
                let mut capture_mask = MoveMasks::get_pawn_capture_mask(side, source) & enemy_occupancies;
                while capture_mask.is_not_empty() {
                    let target = capture_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);

                    if source_rank == pawn_promotion_rank {
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoN));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoN));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoB));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoB));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoR));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoR));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoQ));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoQ));
                    } else {
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::None));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                    }
                }

                // Quiet moves
                let mut quiet_mask = MoveMasks::get_pawn_quiet_mask(side, source) & inv_all_occupancies;
                while quiet_mask.is_not_empty() {
                    let target = quiet_mask.pop_lsb();
                    
                    if source_rank == pawn_starting_rank && target.rank() == pawn_double_push_rank {
                        // Making sure both squares in front of the pawn are empty
                        if (MoveMasks::get_pawn_quiet_mask(side, source) & position.ao).is_empty() {
                            
                            #[cfg(feature = "board_representation_bitboard")]
                            Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, double_pawn_flag));

                            #[cfg(feature = "board_representation_array")]
                                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, double_pawn_flag));
                        } 
                    } else if source_rank == pawn_promotion_rank {
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoN));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoN));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoB));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoB));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoR));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoR));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoQ));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoQ));
                    } else {
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::None));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                    }
                }
                
                // En-passant
                if en_passant_sq != Square::None && source_rank == en_passant_rank {
                    let mut en_passant_mask = MoveMasks::get_pawn_capture_mask(side, source);
                    while en_passant_mask.is_not_empty() {
                        let target = en_passant_mask.pop_lsb();
                        if target == en_passant_sq {
                            #[cfg(feature = "board_representation_bitboard")]
                            Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, en_passant_flag));

                            #[cfg(feature = "board_representation_array")]
                            Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, en_passant_flag));
                        }
                    }
                }
            }
        }

        {
            /*------------------------------*\ 
                    Knight moves
            \*------------------------------*/
            let mut knight_bb = position.bbs[knight];
            while knight_bb.is_not_empty() {
                let source = knight_bb.pop_lsb();
                
                let mut move_mask = MoveMasks::get_knight_mask(source) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, knight, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        King moves
            \*------------------------------*/
            let (
                king_side_castling_flag, queen_side_castling_flag,
                king_side_castling_mask, queen_side_castling_mask,
                king_side_castling_right, queen_side_castling_right,
                castling_square_c, castling_square_d, castling_square_e, castling_square_f, castling_square_g
            ) = match side {
                Color::White => (
                    MoveFlag::WKCastle, MoveFlag::WQCastle,
                    Bitboard::W_KING_SIDE_MASK, Bitboard::W_QUEEN_SIDE_MASK,
                    position.castling_rights.wk(), position.castling_rights.wq(),
                    Square::C1, Square::D1, Square::E1, Square::F1, Square::G1
                ),
                Color::Black => (
                    MoveFlag::BKCastle, MoveFlag::BQCastle,
                    Bitboard::B_KING_SIDE_MASK, Bitboard::B_QUEEN_SIDE_MASK,
                    position.castling_rights.bk(), position.castling_rights.bq(),
                    Square::C8, Square::D8, Square::E8, Square::F8, Square::G8
                ),
            };

            let mut king_bb = position.bbs[king];
            let source = king_bb.pop_lsb();
            let mut move_mask = MoveMasks::get_king_mask(source) & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();

                #[cfg(feature = "board_representation_bitboard")]
                let target_piece = position.get_piece(target);
                
                #[cfg(feature = "board_representation_bitboard")]
                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, king, target_piece, MoveFlag::None));

                #[cfg(feature = "board_representation_array")]
                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
            }

            // Kingside Castling
            #[allow(clippy::collapsible_if)]
            if king_side_castling_right && (position.ao & king_side_castling_mask).is_empty() {
                if !position.is_square_attacked(castling_square_e) &&
                !position.is_square_attacked(castling_square_f) &&
                !position.is_square_attacked(castling_square_g)
                {
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, castling_square_g, king, PieceType::None, king_side_castling_flag));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, castling_square_g, king_side_castling_flag));
                }
            }

            // Queenside Castling
            #[allow(clippy::collapsible_if)]
            if queen_side_castling_right && (position.ao & queen_side_castling_mask).is_empty() {
                if !position.is_square_attacked(castling_square_e) &&
                !position.is_square_attacked(castling_square_d) &&
                !position.is_square_attacked(castling_square_c)
                {
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, castling_square_c, king, PieceType::None, queen_side_castling_flag));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, castling_square_c, queen_side_castling_flag));
                }
            }
        }

        {
            /*------------------------------*\ 
                    Bishop moves
            \*------------------------------*/
            let mut bishop_bb = position.bbs[bishop];
            while bishop_bb.is_not_empty() {
                let source = bishop_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_bishop_mask(source, position.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, bishop, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        Rook moves
            \*------------------------------*/
            let mut rook_bb = position.bbs[rook];
            while rook_bb.is_not_empty() {
                let source = rook_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_rook_mask(source, position.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, rook, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                    Queen moves
            \*------------------------------*/
            let mut queen_bb = position.bbs[queen];
            while queen_bb.is_not_empty() {
                let source = queen_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_queen_mask(source, position.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, queen, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }
        
        move_list
    }

    fn add_move<T: Move, F: Filter>(position: &Position, move_list: &mut MoveList<T>, bit_move: BitMove) {
        if F::should_add(position, bit_move) {
            move_list.add(T::new(position, bit_move));
        }
    }

    #[inline]
    pub fn generate_captures<T: Move, F: Filter>(position: &Position) -> MoveList<T> {
        let mut move_list = MoveList::new();
        
        let side = position.side;
        let en_passant_sq = position.en_passant_sq;
        
        let ([pawn, knight, bishop, rook, queen, king], enemy_occupancies) = match side {
            Color::White => (PieceType::WHITE_PIECES, position.bo),
            Color::Black => (PieceType::BLACK_PIECES, position.wo),
        };

        {
            /*------------------------------*\ 
                        Pawn moves
            \*------------------------------*/
            let (pawn_promotion_rank, en_passant_rank, en_passant_flag) = match side {
                Color::White => (Rank::R7, Rank::R5, MoveFlag::WEnPassant),
                Color::Black => (Rank::R2, Rank::R4, MoveFlag::BEnPassant),
            };

            let mut pawn_bb = position.bbs[pawn];
            while pawn_bb.is_not_empty() {
                let source = pawn_bb.pop_lsb();
                let source_rank = source.rank();

                // Captures
                let mut capture_mask = MoveMasks::get_pawn_capture_mask(side, source) & enemy_occupancies;
                while capture_mask.is_not_empty() {
                    let target = capture_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);

                    if source_rank == pawn_promotion_rank {
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoN));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoN));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoB));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoB));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoR));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoR));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoQ));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoQ));
                    } else {
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::None));

                        #[cfg(feature = "board_representation_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                    }
                }

                #[cfg(feature = "en_passant_in_quiescence")]
                // En-passant
                if en_passant_sq != Square::None && source_rank == en_passant_rank {
                    let mut en_passant_mask = MoveMasks::get_pawn_capture_mask(side, source);
                    while en_passant_mask.is_not_empty() {
                        let target = en_passant_mask.pop_lsb();
                        if target == en_passant_sq {
                            #[cfg(feature = "board_representation_bitboard")]
                            Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, en_passant_flag));

                            #[cfg(feature = "board_representation_array")]
                            Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, en_passant_flag));
                        }
                    }
                }
            }
        }

        {
            /*------------------------------*\ 
                    Knight moves
            \*------------------------------*/
            let mut knight_bb = position.bbs[knight];
            while knight_bb.is_not_empty() {
                let source = knight_bb.pop_lsb();
                
                let mut move_mask = MoveMasks::get_knight_mask(source) & enemy_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, knight, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        King moves
            \*------------------------------*/
            let mut king_bb = position.bbs[king];
            let source = king_bb.pop_lsb();
            let mut move_mask = MoveMasks::get_king_mask(source) & enemy_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();

                #[cfg(feature = "board_representation_bitboard")]
                let target_piece = position.get_piece(target);
                
                #[cfg(feature = "board_representation_bitboard")]
                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, king, target_piece, MoveFlag::None));

                #[cfg(feature = "board_representation_array")]
                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
            }
        }

        {
            /*------------------------------*\ 
                    Bishop moves
            \*------------------------------*/
            let mut bishop_bb = position.bbs[bishop];
            while bishop_bb.is_not_empty() {
                let source = bishop_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_bishop_mask(source, position.ao) & enemy_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, bishop, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        Rook moves
            \*------------------------------*/
            let mut rook_bb = position.bbs[rook];
            while rook_bb.is_not_empty() {
                let source = rook_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_rook_mask(source, position.ao) & enemy_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, rook, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                    Queen moves
            \*------------------------------*/
            let mut queen_bb = position.bbs[queen];
            while queen_bb.is_not_empty() {
                let source = queen_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_queen_mask(source, position.ao) & enemy_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_piece(target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, queen, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }
        
        move_list
    }
}

pub trait Filter {
    fn should_add(position: &Position, bit_move: BitMove) -> bool;
}

pub struct PseudoLegal { }
impl Filter for PseudoLegal {
    #[inline(always)]
    fn should_add(_: &Position, _: BitMove) -> bool {
        true
    }
}

pub struct Legal;
impl Filter for Legal {
    #[inline(always)]
    fn should_add(position: &Position, bit_move: BitMove) -> bool {
        let mut position_copy = position.clone();
        position_copy.make_move(bit_move.get_bit_move())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn generate_pseudo_legal_moves_returns_unique_moves() {
        let move_list = MoveGeneration::generate_moves::<BitMove, PseudoLegal>(&Position::starting_position());
        let mut seen = HashSet::new();
        assert!(move_list.iter().all(|&m| seen.insert(m)));
    }
}
