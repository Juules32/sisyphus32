use crate::{BitMove, BotGameError, Color, HistoryHeuristic, KillerMoves, Legal, MoveGeneration, MoveList, Piece, Position, ScoringMove, Search, Square, TranspositionTable, Uci};

pub struct BotGame {
    thinking_time: u128,
    bot_side: Color,
    to_move: Color,
    position: Position,
    search: Search,
    move_history: Vec<BitMove>,
}

impl BotGame {
    pub fn new(bot_side: Color, thinking_time: u128) -> Self {
        KillerMoves::reset();
        HistoryHeuristic::reset();
        TranspositionTable::reset();

        Self {
            thinking_time,
            bot_side,
            to_move: Color::White,
            position: Default::default(),
            search: Default::default(),
            move_history: Default::default()
        }
    }

    pub fn bot_side(&self) -> Color {
        self.bot_side
    }
    
    pub fn player_side(&self) -> Color {
        self.bot_side.opposite()
    }
    
    pub fn to_move(&self) -> Color {
        self.to_move
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
        let move_list = MoveGeneration::generate_moves::<BitMove, Legal>(&self.position);
        if move_list.contains(&bit_move) {
            self.make_move(bit_move);
            Ok(())
        } else {
            Err(BotGameError::IllegalUciMoveError)
        }
    }

    pub fn player_play_uci_move(&mut self, uci_move: &str) -> Result<(), BotGameError> {
        self.verify_player_to_move()?;
        let move_list = MoveGeneration::generate_moves::<BitMove, Legal>(&self.position);
        let bit_move = Uci::parse_move_string(&move_list, uci_move).map_err(|_| BotGameError::IllegalUciMoveError)?;
        self.make_move(bit_move);
        Ok(())
    }

    fn make_move(&mut self, bit_move: BitMove) {
        self.position.make_move(bit_move);
        self.move_history.push(bit_move);
    }

    pub fn is_checkmate(&self) -> bool {
        let move_list = MoveGeneration::generate_moves::<BitMove, Legal>(&self.position);
        move_list.is_empty()
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

    pub fn player_legal_moves(&self) -> Result<MoveList<BitMove>, BotGameError>  {
        self.verify_player_to_move()?;
        Ok(MoveGeneration::generate_moves::<BitMove, Legal>(&self.position))
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

    pub fn get_2d_board(&self) -> [(Option<Piece>, Square); 64] {
        self.position.pps
            .iter()
            .zip(Square::ALL_SQUARES.iter())
            .map(|(p, s)| (p.clone(), *s))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Should always succeed, both arrays are size 64")
    }
}

// TEST THIS ^^^^
