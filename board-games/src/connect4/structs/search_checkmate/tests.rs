use crate::connect4::structs::board::Connect4Board;
use crate::connect4::structs::search_checkmate::SearchCheckmate;
use my_board_game::TwoPlayer;

fn draw(board: &Connect4Board) {
    println!("1 2 3 4 5 6 7");
    for y in (0..6).rev() {
        let row_string = (0..7)
            .map(|x| match board.get_board()[x][y] {
                TwoPlayer::None => "  ",
                TwoPlayer::First => "■ ",
                TwoPlayer::Second => "□ ",
            })
            .collect::<String>();
        println!("{}", row_string);
    }
    println!("1 2 3 4 5 6 7");
}

fn init_board() -> Connect4Board {
    Connect4Board::default()
}
fn specific_board() -> Connect4Board {
    let mut board = Connect4Board::default();
    let moves = [
        1, 1, 1, 1, 1, 3, 2, 2, 3, 3, 4, 3, 3, 4, 3, 5, 4, 4, 4, 4, 5, 6, 5, 5, 5, 6, 5, 6, 6, 6,
        7, 7, 7,
    ];
    for move_index in moves.iter() {
        board.safe_move(*move_index).unwrap();
    }
    board
}
fn checkmated_board() -> Connect4Board {
    let mut board = Connect4Board::default();
    let moves = [4, 3, 3, 4, 5, 7, 5, 5, 6];
    for move_index in moves.iter() {
        board.safe_move(*move_index).unwrap();
    }
    board
}
#[test]
fn test_nest_search() {
    let mut board = init_board();
    let moves = [4, 1, 5, 1, 3, 1];
    let expects = [
        (TwoPlayer::None, vec![1, 2, 3, 4, 5, 6, 7]), // 後手の手番: 何を指しても誰も勝ちは決まらない
        (TwoPlayer::None, vec![1, 2, 3, 4, 5, 6, 7]), // 先手の手番: 何を指しても誰も勝ちは決まらない
        (TwoPlayer::None, vec![3, 6]), // 後手の手番: 詰めろが発生(3か6を指さないと詰む ※厳密には合法手と比較しないと詰めろかは不明)
        (TwoPlayer::First, vec![3]), // 先手の手番: 詰み発生(少なくとも3を指せば詰む ※1手しか教えてくれないのは仕様)
        (TwoPlayer::First, vec![1, 2, 3, 4, 5, 6, 7]), // 後手の手番: 詰まされている（何を指しても先手の勝ち）
        (TwoPlayer::First, vec![2]),                   // 先手の手番: 2を指せば勝ち
    ];
    for i in 0..6 {
        board.safe_move(moves[i]).expect("move failed");
        let mut search_checkmate = SearchCheckmate::new(board.clone(), 5);
        let result = search_checkmate.search();
        assert_eq!(result, expects[i]);
    }
}

#[test]
fn test_specific_board() {
    let board = specific_board();
    let result = SearchCheckmate::new(board, 7).search();
    assert_eq!(result, (TwoPlayer::Second, vec![1]));
}

#[test]
fn test_checkmated_board() {
    let board = checkmated_board();
    draw(&board);
    let result = SearchCheckmate::new(board, 7).search();
    assert_eq!(result, (TwoPlayer::Second, vec![4]));
}
