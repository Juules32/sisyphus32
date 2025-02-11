use crate::{bit_move::ScoringMove, color::Color, position::Position, square::Square};

static PIECE_SCORES: [i16; 13] = [100, 300, 301, 500, 900, 10000, -100, -300, -301, -500, -900, -10000, 0];

pub fn basic(position: &Position) -> ScoringMove {
    let side_modifier = match position.side {
        Color::White => 1,
        Color::Black => -1
    };
    ScoringMove::blank(Square::ALL_SQUARES.iter().fold(0, |acc, &sq| acc + PIECE_SCORES[position.get_piece(sq) as usize] * side_modifier))
}
