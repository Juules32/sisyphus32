use crate::{board_state::BoardState, move_gen, pl, timer::Timer};

pub struct Engine {
    pub board_state: BoardState,
    // flags: Flags, // Use for debugging, verbose etc.
    pub nodes: u64
}

impl Engine {
    pub fn perft_test(&mut self, depth: u8) {
        let mut cumulative_nodes = 0_u64;
        let timer = Timer::new();
        self.nodes = 0;

        pl!("\n  Performance Test\n");

        let move_list = move_gen::generate_moves(&self.board_state);
        let castling_rights = self.board_state.castling_rights;
        for mv in move_list.iter() {
            if !self.board_state.make_move(*mv, castling_rights) { continue; }
            self.perft_driver(depth - 1);
            self.board_state.undo_move(*mv, castling_rights);

            pl!(format!("  Move: {:<5} Nodes: {}", mv.to_uci_string(), self.nodes));

            cumulative_nodes += self.nodes;
            self.nodes = 0;
        }

        pl!(format!("
 Depth: {}
 Nodes: {}
  Time: {} milliseconds\n",
            depth,
            cumulative_nodes,
            timer.get_time_passed_millis()
        ));
    }
    
    fn perft_driver(&mut self, depth: u8) {
        if depth == 0 {
            self.nodes += 1;
            return;
        }

        let move_list = move_gen::generate_moves(&self.board_state);
        
        let castling_rights = self.board_state.castling_rights;
        for mv in move_list.iter() {
            if !self.board_state.make_move(*mv, castling_rights) { continue; }
            self.perft_driver(depth - 1);
            self.board_state.undo_move(*mv, castling_rights);
        }
    }
}
