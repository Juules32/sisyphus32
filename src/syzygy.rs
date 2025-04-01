use shakmaty::{fen::Fen, CastlingMode, Chess};
use shakmaty_syzygy::Tablebase;

use crate::{bit_move::ScoringMove, fen::FenString, position::Position, score::Score, uci::Uci};

pub struct SyzygyTablebase {
    shakmaty_tablebase: Tablebase<Chess>,
    max_pieces: u8,
}

impl SyzygyTablebase {
    pub fn from_directory(path: &str) -> Result<SyzygyTablebase, Box<dyn std::error::Error>> {
        let mut shakmaty_tablebase = Tablebase::new();
        shakmaty_tablebase.add_directory(path)?;
        let max_pieces = shakmaty_tablebase.max_pieces() as u8;
        Ok(SyzygyTablebase { shakmaty_tablebase, max_pieces })
    }

    pub fn best_move(&self, position: &Position) -> Option<ScoringMove> {
        let shakmaty_position = FenString::from(position).to_string()
            .parse::<Fen>().ok()?
            .into_position::<Chess>(CastlingMode::Standard).ok()?;

        let best_move = self.shakmaty_tablebase.best_move(&shakmaty_position).ok()?;
        
        match best_move {
            Some((chess_move, maybe_rounded_dtz)) => {
                let move_string = chess_move.to_uci(CastlingMode::Standard).to_string();
                let score = if maybe_rounded_dtz.is_zero() { Score::DRAW_SCORE } else if maybe_rounded_dtz.is_positive() { Score::TABLEBASE_MOVE_SCORE } else { -Score::TABLEBASE_MOVE_SCORE };
                Some(ScoringMove::new(Uci::parse_move_string(position, &move_string).ok()?, score))
            },
            None => None,
        }
    }

    #[inline(always)]
    pub fn get_max_pieces(&self) -> u8 {
        self.max_pieces
    }
}
