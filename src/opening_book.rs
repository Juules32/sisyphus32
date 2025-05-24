use std::time::Duration;
use serde::Deserialize;

#[cfg(feature = "ureq")]
use ureq::{Agent, Error};

use crate::{bit_move::BitMove, color::Color, fen::FenString, move_generation::{Legal, MoveGeneration}, position::Position, uci::Uci};

const NUM_GAMES_THRESHOLD: u32 = 1_000;
const WINRATE_THRESHOLD: f32 = 0.35;
const OPENING_BOOK_TIMEOUT_MS: u64 = 500;

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
    fn winrate(&self, side: Color) -> f32 {
        let (wins, losses) = match side {
            Color::White => (self.white, self.black),
            Color::Black => (self.black, self.white),
        };

        if wins + losses == 0 {
            0.0
        } else {
            wins as f32 / (wins + losses) as f32
        }
    }

    fn is_candidate(&self, side: Color) -> bool {
        let total_games = self.white + self.draws + self.black;
        total_games >= NUM_GAMES_THRESHOLD && self.winrate(side) > WINRATE_THRESHOLD
    }
}

impl LichessOpeningStats {
    fn pick_best_opening_move(&self, position: &Position) -> Option<BitMove> {
        let legal_moves = MoveGeneration::generate_moves::<BitMove, Legal>(position);

        self.moves
            .iter()
            .filter(|m| m.is_candidate(position.side))
            .filter_map(|m| {
                let parsed = Uci::parse_move_string(&legal_moves, &m.uci).ok()?;
                Some((parsed, m.winrate(position.side)))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(bit_move, _)| bit_move)
    }
}

#[cfg(feature = "ureq")]
pub struct OpeningBook {
    agent: Agent,
}

#[cfg(feature = "ureq")]
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
        let stats = self.get_lichess_opening_stats(position).ok()?;
        stats.pick_best_opening_move(position)
    }
}

#[cfg(feature = "ureq")]
impl Default for OpeningBook {
    fn default() -> Self {
        Self {
            agent: Agent::config_builder()
                .timeout_global(Some(Duration::from_millis(OPENING_BOOK_TIMEOUT_MS)))
                .build()
                .into(),
        }
    }
}

#[cfg(not(feature = "ureq"))]
pub struct OpeningBook;

#[cfg(not(feature = "ureq"))]
impl OpeningBook {
    pub fn get_move(&self, _position: &Position) -> Option<BitMove> {
        None
    }
}

#[cfg(not(feature = "ureq"))]
impl Default for OpeningBook {
    fn default() -> Self {
        Self
    }
}
