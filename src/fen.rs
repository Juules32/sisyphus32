use crate::{board_state::BoardState, castling_rights::CastlingRights, color::Color, piece::PieceType, square::{Square, SquareParseError}};

pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
pub const KIWIPETE_POSITION: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
pub const ROOK_POSITION: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -";
pub const TRICKY_POSITION: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ -";
pub const TRICKY_POSITION_2: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

#[derive(Debug)]
pub struct FenParseError(&'static str);

fn set_pieces(board_state: &mut BoardState, pieces_str: &str) -> Result<(), FenParseError> {
    let mut sq_index = 0_u8;
    for pieces_char in pieces_str.chars() {
        match pieces_char {
            '1'..='8' => sq_index += pieces_char
                .to_digit(10)
                .ok_or(FenParseError("Could not convert char to digit!"))? as u8,
            '/' => (),
            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                let piece_type = PieceType::from(pieces_char);
                board_state.set_piece(piece_type, Square::from(sq_index));
            }
            _ => return Err(FenParseError("Invalid pieces!"))
        };
        if !pieces_char.is_digit(10) && pieces_char != '/' { sq_index += 1; }
    }
    board_state.populate_occupancies();
    Ok(())
}

fn set_side(board_state: &mut BoardState, side_str: &str) -> Result<(), FenParseError> {
    match side_str {
        "w" => board_state.side = Color::White,
        "b" => board_state.side = Color::Black,
        _ => return Err(FenParseError("Invalid side!")),
    }

    Ok(())
}

fn set_castling_rights(board_state: &mut BoardState, castling_rights_str: &str) -> Result<(), FenParseError> {
    for char in castling_rights_str.chars() {
        match char {
            'K' => board_state.castling_rights.0 |= CastlingRights::WK.0,
            'Q' => board_state.castling_rights.0 |= CastlingRights::WQ.0,
            'k' => board_state.castling_rights.0 |= CastlingRights::BK.0,
            'q' => board_state.castling_rights.0 |= CastlingRights::BQ.0,
            '-' => (),
            _ => return Err(FenParseError("Invalid castling rights!")),
        }
    }

    Ok(())
}

fn set_en_passant_sq(board_state: &mut BoardState, en_passant_sq_str: &str) -> Result<(), FenParseError> {
    match en_passant_sq_str {
        "-" => Ok(()),
        _ => {
            board_state.en_passant_sq = Square::try_from(en_passant_sq_str)
                .map_err(|SquareParseError(msg)| FenParseError(msg))?;
            Ok(())
        }
    }
}

pub fn parse(fen_string: &str) -> Result<BoardState, FenParseError> {
    let mut bs = BoardState::default();

    let mut fen_iter = fen_string.split_whitespace();
    let pieces_str = fen_iter.next().ok_or(FenParseError("No pieces found!"))?;
    let side_str = fen_iter.next().ok_or(FenParseError("No side found!"))?;
    let castling_rights_str = fen_iter.next().ok_or(FenParseError("No castling rights found!"))?;
    let en_passant_sq_str = fen_iter.next().ok_or(FenParseError("No en-passant found!"))?;

    set_pieces(&mut bs, pieces_str)?;
    set_side(&mut bs, side_str)?;
    set_castling_rights(&mut bs, castling_rights_str)?;
    set_en_passant_sq(&mut bs, en_passant_sq_str)?;

    Ok(bs)
}
