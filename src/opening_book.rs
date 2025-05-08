use std::time::Duration;
use ureq::{Agent, Error};
use serde::Deserialize;
use rand::seq::IteratorRandom;

use crate::{bit_move::BitMove, color::Color, fen::FenString, move_generation::{Legal, MoveGeneration}, position::Position, uci::Uci};

const NUM_GAMES_THRESHOLD: u32 = 1_000;
const WINRATE_THRESHOLD: f32 = 0.35;
const OPENING_BOOK_TIMEOUT_SECS: u64 = 1;

#[derive(Deserialize)]
struct LichessOpeningStats {
    white: u32,
    draws: u32,
    black: u32,
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

pub struct OpeningBook {
    agent: Agent
}

impl OpeningBook {
    fn get_lichess_opening_stats(&self, position: &Position) -> Result<LichessOpeningStats, Error> {
        let fen_string = FenString::from(position);
        let fen_with_replaced_spaces = fen_string.to_string().replace(" ", "_");
        let uri = &format!("https://explorer.lichess.ovh/masters?fen={fen_with_replaced_spaces}");
        let resp = self.agent.get(uri).call();
        let body = resp?.body_mut().read_to_string()?;
        let lichess_opening_stats: LichessOpeningStats = serde_json::from_str(&body)
            .map_err(|err| ureq::Error::Json(err))?;
        Ok(lichess_opening_stats)
    }

    pub fn get_move(&self, position: &Position) -> Option<BitMove> {
        let lichess_opening_stats = self.get_lichess_opening_stats(position).ok()?;
        let opening_move_contenders = lichess_opening_stats.get_opening_move_contenders(position);
        opening_move_contenders.iter().choose(&mut rand::rng()).copied()
    }
}

impl Default for OpeningBook {
    fn default() -> Self {
        Self {
            agent: Agent::config_builder()
                .timeout_global(Some(Duration::from_secs(OPENING_BOOK_TIMEOUT_SECS)))
                .build()
                .into()
        }
    }
}
