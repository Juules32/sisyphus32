use crate::{bit_move::{BitMove, MoveFlag}, bitboard::Bitboard, position::Position, color::Color, move_init, move_list::MoveList, piece::PieceType, rank::Rank, square::Square};

#[inline(always)]
pub fn get_pawn_quiet_mask(color: Color, square: Square) -> Bitboard {
    unsafe { move_init::PAWN_QUIET_MASKS[color][square] }
}

#[inline(always)]
pub fn get_pawn_capture_mask(color: Color, square: Square) -> Bitboard {
    unsafe { move_init::PAWN_CAPTURE_MASKS[color][square] }
}

#[inline(always)]
pub fn get_knight_mask(square: Square) -> Bitboard {
    unsafe { move_init::KNIGHT_MASKS[square] }
}

#[inline(always)]
pub fn get_king_mask(square: Square) -> Bitboard {
    unsafe { move_init::KING_MASKS[square] }
}

#[inline(always)]
pub fn get_bishop_mask_old(square: Square, occupancy: Bitboard) -> Bitboard {
    move_init::generate_bishop_moves_on_the_fly(square, occupancy)
}

#[inline(always)]
pub fn get_bishop_mask(square: Square, occupancy: Bitboard) -> Bitboard {
    unsafe {
        let mut index = occupancy.0 & move_init::BISHOP_MASKS[square].0;
        index = 
            index.wrapping_mul(move_init::BISHOP_MAGIC_BITBOARDS[square].0) >> 
            (64 - move_init::BISHOP_RELEVANT_BITS[square]);
        move_init::BISHOP_MOVE_CONFIGURATIONS[square][index as usize]
    }
}

#[inline(always)]
pub fn get_rook_mask_old(square: Square, occupancy: Bitboard) -> Bitboard {
    move_init::generate_rook_moves_on_the_fly(square, occupancy)
}

#[inline(always)]
pub fn get_rook_mask(square: Square, occupancy: Bitboard) -> Bitboard {
    unsafe {
        let mut index = occupancy.0 & move_init::ROOK_MASKS[square].0;
        index = 
            index.wrapping_mul(move_init::ROOK_MAGIC_BITBOARDS[square].0) >> 
            (64 - move_init::ROOK_RELEVANT_BITS[square]);
        move_init::ROOK_MOVE_CONFIGURATIONS[square][index as usize]
    }
}

#[inline(always)]
pub fn get_queen_mask_old(square: Square, occupancy: Bitboard) -> Bitboard {
    get_bishop_mask_old(square, occupancy) | get_rook_mask_old(square, occupancy)
}

#[inline(always)]
pub fn get_queen_mask(square: Square, occupancy: Bitboard) -> Bitboard {
    get_bishop_mask(square, occupancy) | get_rook_mask(square, occupancy)
}

// Based on side, relevant pieces and occupancies can be selected
#[inline]
pub fn generate_moves(position: &Position) -> MoveList {
    let mut move_list = MoveList::default();
    
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
            let mut capture_mask = get_pawn_capture_mask(side, source) & enemy_occupancies;
            while capture_mask.is_not_empty() {
                let target = capture_mask.pop_lsb();
                let target_piece = get_target_piece(position, enemy_pieces, target);

                if source_rank == pawn_promotion_rank {
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoN));
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoB));
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoR));
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoQ));
                } else {
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::None));
                }
            }

            // Quiet moves
            let mut quiet_mask = get_pawn_quiet_mask(side, source) & inv_all_occupancies;
            while quiet_mask.is_not_empty() {
                let target = quiet_mask.pop_lsb();
                
                if source_rank == pawn_starting_rank && target.rank() == pawn_double_push_rank {
                    // Making sure both squares in front of the pawn are empty
                    if (get_pawn_quiet_mask(side, source) & position.ao).is_empty() {
                        move_list.add(BitMove::encode(source, target, pawn, PieceType::None, double_pawn_flag));
                    } 
                } else if source_rank == pawn_promotion_rank {
                    move_list.add(BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoN));
                    move_list.add(BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoB));
                    move_list.add(BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoR));
                    move_list.add(BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::PromoQ));
                } else {
                    move_list.add(BitMove::encode(source, target, pawn, PieceType::None, MoveFlag::None));
                }
            }
            
            // En-passant (could maybe be combined with captures?)
            if en_passant_sq != Square::None && source_rank == en_passant_rank {
                let mut en_passant_mask = get_pawn_capture_mask(side, source);
                while en_passant_mask.is_not_empty() {
                    let target = en_passant_mask.pop_lsb();
                    if target == en_passant_sq {
                        move_list.add(BitMove::encode(source, target, pawn, PieceType::None, en_passant_flag));
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
            
            let mut move_mask = get_knight_mask(source) & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();
                let target_piece = get_target_piece_if_any(position, enemy_pieces, enemy_occupancies, target);
                move_list.add(BitMove::encode(source, target, knight, target_piece, MoveFlag::None));
            }
        }
    }

    {
        /*------------------------------*\ 
                    King moves
        \*------------------------------*/
        let mut king_bb = position.bbs[king];
        let source = king_bb.pop_lsb();
        let mut move_mask = get_king_mask(source) & inv_own_occupancies;
        while move_mask.is_not_empty() {
            let target = move_mask.pop_lsb();
            let target_piece = get_target_piece_if_any(position, enemy_pieces, enemy_occupancies, target);
            move_list.add(BitMove::encode(source, target, king, target_piece, MoveFlag::None));
        }

        // Kingside Castling
        #[allow(clippy::collapsible_if)]
        if king_side_castling_right && (position.ao & king_side_castling_mask).is_empty() {
            if !position.is_square_attacked(castling_square_e, position.side, &enemy_pieces) &&
               !position.is_square_attacked(castling_square_f, position.side, &enemy_pieces) &&
               !position.is_square_attacked(castling_square_g, position.side, &enemy_pieces)
            {
                move_list.add(BitMove::encode(source, castling_square_g, king, PieceType::None, king_side_castling_flag));
            }
        }

        // Queenside Castling
        #[allow(clippy::collapsible_if)]
        if queen_side_castling_right && (position.ao & queen_side_castling_mask).is_empty() {
            if !position.is_square_attacked(castling_square_e, position.side, &enemy_pieces) &&
               !position.is_square_attacked(castling_square_d, position.side, &enemy_pieces) &&
               !position.is_square_attacked(castling_square_c, position.side, &enemy_pieces)
            {
                move_list.add(BitMove::encode(source, castling_square_c, king, PieceType::None, queen_side_castling_flag));
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
            let mut move_mask = get_bishop_mask(source, position.ao) & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();
                let target_piece = get_target_piece_if_any(position, enemy_pieces, enemy_occupancies, target);
                move_list.add(BitMove::encode(source, target, bishop, target_piece, MoveFlag::None));
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
            let mut move_mask = get_rook_mask(source, position.ao) & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();
                let target_piece = get_target_piece_if_any(position, enemy_pieces, enemy_occupancies, target);
                move_list.add(BitMove::encode(source, target, rook, target_piece, MoveFlag::None));
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
            let mut move_mask = get_queen_mask(source, position.ao) & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();
                let target_piece = get_target_piece_if_any(position, enemy_pieces, enemy_occupancies, target);
                move_list.add(BitMove::encode(source, target, queen, target_piece, MoveFlag::None));
            }
        }
    }

    move_list
}

#[inline(always)]
pub fn get_target_piece(position: &Position, enemy_piece_types: [PieceType; 6], target: Square) -> PieceType {
    for piece_type in enemy_piece_types {
        if position.bbs[piece_type].is_set_sq(target) {
            return piece_type;
        }
    }

    panic!("There seems to be something wrong with the occupancy bitboards!")
}

#[inline(always)]
pub fn get_target_piece_if_any(position: &Position, enemy_piece_types: [PieceType; 6], enemy_occupancies: Bitboard, target: Square) -> PieceType {
    if (enemy_occupancies & target.to_bb()).is_empty() {
        return PieceType::None;
    }
    
    get_target_piece(position, enemy_piece_types, target)
}
