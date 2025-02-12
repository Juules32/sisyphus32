use std::collections::HashSet;

use crate::{bit_move::{BitMove, Move, ScoringMove}, bitboard::Bitboard, color::Color, move_flag::MoveFlag, move_list::MoveList, move_masks, piece::PieceType, position::Position, rank::Rank, square::Square};

pub struct MoveGeneration { }

impl MoveGeneration {
    // Based on side, relevant pieces and occupancies can be selected
    #[inline]
    pub fn generate_moves<T: Move>(position: &Position, add: fn(&Position, &mut MoveList<T>, BitMove)) -> MoveList<T> {
        let mut move_list = MoveList::new();
        
        let side = position.side;
        let en_passant_sq = position.en_passant_sq;
        let inv_all_occupancies = !position.ao;
        
        let ([pawn, knight, bishop, rook, queen, king], enemy_pieces) = match side {
            Color::White => (PieceType::WHITE_PIECES, PieceType::BLACK_PIECES),
            Color::Black => (PieceType::BLACK_PIECES, PieceType::WHITE_PIECES)
        };

        let (inv_own_occupancies, enemy_occupancies) = match side {
            Color::White => (!position.wo, position.bo),
            Color::Black => (!position.bo, position.wo)
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
            Color::White => (position.castling_rights.wk(), position.castling_rights.wq()),
            Color::Black => (position.castling_rights.bk(), position.castling_rights.bq())
        };

        let (castling_square_c, castling_square_d, castling_square_e, castling_square_f, castling_square_g) = match side {
            Color::White => (Square::C1, Square::D1, Square::E1, Square::F1, Square::G1),
            Color::Black => (Square::C8, Square::D8, Square::E8, Square::F8, Square::G8)
        };

        {
            /*------------------------------*\ 
                        Pawn moves
            \*------------------------------*/
            let mut pawn_bb = position.bbs[pawn];
            while pawn_bb.is_not_empty() {
                let source = pawn_bb.pop_lsb();
                let source_rank = source.rank();

                // Captures
                let mut capture_mask = move_masks::get_pawn_capture_mask(side, source) & enemy_occupancies;
                while capture_mask.is_not_empty() {
                    let target = capture_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_target_piece(enemy_pieces, target);

                    if source_rank == pawn_promotion_rank {
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoN));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoN));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoB));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoB));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoR));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoR));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoQ));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoQ));
                    } else {
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, target_piece, MoveFlag::None));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                    }
                }

                // Quiet moves
                let mut quiet_mask = move_masks::get_pawn_quiet_mask(side, source) & inv_all_occupancies;
                while quiet_mask.is_not_empty() {
                    let target = quiet_mask.pop_lsb();
                    
                    if source_rank == pawn_starting_rank && target.rank() == pawn_double_push_rank {
                        // Making sure both squares in front of the pawn are empty
                        if (move_masks::get_pawn_quiet_mask(side, source) & position.ao).is_empty() {
                            
                            #[cfg(feature = "board_representation_bitboard")]
                            add(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, double_pawn_flag));

                            #[cfg(feature = "board_representation_array")]
                                add(position, &mut move_list, BitMove::encode(source, target, double_pawn_flag));
                        } 
                    } else if source_rank == pawn_promotion_rank {
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoN));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoN));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoB));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoB));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoR));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoR));
                        
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoQ));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::PromoQ));
                    } else {
                        #[cfg(feature = "board_representation_bitboard")]
                        add(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::None));

                        #[cfg(feature = "board_representation_array")]
                        add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                    }
                }
                
                // En-passant (could maybe be combined with captures?)
                if en_passant_sq != Square::None && source_rank == en_passant_rank {
                    let mut en_passant_mask = move_masks::get_pawn_capture_mask(side, source);
                    while en_passant_mask.is_not_empty() {
                        let target = en_passant_mask.pop_lsb();
                        if target == en_passant_sq {
                            #[cfg(feature = "board_representation_bitboard")]
                            add(position, &mut move_list, BitMove::encode(source, target, pawn, PieceType::None, en_passant_flag));

                            #[cfg(feature = "board_representation_array")]
                            add(position, &mut move_list, BitMove::encode(source, target, en_passant_flag));
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
                
                let mut move_mask = move_masks::get_knight_mask(source) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(position, &mut move_list, BitMove::encode(source, target, knight, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        King moves
            \*------------------------------*/
            let mut king_bb = position.bbs[king];
            let source = king_bb.pop_lsb();
            let mut move_mask = move_masks::get_king_mask(source) & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();

                #[cfg(feature = "board_representation_bitboard")]
                let target_piece = position.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                
                #[cfg(feature = "board_representation_bitboard")]
                add(position, &mut move_list, BitMove::encode(source, target, king, target_piece, MoveFlag::None));

                #[cfg(feature = "board_representation_array")]
                add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
            }

            // Kingside Castling
            #[allow(clippy::collapsible_if)]
            if king_side_castling_right && (position.ao & king_side_castling_mask).is_empty() {
                if !position.is_square_attacked(castling_square_e, position.side, &enemy_pieces) &&
                !position.is_square_attacked(castling_square_f, position.side, &enemy_pieces) &&
                !position.is_square_attacked(castling_square_g, position.side, &enemy_pieces)
                {
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(position, &mut move_list, BitMove::encode(source, castling_square_g, king, PieceType::None, king_side_castling_flag));

                    #[cfg(feature = "board_representation_array")]
                    add(position, &mut move_list, BitMove::encode(source, castling_square_g, king_side_castling_flag));
                }
            }

            // Queenside Castling
            #[allow(clippy::collapsible_if)]
            if queen_side_castling_right && (position.ao & queen_side_castling_mask).is_empty() {
                if !position.is_square_attacked(castling_square_e, position.side, &enemy_pieces) &&
                !position.is_square_attacked(castling_square_d, position.side, &enemy_pieces) &&
                !position.is_square_attacked(castling_square_c, position.side, &enemy_pieces)
                {
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(position, &mut move_list, BitMove::encode(source, castling_square_c, king, PieceType::None, queen_side_castling_flag));

                    #[cfg(feature = "board_representation_array")]
                    add(position, &mut move_list, BitMove::encode(source, castling_square_c, queen_side_castling_flag));
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
                let mut move_mask = move_masks::get_bishop_mask(source, position.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(position, &mut move_list, BitMove::encode(source, target, bishop, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
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
                let mut move_mask = move_masks::get_rook_mask(source, position.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(position, &mut move_list, BitMove::encode(source, target, rook, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
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
                let mut move_mask = move_masks::get_queen_mask(source, position.ao) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "board_representation_bitboard")]
                    let target_piece = position.get_target_piece_if_any(enemy_pieces, enemy_occupancies, target);
                    
                    #[cfg(feature = "board_representation_bitboard")]
                    add(position, &mut move_list, BitMove::encode(source, target, queen, target_piece, MoveFlag::None));

                    #[cfg(feature = "board_representation_array")]
                    add(position, &mut move_list, BitMove::encode(source, target, MoveFlag::None));
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
    pub fn generate_pseudo_legal_moves(position: &Position) -> MoveList<BitMove> {
        Self::generate_moves::<BitMove>(position, |_position, move_list, bit_move| {
            move_list.add(bit_move);
        })
    }

    // NOTE: This function is inefficient for perft and move ordering.
    // generate_pseudo_legal_moves() is faster in those cases.
    #[inline]
    pub fn generate_legal_moves(position: &Position) -> MoveList<BitMove> {
        Self::generate_moves::<BitMove>(position, |position, move_list, bit_move| {
            let mut position_copy = position.clone();
            if position_copy.make_move(bit_move) {
                move_list.add(bit_move);
            }
        })
    }

    #[inline]
    pub fn generate_pseudo_legal_scoring_moves(position: &Position) -> MoveList<ScoringMove> {
        Self::generate_moves::<ScoringMove>(position, |_position, move_list, bit_move| {
            move_list.add(ScoringMove::from(bit_move));
        })
    }

    #[inline]
    pub fn generate_legal_scoring_moves(position: &Position) -> MoveList<ScoringMove> {
        Self::generate_moves::<ScoringMove>(position, |position, move_list, bit_move| {
            let mut position_copy = position.clone();
            if position_copy.make_move(bit_move) {
                move_list.add(ScoringMove::from(bit_move));
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn generate_pseudo_legal_moves_returns_unique_moves() {
        let move_list = MoveGeneration::generate_pseudo_legal_moves(&Position::starting_position());
        let mut seen = HashSet::new();
        assert!(move_list.iter().all(|&m| seen.insert(m)));
    }
}
