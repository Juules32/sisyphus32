use crate::{board_state::BoardState, fen, move_gen, pl, timer::Timer};

pub struct PerftResult {
    depth: u8,
    nodes: u64,
    time: u128,
}

struct PerftPosition {
    name: &'static str,
    fen: &'static str,
    depth: u8,
    target_nodes: u64
}

static PERFT_POSITIONS: [PerftPosition; 5] = [
    PerftPosition {
        name: "Starting Position",
        fen: fen::STARTING_POSITION,
        depth: 6,
        target_nodes: 119_060_324
    },
    PerftPosition {
        name: "Kiwipete Position",
        fen: fen::KIWIPETE_POSITION,
        depth: 5,
        target_nodes: 193_690_690
    },
    PerftPosition {
        name: "Rook Position",
        fen: fen::ROOK_POSITION,
        depth: 7,
        target_nodes: 178_633_661
    },
    PerftPosition {
        name: "Tricky Position",
        fen: fen::TRICKY_POSITION,
        depth: 6,
        target_nodes: 706_045_033
    },
    PerftPosition {
        name: "Tricky Position 2",
        fen: fen::TRICKY_POSITION_2,
        depth: 5,
        target_nodes: 89_941_194
    },
];

pub fn perft_test(board_state: &mut BoardState, depth: u8, print_result: bool) -> PerftResult {
    let mut current_nodes = 0_u64;
    let mut cumulative_nodes = 0_u64;
    let timer = Timer::new();

    if print_result { pl!("\n  Performance Test\n"); }

    let move_list = move_gen::generate_moves(&board_state);
    let castling_rights = board_state.castling_rights;
    for mv in move_list.iter() {
        if !board_state.make_move(*mv, castling_rights) { continue; }
        perft_driver(board_state, depth - 1, &mut current_nodes);
        board_state.undo_move(*mv, castling_rights);

        if print_result {
            pl!(format!("  Move: {:<5} Nodes: {}", mv.to_uci_string(), current_nodes));
        }

        cumulative_nodes += current_nodes;
        current_nodes = 0;
    }

    let perft_result = PerftResult {
        depth,
        nodes: cumulative_nodes,
        time: timer.get_time_passed_millis(),
    };

    if print_result {
        pl!(format!("
Depth: {}
Nodes: {}
Time: {} milliseconds\n",
            perft_result.depth,
            perft_result.nodes,
            perft_result.time
        ));
    }

    perft_result
}

fn perft_driver(board_state: &mut BoardState, depth: u8, nodes: &mut u64) {
    if depth <= 0 {
        *nodes += 1;
        return;
    }

    let move_list = move_gen::generate_moves(&board_state);
    
    let castling_rights = board_state.castling_rights;
    for mv in move_list.iter() {
        if !board_state.make_move(*mv, castling_rights) { continue; }
        perft_driver(board_state, depth - 1, nodes);
        board_state.undo_move(*mv, castling_rights);
    }
}

pub fn perft_tests() {
    println!("\n    Printing performance test results:");
    println!("  |-----------------------------------------------------------------|");
    println!("  | {:<18} | {:<6} | {:<10} | {:<6} | {:<11} |", "Position", "Depth", "Nodes", "Time", "Performance");
    println!("  |-----------------------------------------------------------------|");

    for perft_position in &PERFT_POSITIONS {
        let mut board_state = fen::parse(perft_position.fen).expect("FEN parser could not parse given position!");
        let perft_result = perft_test(&mut board_state, perft_position.depth, false);
        if perft_result.nodes != perft_position.target_nodes {
            panic!("Perft test of {} did not get the target nodes!", perft_position.name);
        }
        let score = perft_result.nodes as u128 / perft_result.time;
        println!("  | {:<18} | {:<6} | {:<10} | {:<6} | {:<11} |", perft_position.name, perft_position.depth, perft_result.nodes, perft_result.time, score);
    }

    println!("  |-----------------------------------------------------------------|");
}
