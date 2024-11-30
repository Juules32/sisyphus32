



pub mod move_init {
    use crate::{bitboard::Bitboard, color::Color, rank::Rank, square::Square};

    pub static mut PAWN_QUIET_MASKS: [[Bitboard; 64]; 2] = [[Bitboard::NULL; 64]; 2];
    pub static mut PAWN_CAPTURE_MASKS: [[Bitboard; 64]; 2] = [[Bitboard::NULL; 64]; 2];
    pub static mut KNIGHT_MASKS: [Bitboard; 64] = [Bitboard::NULL; 64];
    pub static mut KING_MASKS: [Bitboard; 64] = [Bitboard::NULL; 64];
    pub static mut ROOK_MASKS: [Bitboard; 64] = [Bitboard::NULL; 64];
    pub static mut BISHOP_MASKS: [Bitboard; 64] = [Bitboard::NULL; 64];
    pub static mut BISHOP_MOVE_CONFIGURATIONS: [[Bitboard; 64]; 512] = [[Bitboard::NULL; 64]; 512];
    pub static mut ROOK_MOVE_CONFIGURATIONS: [[Bitboard; 64]; 4096] = [[Bitboard::NULL; 64]; 4096];

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

    pub fn init() {
        init_masks();
        init_slider_configurations();
    }

    pub fn init_masks() {
        unsafe {
            for square in Square::ALL_SQUARES {
                PAWN_QUIET_MASKS[Color::WHITE][square] = get_pawn_quiet_mask(Color::WHITE, square);
            }
        }
    }

    pub fn init_slider_configurations() {
        for square in Square::ALL_SQUARES {
            
        }
    }

    pub fn get_pawn_quiet_mask(color: Color, square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::NULL;
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
}
