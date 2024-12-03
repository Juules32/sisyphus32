use crate::{bit_move::{BitMove, MoveFlag}, bitboard::Bitboard, board_state::BoardState, color::Color, move_init, move_list::MoveList, piece::PieceType, rank::Rank, square::Square};

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
pub fn get_queen_mask(square: Square, occupancy: Bitboard) -> Bitboard {
    get_bishop_mask(square, occupancy) | get_rook_mask(square, occupancy)
}

// based on state side, relevant pieces and occupancies can be selected
#[inline]
pub fn generate_moves(board_state: &BoardState) -> MoveList {
    let mut move_list = MoveList::default();
    
    let side = board_state.side;

    let [pawn, knight, bishop, rook, queen, king] = match side {
        Color::WHITE => PieceType::WHITE_PIECES,
        Color::BLACK => PieceType::BLACK_PIECES,
        _ => panic!("Illegal color found")
    };

    let enemy_pieces = match side {
        Color::WHITE => PieceType::BLACK_PIECES,
        Color::BLACK => PieceType::WHITE_PIECES,
        _ => panic!("Illegal color found")
    };

    let inv_own_occupancies = match side {
        Color::WHITE => !board_state.wo,
        Color::BLACK => !board_state.bo,
        _ => panic!("Illegal color found")
    };
    
    let enemy_occupancies = match side {
        Color::WHITE => board_state.bo,
        Color::BLACK => board_state.wo,
        _ => panic!("Illegal color found")
    };

    let pawn_promotion_rank = match side {
        Color::WHITE => Rank::R7,
        Color::BLACK => Rank::R2,
        _ => panic!("Illegal color found")
    };

    let pawn_starting_rank = match side {
        Color::WHITE => Rank::R2,
        Color::BLACK => Rank::R7,
        _ => panic!("Illegal color found")
    };

    {
        /*------------------------------*\ 
                   Knight moves
        \*------------------------------*/
        let mut knight_bb = board_state.bbs[knight];
        while knight_bb.is_not_empty() {
            let source = knight_bb.pop_lsb();
            
            let mut move_mask = unsafe { move_init::KNIGHT_MASKS[source] } & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();
                let target_piece = get_target_piece_if_any(board_state, enemy_pieces, enemy_occupancies, target);
                move_list.add(BitMove::encode(source, target, knight, target_piece, MoveFlag::Null));
            }
        }
    }

    {
        /*------------------------------*\ 
                    King moves
        \*------------------------------*/
        let mut king_bb = board_state.bbs[king];
        while king_bb.is_not_empty() {
            let source = king_bb.pop_lsb();
            let mut move_mask = unsafe { move_init::KING_MASKS[source] } & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();
                let target_piece = get_target_piece_if_any(board_state, enemy_pieces, enemy_occupancies, target);
                move_list.add(BitMove::encode(source, target, king, target_piece, MoveFlag::Null));
            }

            // Castling
        }
    }

    {
        /*------------------------------*\ 
                    Pawn moves
        \*------------------------------*/
        let mut pawn_bb = board_state.bbs[pawn];
        while pawn_bb.is_not_empty() {
            let source = pawn_bb.pop_lsb();
            let source_rank = source.rank();

            let mut move_mask = unsafe { move_init::PAWN_CAPTURE_MASKS[board_state.side][source] } & enemy_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();
                let target_piece = get_target_piece(board_state, enemy_pieces, target);

                if source_rank == pawn_promotion_rank {
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoN));
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoB));
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoR));
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::PromoQ));
                }
                else {
                    move_list.add(BitMove::encode(source, target, pawn, target_piece, MoveFlag::Null));
                }
            }

            // En-passant

            // Normal pushes (can't be captures) (remember promotion)
        }
    }

    move_list
}

#[inline(always)]
pub fn get_target_piece(board_state: &BoardState, enemy_piece_types: [PieceType; 6], target: Square) -> PieceType {
    for piece_type in enemy_piece_types {
        if board_state.bbs[piece_type].is_set_sq(target) {
            return piece_type;
        }
    }

    panic!("There seems to be something wrong with the occupancy bitboards!")
}


#[inline(always)]
pub fn get_target_piece_if_any(board_state: &BoardState, enemy_piece_types: [PieceType; 6], enemy_occupancies: Bitboard, target: Square) -> PieceType {
    if (enemy_occupancies & target.to_bb()).is_empty() {
        return PieceType::None;
    }
    
    get_target_piece(board_state, enemy_piece_types, target)
}
