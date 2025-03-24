use shakmaty::{fen::Fen, CastlingMode, Chess};
use shakmaty_syzygy::Tablebase;

use crate::{bit_move::BitMove, fen::FenString, position::Position};

pub struct SyzygyTablebase {
    shakmaty_tablebase: Tablebase<Chess>
}

impl SyzygyTablebase {
    pub fn from_directory(path: &str) -> Result<SyzygyTablebase, Box<dyn std::error::Error>> {
        let mut shakmaty_tablebase = Tablebase::new();
        shakmaty_tablebase.add_directory(path)?;
        Ok(SyzygyTablebase { shakmaty_tablebase })
    }

    pub fn best_move(&self, position: &Position) -> Option<BitMove> {
        let shakmaty_position = FenString::from(position).to_string()
            .parse::<Fen>().ok()?
            .into_position::<Chess>(CastlingMode::Standard).ok()?;

        let best_move = self.shakmaty_tablebase.best_move(&shakmaty_position).ok()?;
        
        match best_move {
            Some((chess_move, _)) => {
                let move_string = chess_move.to_uci(CastlingMode::Standard).to_string();
                Some(position.parse_move_string(&move_string).ok()?)
            },
            None => None,
        }
    }
}
