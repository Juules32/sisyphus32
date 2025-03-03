use std::mem;

use ctor::ctor;

use crate::{bitboard::Bitboard, color::Color, file::File, piece::Piece, rank::Rank, square::Square};

const NUM_ROOK_MOVE_PERMUTATIONS: usize = 4096;
const NUM_BISHOP_MOVE_PERMUTATIONS: usize = 512;

pub static mut WHITE_PAWN_QUIET_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
pub static mut BLACK_PAWN_QUIET_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
pub static mut WHITE_PAWN_CAPTURE_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
pub static mut BLACK_PAWN_CAPTURE_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
pub static mut KNIGHT_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
pub static mut KING_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
pub static mut BISHOP_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
pub static mut ROOK_MASKS: [Bitboard; 64] = unsafe { mem::zeroed() };
pub static mut ROOK_MOVE_CONFIGURATIONS: [[Bitboard; NUM_ROOK_MOVE_PERMUTATIONS]; 64] = unsafe { mem::zeroed() };
pub static mut BISHOP_MOVE_CONFIGURATIONS: [[Bitboard; NUM_BISHOP_MOVE_PERMUTATIONS]; 64] = unsafe { mem::zeroed() };

pub static BISHOP_RELEVANT_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

pub static ROOK_RELEVANT_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12
];

pub static BISHOP_MAGIC_BITBOARDS: [Bitboard; 64] = [
    Bitboard(0x40040844404084),
    Bitboard(0x2004208a004208),
    Bitboard(0x10190041080202),
    Bitboard(0x108060845042010),
    Bitboard(0x581104180800210),
    Bitboard(0x2112080446200010),
    Bitboard(0x1080820820060210),
    Bitboard(0x3c0808410220200),
    Bitboard(0x4050404440404),
    Bitboard(0x21001420088),
    Bitboard(0x24d0080801082102),
    Bitboard(0x1020a0a020400),
    Bitboard(0x40308200402),
    Bitboard(0x4011002100800),
    Bitboard(0x401484104104005),
    Bitboard(0x801010402020200),
    Bitboard(0x400210c3880100),
    Bitboard(0x404022024108200),
    Bitboard(0x810018200204102),
    Bitboard(0x4002801a02003),
    Bitboard(0x85040820080400),
    Bitboard(0x810102c808880400),
    Bitboard(0xe900410884800),
    Bitboard(0x8002020480840102),
    Bitboard(0x220200865090201),
    Bitboard(0x2010100a02021202),
    Bitboard(0x152048408022401),
    Bitboard(0x20080002081110),
    Bitboard(0x4001001021004000),
    Bitboard(0x800040400a011002),
    Bitboard(0xe4004081011002),
    Bitboard(0x1c004001012080),
    Bitboard(0x8004200962a00220),
    Bitboard(0x8422100208500202),
    Bitboard(0x2000402200300c08),
    Bitboard(0x8646020080080080),
    Bitboard(0x80020a0200100808),
    Bitboard(0x2010004880111000),
    Bitboard(0x623000a080011400),
    Bitboard(0x42008c0340209202),
    Bitboard(0x209188240001000),
    Bitboard(0x400408a884001800),
    Bitboard(0x110400a6080400),
    Bitboard(0x1840060a44020800),
    Bitboard(0x90080104000041),
    Bitboard(0x201011000808101),
    Bitboard(0x1a2208080504f080),
    Bitboard(0x8012020600211212),
    Bitboard(0x500861011240000),
    Bitboard(0x180806108200800),
    Bitboard(0x4000020e01040044),
    Bitboard(0x300000261044000a),
    Bitboard(0x802241102020002),
    Bitboard(0x20906061210001),
    Bitboard(0x5a84841004010310),
    Bitboard(0x4010801011c04),
    Bitboard(0xa010109502200),
    Bitboard(0x4a02012000),
    Bitboard(0x500201010098b028),
    Bitboard(0x8040002811040900),
    Bitboard(0x28000010020204),
    Bitboard(0x6000020202d0240),
    Bitboard(0x8918844842082200),
    Bitboard(0x4010011029020020),
];

pub static ROOK_MAGIC_BITBOARDS: [Bitboard; 64] = [
    Bitboard(0x8a80104000800020),
    Bitboard(0x140002000100040),
    Bitboard(0x2801880a0017001),
    Bitboard(0x100081001000420),
    Bitboard(0x200020010080420),
    Bitboard(0x3001c0002010008),
    Bitboard(0x8480008002000100),
    Bitboard(0x2080088004402900),
    Bitboard(0x800098204000),
    Bitboard(0x2024401000200040),
    Bitboard(0x100802000801000),
    Bitboard(0x120800800801000),
    Bitboard(0x208808088000400),
    Bitboard(0x2802200800400),
    Bitboard(0x2200800100020080),
    Bitboard(0x801000060821100),
    Bitboard(0x80044006422000),
    Bitboard(0x100808020004000),
    Bitboard(0x12108a0010204200),
    Bitboard(0x140848010000802),
    Bitboard(0x481828014002800),
    Bitboard(0x8094004002004100),
    Bitboard(0x4010040010010802),
    Bitboard(0x20008806104),
    Bitboard(0x100400080208000),
    Bitboard(0x2040002120081000),
    Bitboard(0x21200680100081),
    Bitboard(0x20100080080080),
    Bitboard(0x2000a00200410),
    Bitboard(0x20080800400),
    Bitboard(0x80088400100102),
    Bitboard(0x80004600042881),
    Bitboard(0x4040008040800020),
    Bitboard(0x440003000200801),
    Bitboard(0x4200011004500),
    Bitboard(0x188020010100100),
    Bitboard(0x14800401802800),
    Bitboard(0x2080040080800200),
    Bitboard(0x124080204001001),
    Bitboard(0x200046502000484),
    Bitboard(0x480400080088020),
    Bitboard(0x1000422010034000),
    Bitboard(0x30200100110040),
    Bitboard(0x100021010009),
    Bitboard(0x2002080100110004),
    Bitboard(0x202008004008002),
    Bitboard(0x20020004010100),
    Bitboard(0x2048440040820001),
    Bitboard(0x101002200408200),
    Bitboard(0x40802000401080),
    Bitboard(0x4008142004410100),
    Bitboard(0x2060820c0120200),
    Bitboard(0x1001004080100),
    Bitboard(0x20c020080040080),
    Bitboard(0x2935610830022400),
    Bitboard(0x44440041009200),
    Bitboard(0x280001040802101),
    Bitboard(0x2100190040002085),
    Bitboard(0x80c0084100102001),
    Bitboard(0x4024081001000421),
    Bitboard(0x20030a0244872),
    Bitboard(0x12001008414402),
    Bitboard(0x2006104900a0804),
    Bitboard(0x1004081002402),
];

pub struct MoveMasks { }

impl MoveMasks {
    unsafe fn init_masks() {
        for square in Square::ALL_SQUARES {
            WHITE_PAWN_QUIET_MASKS[square] = Self::generate_pawn_quiet_mask(Color::White, square);
            BLACK_PAWN_QUIET_MASKS[square] = Self::generate_pawn_quiet_mask(Color::Black, square);
            WHITE_PAWN_CAPTURE_MASKS[square] = Self::generate_pawn_capture_mask(Color::White, square);
            BLACK_PAWN_CAPTURE_MASKS[square] = Self::generate_pawn_capture_mask(Color::Black, square);
            KNIGHT_MASKS[square] = Self::generate_knight_mask(square);
            KING_MASKS[square] = Self::generate_king_mask(square);
            BISHOP_MASKS[square] = Self::generate_bishop_mask(square);
            ROOK_MASKS[square] = Self::generate_rook_mask(square);

            debug_assert_eq!(BISHOP_MASKS[square].count_bits(), BISHOP_RELEVANT_BITS[square]);
            debug_assert_eq!(ROOK_MASKS[square].count_bits(), ROOK_RELEVANT_BITS[square]);
        }
    }

    unsafe fn init_slider_configurations() {
        for square in Square::ALL_SQUARES {
            let bishop_mask = BISHOP_MASKS[square];
            let rook_mask = ROOK_MASKS[square];

            let num_bishop_relevant_bits = BISHOP_RELEVANT_BITS[square];
            let num_rook_relevant_bits = ROOK_RELEVANT_BITS[square];

            let max_bishop_occupancy_index = 1 << num_bishop_relevant_bits;
            let max_rook_occupancy_index = 1 << num_rook_relevant_bits;

            for occupancy_index in 0..max_bishop_occupancy_index {
                let occupancy = Self::generate_occupancy_permutation(occupancy_index, num_bishop_relevant_bits, bishop_mask);
                let magic_index = (occupancy.0.wrapping_mul(BISHOP_MAGIC_BITBOARDS[square].0) >> (64 - num_bishop_relevant_bits)) as usize;
                BISHOP_MOVE_CONFIGURATIONS[square][magic_index] = Self::generate_bishop_moves_on_the_fly(square, occupancy);
            }

            for occupancy_index in 0..max_rook_occupancy_index {
                let occupancy = Self::generate_occupancy_permutation(occupancy_index, num_rook_relevant_bits, rook_mask);
                let magic_index = (occupancy.0.wrapping_mul(ROOK_MAGIC_BITBOARDS[square].0) >> (64 - num_rook_relevant_bits)) as usize;
                ROOK_MOVE_CONFIGURATIONS[square][magic_index] = Self::generate_rook_moves_on_the_fly(square, occupancy);
            }
        }
    }

    fn generate_pawn_quiet_mask(color: Color, square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let square_rank = square.rank();

        match color {
            Color::White => {
                bb_mask |= square_bb.shift_upwards(8);

                if square_rank == Rank::R2 {
                    bb_mask |= square_bb.shift_upwards(16);
                }
            }
            Color::Black => {
                bb_mask |= square_bb.shift_downwards(8);

                if square_rank == Rank::R7 {
                    bb_mask |= square_bb.shift_downwards(16);
                }
            }
        };

        bb_mask
    }

    fn generate_pawn_capture_mask(color: Color, square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let square_file = square.file();

        match color {
            Color::White => {
                if square_file != File::FA {
                    bb_mask |= square_bb.shift_upwards(9);
                }

                if square_file != File::FH {
                    bb_mask |= square_bb.shift_upwards(7);
                }
            }
            Color::Black => {
                if square_file != File::FA {
                    bb_mask |= square_bb.shift_downwards(7);
                }

                if square_file != File::FH {
                    bb_mask |= square_bb.shift_downwards(9);
                }
            }
        };

        bb_mask
    }

    fn generate_knight_mask(square: Square) -> Bitboard {
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

    fn generate_king_mask(square: Square) -> Bitboard {
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

    fn generate_bishop_mask(square: Square) -> Bitboard {
        use std::cmp::min;

        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let rank_u8 = square.rank_as_u8();
        let file_u8 = square.file_as_u8();

        // Bottom right
        for i in 1..=min(6_u8.saturating_sub(rank_u8), 6_u8.saturating_sub(file_u8)) {
            let ray = square_bb.shift_downwards(i * 9);
            bb_mask |= ray;
        }
        
        // Top right
        for i in 1..=min(rank_u8.saturating_sub(1), 6_u8.saturating_sub(file_u8)) {
            let ray = square_bb.shift_upwards(i * 7);
            bb_mask |= ray;
        }

        // Bottom left
        for i in 1..=min(6_u8.saturating_sub(rank_u8), file_u8.saturating_sub(1)) {
            let ray = square_bb.shift_downwards(i * 7);
            bb_mask |= ray;
        }

        // Top left
        for i in 1..=min(rank_u8.saturating_sub(1), file_u8.saturating_sub(1)) {
            let ray = square_bb.shift_upwards(i * 9);
            bb_mask |= ray;
        }

        bb_mask
    }


    fn generate_rook_mask(square: Square) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let rank_u8 = square.rank_as_u8();
        let file_u8 = square.file_as_u8();

        // Down
        for i in 1..=(6_u8.saturating_sub(rank_u8)) {
            let ray = square_bb.shift_downwards(i * 8);
            bb_mask |= ray;
        }
        
        // Up
        for i in 1..=(rank_u8.saturating_sub(1)) {
            let ray = square_bb.shift_upwards(i * 8);
            bb_mask |= ray;
        }

        // Right
        for i in 1..=(6_u8.saturating_sub(file_u8)) {
            let ray = square_bb.shift_downwards(i);
            bb_mask |= ray;
        }

        // Left
        for i in 1..=(file_u8.saturating_sub(1)) {
            let ray = square_bb.shift_upwards(i);
            bb_mask |= ray;
        }

        bb_mask
    }


    pub fn generate_bishop_moves_on_the_fly(square: Square, occupancy: Bitboard) -> Bitboard {
        use std::cmp::min;

        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let rank_u8 = square.rank_as_u8();
        let file_u8 = square.file_as_u8();

        // Bottom right
        for i in 1..=min(7_u8.saturating_sub(rank_u8), 7_u8.saturating_sub(file_u8)) {
            let ray = square_bb.shift_downwards(i * 9);
            bb_mask |= ray;
            if (ray & occupancy).is_not_empty() { break; }
        }
        
        // Top right
        for i in 1..=min(rank_u8, 7_u8.saturating_sub(file_u8)) {
            let ray = square_bb.shift_upwards(i * 7);
            bb_mask |= ray;
            if (ray & occupancy).is_not_empty() { break; }
        }

        // Bottom left
        for i in 1..=min(7_u8.saturating_sub(rank_u8), file_u8) {
            let ray = square_bb.shift_downwards(i * 7);
            bb_mask |= ray;
            if (ray & occupancy).is_not_empty() { break; }
        }

        // Top left
        for i in 1..=min(rank_u8, file_u8) {
            let ray = square_bb.shift_upwards(i * 9);
            bb_mask |= ray;
            if (ray & occupancy).is_not_empty() { break; }
        }

        bb_mask
    }

    pub fn generate_rook_moves_on_the_fly(square: Square, occupancy: Bitboard) -> Bitboard {
        let mut bb_mask = Bitboard::EMPTY;
        let square_bb = square.to_bb();
        let rank_u8 = square.rank_as_u8();
        let file_u8 = square.file_as_u8();

        // Down
        for i in 1..=(7_u8.saturating_sub(rank_u8)) {
            let ray = square_bb.shift_downwards(i * 8);
            bb_mask |= ray;
            if (ray & occupancy).is_not_empty() { break; }
        }
        
        // Up
        for i in 1..=rank_u8 {
            let ray = square_bb.shift_upwards(i * 8);
            bb_mask |= ray;
            if (ray & occupancy).is_not_empty() { break; }
        }

        // Right
        for i in 1..=(7_u8.saturating_sub(file_u8)) {
            let ray = square_bb.shift_downwards(i);
            bb_mask |= ray;
            if (ray & occupancy).is_not_empty() { break; }
        }

        // Left
        for i in 1..=file_u8 {
            let ray = square_bb.shift_upwards(i);
            bb_mask |= ray;
            if (ray & occupancy).is_not_empty() { break; }
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

    #[inline(always)]
    pub fn get_pawn_quiet_mask(color: Color, square: Square) -> Bitboard {
        unsafe {
            match color {
                Color::White => WHITE_PAWN_QUIET_MASKS[square],
                Color::Black => BLACK_PAWN_QUIET_MASKS[square],
            }
        }
    }

    #[inline(always)]
    pub fn get_pawn_capture_mask(color: Color, square: Square) -> Bitboard {
        unsafe {
            match color {
                Color::White => WHITE_PAWN_CAPTURE_MASKS[square],
                Color::Black => BLACK_PAWN_CAPTURE_MASKS[square],
            }
        }
    }

    #[inline(always)]
    pub fn get_knight_mask(square: Square) -> Bitboard {
        unsafe { KNIGHT_MASKS[square] }
    }

    #[inline(always)]
    pub fn get_king_mask(square: Square) -> Bitboard {
        unsafe { KING_MASKS[square] }
    }

    #[inline(always)]
    #[cfg(not(feature = "unit_magic_bbs"))]
    pub fn get_bishop_mask(square: Square, occupancy: Bitboard) -> Bitboard {
        MoveMasks::generate_bishop_moves_on_the_fly(square, occupancy)
    }

    #[inline(always)]
    #[cfg(feature = "unit_magic_bbs")]
    pub fn get_bishop_mask(square: Square, occupancy: Bitboard) -> Bitboard {
        unsafe {
            let index = (
                (occupancy.0 & BISHOP_MASKS[square].0).wrapping_mul(BISHOP_MAGIC_BITBOARDS[square].0) >> 
                (64 - BISHOP_RELEVANT_BITS[square])
            ) as usize;
            BISHOP_MOVE_CONFIGURATIONS[square][index]
        }
    }

    #[inline(always)]
    #[cfg(not(feature = "unit_magic_bbs"))]
    pub fn get_rook_mask(square: Square, occupancy: Bitboard) -> Bitboard {
        MoveMasks::generate_rook_moves_on_the_fly(square, occupancy)
    }

    #[inline(always)]
    #[cfg(feature = "unit_magic_bbs")]
    pub fn get_rook_mask(square: Square, occupancy: Bitboard) -> Bitboard {
        unsafe {
            let index = ( 
                (occupancy.0 & ROOK_MASKS[square].0).wrapping_mul(ROOK_MAGIC_BITBOARDS[square].0) >> 
                (64 - ROOK_RELEVANT_BITS[square])
            ) as usize;
            ROOK_MOVE_CONFIGURATIONS[square][index]
        }
    }

    #[inline(always)]
    #[cfg(not(feature = "unit_magic_bbs"))]
    pub fn get_queen_mask(square: Square, occupancy: Bitboard) -> Bitboard {
        MoveMasks::get_bishop_mask(square, occupancy) | MoveMasks::get_rook_mask(square, occupancy)
    }

    #[inline(always)]
    #[cfg(feature = "unit_magic_bbs")]
    pub fn get_queen_mask(square: Square, occupancy: Bitboard) -> Bitboard {
        Self::get_bishop_mask(square, occupancy) | Self::get_rook_mask(square, occupancy)
    }

    #[inline(always)]
    pub fn get_piece_mask(piece: Piece, square: Square, occupancy: Bitboard) -> Bitboard {
        match piece {
            Piece::WP => Self::get_pawn_capture_mask(Color::White, square),
            Piece::BP => Self::get_pawn_capture_mask(Color::Black, square),
            Piece::WN | Piece::BN => Self::get_knight_mask(square),
            Piece::WB | Piece::BB => Self::get_bishop_mask(square, occupancy),
            Piece::WR | Piece::BR => Self::get_rook_mask(square, occupancy),
            Piece::WQ | Piece::BQ => Self::get_queen_mask(square, occupancy),
            Piece::WK | Piece::BK => Self::get_king_mask(square),
            Piece::None => panic!("Can't get piece mask for none piece!"),
        }
    }
}

#[ctor]
unsafe fn init_masks_and_slider_configurations() {
    MoveMasks::init_masks();
    MoveMasks::init_slider_configurations();
}
