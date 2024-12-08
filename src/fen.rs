use crate::{board_state::BoardState, castling_rights::CastlingRights, color::Color, piece::PieceType, square::Square};

pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
pub const KIWIPETE_POSITION: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
pub const ROOK_POSITION: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -";
pub const TRICKY_POSITION: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ -";
pub const TRICKY_POSITION_2: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

fn set_pieces(board_state: &mut BoardState, pieces_str: &str) {
    let mut sq_index = 0_u8;
    for pieces_char in pieces_str.chars() {
        match pieces_char {
            '1'..='8' => sq_index += pieces_char.to_digit(10).expect("Could not convert pieces_char to digit!") as u8,
            '/' => (),
            _ => {
                let piece_type = PieceType::from(pieces_char);
                board_state.set_piece(piece_type, Square::from(sq_index));
            }
        };
        if !pieces_char.is_digit(10) && pieces_char != '/' { sq_index += 1; }
    }
    board_state.populate_occupancies();
}

fn set_side(board_state: &mut BoardState, side_str: &str) {
    if side_str == "w" {
        board_state.side = Color::White;
    }
    else if side_str == "b" {
        board_state.side = Color::Black;
    }
    else {
        panic!("Incorrect side_str!");
    }
}

fn set_castling_rights(board_state: &mut BoardState, castling_rights_str: &str) {
    for char in castling_rights_str.chars() {
        match char {
            'K' => board_state.castling_rights.0 |= CastlingRights::WK.0,
            'Q' => board_state.castling_rights.0 |= CastlingRights::WQ.0,
            'k' => board_state.castling_rights.0 |= CastlingRights::BK.0,
            'q' => board_state.castling_rights.0 |= CastlingRights::BQ.0,
            '-' => (),
            _ => panic!("Illegal char in castling_rights_str found!")
        }
    }
}

fn set_en_passant_sq(board_state: &mut BoardState, en_passant_sq_str: &str) {
    if en_passant_sq_str != "-" {
        board_state.en_passant_sq = Square::from(en_passant_sq_str);
    }
}

pub fn parse(fen_string: &str) -> BoardState {
    let mut bs = BoardState::default();

    let mut fen_iter = fen_string.split_whitespace();
    let pieces_str = fen_iter.next().expect("No pieces_str found!");
    let side_str = fen_iter.next().expect("No side_str found!");
    let castling_rights_str = fen_iter.next().expect("No castling_rights_str found!");
    let en_passant_sq_str = fen_iter.next().expect("No en_passant_sq_str found!");

    set_pieces(&mut bs, pieces_str);
    set_side(&mut bs, side_str);
    set_castling_rights(&mut bs, castling_rights_str);
    set_en_passant_sq(&mut bs, en_passant_sq_str);

    bs
}
