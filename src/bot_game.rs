use std::{collections::HashMap};

use crate::{BitMove, BotGameError, Color, HistoryHeuristic, KillerMoves, Legal, MoveGeneration, MoveList, Piece, Position, ScoringMove, Search, Square, TranspositionTable, Uci};

pub struct BotGame {
    thinking_time: u128,
    player_side: Color,
    position: Position,
    search: Search,
    move_history: Vec<BitMove>,
    legal_moves: MoveList<BitMove>,
}

impl Default for BotGame {
    fn default() -> Self {
        Self::new(Color::Black, 5000)
    }
}

impl BotGame {
    pub fn new(player_side: Color, thinking_time: u128) -> Self {
        KillerMoves::reset();
        HistoryHeuristic::reset();
        TranspositionTable::reset();
        
        Self {
            thinking_time,
            player_side,
            position: Position::starting_position(),
            search: Default::default(),
            move_history: Default::default(),
            legal_moves: MoveGeneration::generate_moves::<BitMove, Legal>(&Position::starting_position())
        }
    }

    pub fn bot_side(&self) -> Color {
        self.player_side.opposite()
    }
    
    pub fn player_side(&self) -> Color {
        self.player_side
    }
    
    pub fn to_move(&self) -> Color {
        self.position.side
    }

    pub fn bot_to_move(&self) -> bool {
        self.bot_side() == self.to_move()
    }
    
    pub fn player_to_move(&self) -> bool {
        self.player_side() == self.to_move()
    }

    pub fn set_thinking_time(&mut self, thinking_time: u128) -> Result<(), BotGameError> {
        self.verify_player_to_move()?;
        self.thinking_time = thinking_time;
        Ok(())
    }

    pub fn bot_play_move(&mut self) -> Result<ScoringMove, BotGameError> {
        self.verify_bot_to_move()?;
        let best_move = self.search.go(&self.position, None, Some(self.thinking_time));
        self.make_move(best_move.bit_move);
        Ok(best_move)
    }

    pub fn player_play_bit_move(&mut self, bit_move: BitMove) -> Result<(), BotGameError> {
        self.verify_player_to_move()?;
        if self.get_legal_moves().contains(&bit_move) {
            self.make_move(bit_move);
            Ok(())
        } else {
            Err(BotGameError::IllegalUciMoveError)
        }
    }

    pub fn player_play_uci_move(&mut self, uci_move: &str) -> Result<(), BotGameError> {
        self.verify_player_to_move()?;
        let bit_move = Uci::parse_move_string(&self.legal_moves, uci_move).map_err(|_| BotGameError::IllegalUciMoveError)?;
        self.make_move(bit_move);
        Ok(())
    }

    fn make_move(&mut self, bit_move: BitMove) {
        self.position.make_move(bit_move);
        self.move_history.push(bit_move);
        self.legal_moves = MoveGeneration::generate_moves::<BitMove, Legal>(&self.position);
    }

    pub fn is_checkmate(&self) -> bool {
        self.get_legal_moves().is_empty()
    }

    pub fn bot_won(&self) -> bool {
        self.is_checkmate() && self.player_to_move()
    }

    pub fn player_won(&self) -> bool {
        self.is_checkmate() && self.bot_to_move()
    }

    pub fn white_won(&self) -> bool {
        self.is_checkmate() && self.to_move() == Color::Black
    }

    pub fn black_won(&self) -> bool {
        self.is_checkmate() && self.to_move() == Color::White
    }

    pub fn player_legal_moves(&self) -> Result<&MoveList<BitMove>, BotGameError>  {
        self.verify_player_to_move()?;
        Ok(self.get_legal_moves())
    }

    pub fn get_legal_moves(&self) -> &MoveList<BitMove> {
        &self.legal_moves
    }

    fn verify_side_to_move(&self, side: Color) -> Result<(), BotGameError> {
        if self.to_move() == side {
            Ok(())
        } else {
            Err(BotGameError::IllegalActionError)
        }
    }

    fn verify_player_to_move(&self) -> Result<(), BotGameError> {
        self.verify_side_to_move(self.player_side())
    }

    fn verify_bot_to_move(&self) -> Result<(), BotGameError> {
        self.verify_side_to_move(self.bot_side())
    }

    #[cfg(feature = "bb_array")]
    pub fn get_2d_board(&self) -> [Option<Piece>; 64] {
        self.position.pps
    }

    #[cfg(feature = "bb_array")]
    pub fn get_piece_map(&self) -> HashMap<Square, Piece> {
        let mut piece_map = HashMap::new();

        for (index, piece) in self.position.pps.iter().enumerate() {
            if let Some(piece) = piece {
                let square = Square::from(index as u8);
                piece_map.insert(square, *piece);
            }
        }

        piece_map
    }

    pub fn get_move_history(&self) -> &[BitMove] {
        &self.move_history
    }

    pub fn get_last_move(&self) -> Option<BitMove> {
        self.get_move_history().last().copied()
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn in_check(&self) -> bool {
        self.get_position().in_check(self.to_move())
    }

    pub fn get_king_square(&self, side: Color) -> Square {
        match side {
            Color::White => self.position.bitboards[Piece::WK].into(),
            Color::Black => self.position.bitboards[Piece::BK].into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn making_moves_changes_color() {
        let mut bot_game = BotGame::new(Color::White, 5000);
        bot_game.set_thinking_time(100).unwrap();
        assert_eq!(bot_game.to_move(), Color::White);
        assert!(bot_game.player_to_move());
        bot_game.player_play_bit_move(bot_game.player_legal_moves().unwrap().first()).unwrap();
        assert_eq!(bot_game.to_move(), Color::Black);
        assert!(bot_game.bot_to_move());
        bot_game.bot_play_move().unwrap();
        assert_eq!(bot_game.to_move(), Color::White);
        assert!(bot_game.player_to_move());
    }

    #[test]
    fn initial_bot_game_has_moves() {
        let bot_game = BotGame::new(Color::White, 1000);
        assert!(bot_game.get_legal_moves().len() > 0);
    }

    #[cfg(feature = "bb_array")]
    #[test]
    fn get_2d_board_returns_array_of_tuples() {
        let bot_game = BotGame::new(Color::Black, 5000);
        let piece_positions = bot_game.get_2d_board();
        let piece_map = bot_game.get_piece_map();
        assert_eq!(piece_positions[Square::G8], Some(Piece::BN));
        let piece_set_entry = piece_map.get(&Square::D1);
        assert!(piece_set_entry.is_some_and(|p| *p == Piece::WQ));
    }
}
