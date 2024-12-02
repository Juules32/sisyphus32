



pub mod move_init {
    use crate::{bitboard::Bitboard, color::Color, rank::Rank, file::File, square::Square};

    pub static mut WHITE_PAWN_QUIET_MASKS: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    pub static mut BLACK_PAWN_QUIET_MASKS: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    pub static mut WHITE_PAWN_CAPTURE_MASKS: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    pub static mut BLACK_PAWN_CAPTURE_MASKS: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    pub static mut KNIGHT_MASKS: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    pub static mut KING_MASKS: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    pub static mut BISHOP_MASKS: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    pub static mut ROOK_MASKS: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    pub static mut ROOK_MOVE_CONFIGURATIONS: [[Bitboard; 64]; 4096] = [[Bitboard::EMPTY; 64]; 4096];
    pub static mut BISHOP_MOVE_CONFIGURATIONS: [[Bitboard; 64]; 512] = [[Bitboard::EMPTY; 64]; 512];

    pub const BISHOP_RELEVANT_BITS: [u8; 64] = [
        6, 5, 5, 5, 5, 5, 5, 6,
        5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5,
        6, 5, 5, 5, 5, 5, 5, 6
    ];

    pub const ROOK_RELEVANT_BITS: [u8; 64] = [
        12, 11, 11, 11, 11, 11, 11, 12,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        12, 11, 11, 11, 11, 11, 11, 12
    ];

    pub unsafe fn init() {
        init_masks();
        init_slider_configurations();
    }

    unsafe fn init_masks() {
        for square in Square::ALL_SQUARES {
            WHITE_PAWN_QUIET_MASKS[square] = get_pawn_quiet_mask(Color::WHITE, square);
            WHITE_PAWN_CAPTURE_MASKS[square] = get_pawn_capture_mask(Color::WHITE, square);
            BLACK_PAWN_QUIET_MASKS[square] = get_pawn_quiet_mask(Color::BLACK, square);
            BLACK_PAWN_CAPTURE_MASKS[square] = get_pawn_capture_mask(Color::BLACK, square);
            KNIGHT_MASKS[square] = get_knight_mask(square);
            KING_MASKS[square] = get_king_mask(square);
            BISHOP_MASKS[square] = get_bishop_mask(square);
            ROOK_MASKS[square] = get_rook_mask(square);

            debug_assert_eq!(BISHOP_MASKS[square].count_bits(), BISHOP_RELEVANT_BITS[square]);
            debug_assert_eq!(ROOK_MASKS[square].count_bits(), ROOK_RELEVANT_BITS[square]);
        }
    }

    unsafe fn init_slider_configurations() {
        for square in Square::ALL_SQUARES {
            
        }
    }

    fn get_pawn_quiet_mask(color: Color, square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let square_rank = square.rank();
        
        match color {
            Color::WHITE => {
                bb_mask |= square_bb.shift_upwards(8);

                if square_rank == Rank::R2 {
                    bb_mask |= square_bb.shift_upwards(16);
                }
            },
            Color::BLACK => {
                bb_mask |= square_bb.shift_downwards(8);

                if square_rank == Rank::R7 {
                    bb_mask |= square_bb.shift_downwards(16);
                }
            },
            _ => panic!("Illegal color used!")
        };

        bb_mask
    }

    fn get_pawn_capture_mask(color: Color, square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let square_file = square.file();

        match color {
            Color::WHITE => {
                if square_file != File::FA {
                    bb_mask |= square_bb.shift_upwards(9);
                }

                if square_file != File::FH {
                    bb_mask |= square_bb.shift_upwards(7);
                }
            },
            Color::BLACK => {
                if square_file != File::FA {
                    bb_mask |= square_bb.shift_downwards(7);
                }

                if square_file != File::FH {
                    bb_mask |= square_bb.shift_downwards(9);
                }
            },
            _ => panic!("Illegal color used!")
        };

        bb_mask
    }

    fn get_knight_mask(square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let square_file = square.file();

        if square_file != File::FA {
            bb_mask |= square_bb.shift_upwards(17);
            bb_mask |= square_bb.shift_downwards(15);

            if square_file != File::FB {
                bb_mask |= square_bb.shift_upwards(10);
                bb_mask |= square_bb.shift_downwards(6);
            }
        }

        if square_file != File::FH {
            bb_mask |= square_bb.shift_upwards(15);
            bb_mask |= square_bb.shift_downwards(17);

            if square_file != File::FG {
                bb_mask |= square_bb.shift_upwards(6);
                bb_mask |= square_bb.shift_downwards(10);
            }
        }

        bb_mask
    }

    fn get_king_mask(square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let square_file = square.file();

        bb_mask |= square_bb.shift_upwards(8);
        bb_mask |= square_bb.shift_downwards(8);

        if square_file != File::FA {
            bb_mask |= square_bb.shift_upwards(1);
            bb_mask |= square_bb.shift_upwards(9);
            bb_mask |= square_bb.shift_downwards(7);
        }

        if square_file != File::FH {
            bb_mask |= square_bb.shift_upwards(7);
            bb_mask |= square_bb.shift_downwards(1);
            bb_mask |= square_bb.shift_downwards(9);
        }

        bb_mask
    }

    fn get_bishop_mask(square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();

        let mut seeker = square_bb;
        while 
            (seeker & Bitboard::RANK_8).is_empty() &&
            (seeker & Bitboard::FILE_A).is_empty()
        {
            bb_mask |= seeker;
            seeker = seeker.shift_upwards(9);
        }

        seeker = square_bb;
        while
            (seeker & Bitboard::RANK_8).is_empty() &&
            (seeker & Bitboard::FILE_H).is_empty()
        {
            bb_mask |= seeker;
            seeker = seeker.shift_upwards(7);
        }

        seeker = square_bb;
        while
            (seeker & Bitboard::RANK_1).is_empty() &&
            (seeker & Bitboard::FILE_A).is_empty()
        {
            bb_mask |= seeker;
            seeker = seeker.shift_downwards(7);
        }

        seeker = square_bb;
        while
            (seeker & Bitboard::RANK_1).is_empty() &&
            (seeker & Bitboard::FILE_H).is_empty()
        {
            bb_mask |= seeker;
            seeker = seeker.shift_downwards(9);
        }

        bb_mask.pop_sq(square);

        bb_mask
    }

    fn get_rook_mask(square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();

        let mut seeker = square_bb;
        while (seeker & Bitboard::RANK_8).is_empty() {
            bb_mask |= seeker;
            seeker = seeker.shift_upwards(8);
        }

        seeker = square_bb;
        while (seeker & Bitboard::RANK_1).is_empty() {
            bb_mask |= seeker;
            seeker = seeker.shift_downwards(8);
        }

        seeker = square_bb;
        while (seeker & Bitboard::FILE_A).is_empty() {
            bb_mask |= seeker;
            seeker = seeker.shift_upwards(1);
        }

        seeker = square_bb;
        while (seeker & Bitboard::FILE_H).is_empty() {
            bb_mask |= seeker;
            seeker = seeker.shift_downwards(1);
        }

        bb_mask.pop_sq(square);

        bb_mask
    }

    pub fn get_bishop_moves_on_the_fly(square: Square, blockers: Bitboard) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();

        let mut seeker = square_bb;
        while 
            (seeker & Bitboard::RANK_8).is_empty() &&
            (seeker & Bitboard::FILE_A).is_empty() &&
            (seeker & blockers).is_empty()
        {
            seeker = seeker.shift_upwards(9);
            bb_mask |= seeker;
        }

        seeker = square_bb;
        while
            (seeker & Bitboard::RANK_8).is_empty() &&
            (seeker & Bitboard::FILE_H).is_empty() &&
            (seeker & blockers).is_empty()
        {
            seeker = seeker.shift_upwards(7);
            bb_mask |= seeker;
        }

        seeker = square_bb;
        while
            (seeker & Bitboard::RANK_1).is_empty() &&
            (seeker & Bitboard::FILE_A).is_empty() &&
            (seeker & blockers).is_empty()
        {
            seeker = seeker.shift_downwards(7);
            bb_mask |= seeker;
        }

        seeker = square_bb;
        while
            (seeker & Bitboard::RANK_1).is_empty() &&
            (seeker & Bitboard::FILE_H).is_empty() &&
            (seeker & blockers).is_empty()
        {
            seeker = seeker.shift_downwards(9);
            bb_mask |= seeker;
        }

        bb_mask
    }

    pub fn get_rook_moves_on_the_fly(square: Square, blockers: Bitboard) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();

        let mut seeker = square_bb;
        while (seeker & Bitboard::RANK_8).is_empty() && (seeker & blockers).is_empty() {
            seeker = seeker.shift_upwards(8);
            bb_mask |= seeker;
        }

        seeker = square_bb;
        while (seeker & Bitboard::RANK_1).is_empty() && (seeker & blockers).is_empty() {
            seeker = seeker.shift_downwards(8);
            bb_mask |= seeker;
        }

        seeker = square_bb;
        while (seeker & Bitboard::FILE_A).is_empty() && (seeker & blockers).is_empty() {
            seeker = seeker.shift_upwards(1);
            bb_mask |= seeker;
        }

        seeker = square_bb;
        while (seeker & Bitboard::FILE_H).is_empty() && (seeker & blockers).is_empty() {
            seeker = seeker.shift_downwards(1);
            bb_mask |= seeker;
        }

        bb_mask
    }

    // Generates the relevant occupancy bitboard for a slider piece from an index,
    // the number of relevant bits, and the relevant mask.
    pub fn generate_occupancy_permutation(occupancy_index: u32, num_bits: u8, mut mask: Bitboard) -> Bitboard {
        let mut occupancy = Bitboard::EMPTY;
        for i in 0..num_bits {
            let square = mask.pop_lsb();
            if occupancy_index & (1 << i) != 0 {
                occupancy.set_sq(square);
            }
        }

        occupancy
    }
}
