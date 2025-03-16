use core::fmt;

use crate::{castling_rights::CastlingRights, color::Color, eval_position::EvalPosition, piece::Piece, position::Position, square::{Square, SquareParseError}, zobrist::ZobristKey};

#[derive(Debug)]
pub struct FenParseError(pub &'static str);

pub struct FenString { string: String }

impl FenString {
    pub fn startpos() -> FenString {
        FenString::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -")
    }

    pub fn kiwipete() -> FenString {
        FenString::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
    }

    pub fn rook() -> FenString {
        FenString::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -")
    }

    pub fn tricky() -> FenString {
        FenString::from("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ -")
    }

    pub fn tricky2() -> FenString {
        FenString::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
    }

    pub fn parse(&self) -> Result<Position, FenParseError> {
        let mut position = Position::default();
        
        let mut fen_iter = self.string.split_whitespace();
        let pieces_str = fen_iter.next().ok_or(FenParseError("No pieces found!"))?;
        let side_str = fen_iter.next().ok_or(FenParseError("No side found!"))?;
        let castling_rights_str = fen_iter.next().ok_or(FenParseError("No castling rights found!"))?;
        let en_passant_sq_str = fen_iter.next().ok_or(FenParseError("No en-passant found!"))?;
        
        Self::set_pieces(&mut position, pieces_str)?;
        Self::set_side(&mut position, side_str)?;
        Self::set_castling_rights(&mut position, castling_rights_str)?;
        Self::set_en_passant_sq(&mut position, en_passant_sq_str)?;
        
        position.zobrist_key = ZobristKey::generate(&position);

        #[cfg(feature = "unit_interpolated_eval")]
        { position.game_phase_score = EvalPosition::get_game_phase_score(&position); }
        
        Ok(position)
    }
    
    fn set_pieces(position: &mut Position, pieces_str: &str) -> Result<(), FenParseError> {
        let mut sq_index = 0_u8;
        for pieces_char in pieces_str.chars() {
            match pieces_char {
                '1'..='8' => sq_index += pieces_char
                .to_digit(10)
                .ok_or(FenParseError("Could not convert char to digit!"))? as u8,
                '/' => (),
                'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                    let piece = Piece::from(pieces_char);
                    position.set_piece(piece, Square::from(sq_index));
                }
                _ => return Err(FenParseError("Invalid pieces!")),
            };
            if !pieces_char.is_ascii_digit() && pieces_char != '/' { sq_index += 1; }
        }
        position.populate_occupancies();
        Ok(())
    }
    
    fn set_side(position: &mut Position, side_str: &str) -> Result<(), FenParseError> {
        match side_str {
            "w" => position.side = Color::White,
            "b" => position.side = Color::Black,
            _ => return Err(FenParseError("Invalid side!")),
        }
        
        Ok(())
    }
    
    fn set_castling_rights(position: &mut Position, castling_rights_str: &str) -> Result<(), FenParseError> {
        for char in castling_rights_str.chars() {
            match char {
                'K' => position.castling_rights.0 |= CastlingRights::WK.0,
                'Q' => position.castling_rights.0 |= CastlingRights::WQ.0,
                'k' => position.castling_rights.0 |= CastlingRights::BK.0,
                'q' => position.castling_rights.0 |= CastlingRights::BQ.0,
                '-' => (),
                _ => return Err(FenParseError("Invalid castling rights!")),
            }
        }
        
        Ok(())
    }
    
    fn set_en_passant_sq(position: &mut Position, en_passant_sq_str: &str) -> Result<(), FenParseError> {
        match en_passant_sq_str {
            "-" => Ok(()),
            _ => {
                position.en_passant_option = Some(Square::try_from(en_passant_sq_str)
                    .map_err(|SquareParseError(msg)| FenParseError(msg))?);
                Ok(())
            }
        }
    }
}

impl TryInto<Position> for FenString {
    type Error = FenParseError;

    fn try_into(self) -> Result<Position, Self::Error> {
        self.parse()
    }
}

impl From<&str> for FenString {
    fn from(string: &str) -> Self {
        FenString { string: string.to_string() }
    }
}

impl From<String> for FenString {
    fn from(string: String) -> Self {
        FenString { string }
    }
}

impl From<&Position> for FenString {
    fn from(position: &Position) -> Self {
        let mut fen_str = String::new();
        let mut curr_width = 0;
        let mut curr_empty = 0;
        for square in Square::ALL_SQUARES {
            curr_width += 1;

            let piece_option = position.get_piece_option(square);
            match piece_option {
                None => curr_empty += 1,
                Some(piece) => {
                    if curr_empty != 0 {
                        fen_str.push_str(&curr_empty.to_string());
                        curr_empty = 0;
                    }
                    fen_str.push(piece.into())
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
            match position.side {
                Color::White => 'w',
                Color::Black => 'b',
            }
        );

        fen_str.push(' ');

        fen_str.push_str(&position.castling_rights.to_string());

        fen_str.push(' ');

        fen_str.push_str(
            &match position.en_passant_option {
                None => "-".to_owned(),
                Some(en_passant_sq) => en_passant_sq.to_string(),
            }
        );
        
        fen_str.into()
    }
}

impl fmt::Display for FenString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.string)
    }
}
