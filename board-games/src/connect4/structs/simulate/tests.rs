use crate::connect4::structs::board::Connect4Board;
use crate::connect4::structs::simulate::Connect4Simulate;
use crate::framework::TwoPlayer;

fn init_board() -> Connect4Board {
    Connect4Board::default()
}
#[test]
fn test_winner_confirmed() {
    let mut board = init_board();
    board.safe_move(1).unwrap();
    board.safe_move(2).unwrap();
    board.safe_move(1).unwrap();
    board.safe_move(2).unwrap();
    board.safe_move(1).unwrap();
    board.safe_move(2).unwrap();
    let mut simulate =
        Connect4Simulate::new(&board, 0, (TwoPlayer::None, vec![1, 2, 3, 4, 5, 6, 7]));
    simulate.simulate();
    assert_eq!(simulate.result[0], (1, Some(100)));
    assert_eq!(simulate.get_best_move_with_checkmate_search(), Some(1));
}
