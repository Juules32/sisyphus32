use sisyphus32::fen::FenString;
use sisyphus32::move_masks::MoveMasks;
use sisyphus32::bit_move::BitMove;
use sisyphus32::move_generation::{MoveGeneration, PseudoLegal};
use sisyphus32::piece::PieceType;

#[test]
fn test_fen_parsing_and_move_generation() {
    MoveMasks::init();
    let position = FenString::kiwipete().parse().unwrap();
    let move_list = MoveGeneration::generate_captures::<BitMove, PseudoLegal>(&position);
    assert_eq!(move_list.len(), 8);

    for bit_move in move_list {
        assert_ne!(position.get_piece(bit_move.target()), PieceType::None)
    }
}
