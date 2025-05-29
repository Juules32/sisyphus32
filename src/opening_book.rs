use std::time::Duration;
use serde::Deserialize;

use crate::{BitMove, Color, FenString, Legal, MoveGeneration, Position, Uci};

const NUM_GAMES_THRESHOLD: u32 = 1_000;
const WINRATE_THRESHOLD: f32 = 0.35;
const OPENING_BOOK_TIMEOUT_MS: u64 = 500;

#[derive(Deserialize)]
struct LichessOpeningStats {
    moves: Vec<LichessMoveStats>,
}

#[derive(Deserialize)]
struct LichessMoveStats {
    uci: String,
    white: u32,
    draws: u32,
    black: u32,
}

impl LichessMoveStats {
    #[inline(always)]
    fn has_enough_games(&self) -> bool {
        self.white + self.draws + self.black >= NUM_GAMES_THRESHOLD
    }
    
    #[inline(always)]
    fn has_acceptable_winrate(&self, side: Color) -> bool {
        let (playing_side, opposing_side) = match side {
            Color::White => (self.white, self.black),
            Color::Black => (self.black, self.white),
        };

        // NOTE: To prevent dividing by zero
        if opposing_side == 0 {
            return playing_side > 0;
        }

        playing_side as f32 / (playing_side + opposing_side) as f32 > WINRATE_THRESHOLD
    }

    #[inline(always)]
    fn is_candidate(&self, side: Color) -> bool {
        self.has_enough_games() && self.has_acceptable_winrate(side)
    }
}

impl LichessOpeningStats {
    #[inline(always)]
    fn get_opening_move_contenders(&self, position: &Position) -> Vec<BitMove> {
        let legal_moves = MoveGeneration::generate_moves::<BitMove, Legal>(position);
        self.moves
            .iter()
            .filter_map(|opening_move| {
                if opening_move.is_candidate(position.side) {
                    Uci::parse_move_string(&legal_moves, &opening_move.uci).ok()
                } else {
                    None
                }
            })
            .collect()
    }
}

pub(crate) struct OpeningBook {
    agent: ureq::Agent
}

impl Default for OpeningBook {
    fn default() -> Self {
        Self {
            agent: ureq::Agent::config_builder()
                .timeout_global(Some(Duration::from_millis(OPENING_BOOK_TIMEOUT_MS)))
                .build()
                .into()
        }
    }
}

impl OpeningBook {
    fn get_lichess_opening_stats(&self, position: &Position) -> Result<LichessOpeningStats, ureq::Error> {
        let fen_string = FenString::from(position);
        let fen_with_replaced_spaces = fen_string.to_string().replace(" ", "_");
        let uri = &format!("https://explorer.lichess.ovh/masters?fen={fen_with_replaced_spaces}");
        let resp = self.agent.get(uri).call();
        let body = resp?.body_mut().read_to_string()?;
        let lichess_opening_stats: LichessOpeningStats = serde_json::from_str(&body)
            .map_err(ureq::Error::Json)?;
        Ok(lichess_opening_stats)
    }

    pub(crate) fn get_move(&self, position: &Position) -> Option<BitMove> {
        let lichess_opening_stats = self.get_lichess_opening_stats(position).ok()?;
        let opening_move_contenders = lichess_opening_stats.get_opening_move_contenders(position);
        rand::seq::IteratorRandom::choose(opening_move_contenders.iter(), &mut rand::rng()).copied()
    }
}
