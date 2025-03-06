use sisyphus32::fen::FenString;
use sisyphus32::bit_move::BitMove;
use sisyphus32::move_generation::{Legal, MoveGeneration, PseudoLegal};
use sisyphus32::position::Position;
use sisyphus32::zobrist::ZobristKey;

#[test]
fn test_fen_parsing_and_move_generation() {
    let position = FenString::kiwipete().parse().unwrap();
    let move_list = MoveGeneration::generate_captures::<BitMove, PseudoLegal>(&position);
    assert_eq!(move_list.len(), 8);

    for bit_move in move_list {
        assert_ne!(position.get_piece_option(bit_move.target()), None);
    }
}

#[test]
fn test_zobrist_key_incremental_updates_are_correct() {
    let position = Position::starting_position();

    for legal_move in MoveGeneration::generate_moves::<BitMove, Legal>(&position) {
        let mut position_copy = position.clone();
        position_copy.make_move(legal_move);
        assert_eq!(position_copy.zobrist_key, ZobristKey::generate(&position_copy), "{}", position_copy);
    }
}
