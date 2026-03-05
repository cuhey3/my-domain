use crate::connect4::structs::board::Connect4Board;
use my_board_game::TwoPlayer;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

struct RandomWrapper {
    rng: SmallRng,
}
impl RandomWrapper {
    fn random_index(&mut self) -> usize {
        self.rng.random_range(1..8)
    }
}
fn init_board() -> Connect4Board {
    Connect4Board::default()
}

fn random_safe_move(board: &mut Connect4Board, random_wrapper: &mut RandomWrapper) {
    loop {
        match board.safe_move(random_wrapper.random_index()) {
            Ok(_) => break,
            Err(error) => match error.as_str() {
                "その列にはもう置けません" => continue,
                _ => break,
            },
        };
    }
}
fn board_state_tests(board: &Connect4Board, cloned: &Connect4Board) {
    assert_eq!(board.stone_count, cloned.stone_count);
    assert_eq!(board.last_x, cloned.last_x);
    assert_eq!(board.last_y, cloned.last_y);
    assert_eq!(board.last_stone, cloned.last_stone);
    assert_eq!(board.board, cloned.board);
    assert_eq!(board.column_height, cloned.column_height);
    assert_eq!(board.is_first_player_turn(), cloned.is_first_player_turn());
    assert_eq!(board.is_fill(), cloned.is_fill());
    assert_eq!(board.winner, cloned.winner);
}

#[test]
fn test_board_safe_move_only_one() {
    let mut board = init_board();
    let input_x = 0;
    let move_result = board.safe_move(input_x + 1);
    assert!(move_result.is_ok());
    assert_eq!(board.stone_count, 1);
    assert_eq!(board.last_x, input_x);
    assert_eq!(board.last_y, 0);
    assert_eq!(board.last_stone, TwoPlayer::First);
    assert_eq!(board.board[input_x][0], TwoPlayer::First);
    assert_eq!(board.column_height[input_x], 1);
    assert!(!board.is_first_player_turn());
    assert_eq!(board.get_next_player(), TwoPlayer::Second);
    assert!(!board.is_fill());
    assert_eq!(board.winner, TwoPlayer::None);
}

#[test]
fn test_board_winner() {
    let mut board = init_board();
    // 先手は1, 後手は2に積む
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(2), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(2), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(2), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(2), Err("勝敗が決しています".into()));
    assert_eq!(board.winner(), TwoPlayer::First);
}

#[test]
fn test_board_is_too_high() {
    let mut board = init_board();
    // 先手は1, 後手は2に積む
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(1), Err("その列にはもう置けません".into()));
}

#[test]
fn test_board_is_full() {
    let mut board = init_board();
    // 引き分けるためにずらしながら置く
    for i in 0..6 {
        if i % 3 == 1 {
            for j in 1..8 {
                assert_eq!(board.safe_move((j + 3) % 7 + 1), Ok(()));
            }
        } else if i % 3 == 2 {
            for j in 1..8 {
                assert_eq!(board.safe_move((j + 2) % 7 + 1), Ok(()));
            }
        } else {
            for j in 1..8 {
                assert_eq!(board.safe_move(j), Ok(()));
            }
        };
    }
    assert_eq!(board.safe_move(1), Err("この盤にはもう置けません".into()));
    assert_eq!(board.is_fill(), true);
    assert_eq!(board.winner(), TwoPlayer::None);
    assert_eq!(board.last_stone, TwoPlayer::Second);
}

#[test]
fn test_reject_last_2_move_small() {
    let mut board = init_board();
    assert_eq!(board.safe_move(1), Ok(()));
    let cloned = board.clone();
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.safe_move(1), Ok(()));
    assert_eq!(board.reject_last_2_move(), Ok(()));
    board_state_tests(&board, &cloned);
}

#[test]
// ランダムに20手追加したあと clone
// そのあとさらに2つランダム追加して reject_2_move
// clone と比較
// これを100回繰り返す
fn test_reject_last_2_move_random() {
    let mut random_wrapper = RandomWrapper {
        rng: SmallRng::seed_from_u64(0),
    };
    let mut test_count = 0;
    while test_count < 100 {
        let mut board = init_board();
        let mut random_count = 0;
        while random_count < 20 {
            random_safe_move(&mut board, &mut random_wrapper);
            // assert_eq!(board.safe_move(random_index()), Ok(()));
            random_count += 1;
        }
        // TODO
        // ランダム20手の状態で winner が決まっていたら正しく動作せず
        if board.winner.exist() {
            continue;
        }
        board.draw();
        let cloned = board.clone();
        random_safe_move(&mut board, &mut random_wrapper);
        random_safe_move(&mut board, &mut random_wrapper);

        // TODO
        // 追加のランダム2手で winner が決まっていたら正しく動作せず
        if board.winner.exist() {
            continue;
        }
        board.draw();
        assert_eq!(board.reject_last_2_move(), Ok(()));
        board_state_tests(&board, &cloned);
        test_count += 1;
    }
}

#[test]
// ランダムに20手追加したあと clone
// そのあとさらに2つランダム追加して reject_one_move
// clone と比較
// これを10回繰り返す
fn test_reject_last_one_move_random() {
    let mut random_wrapper = RandomWrapper {
        rng: SmallRng::seed_from_u64(0),
    };
    let mut test_count = 0;
    while test_count < 100 {
        let mut board = init_board();
        let mut random_count = 0;
        while random_count < 20 {
            random_safe_move(&mut board, &mut random_wrapper);
            // assert_eq!(board.safe_move(random_index()), Ok(()));
            random_count += 1;
        }
        // TODO
        // ランダム20手の状態で winner が決まっていたら正しく動作せず
        if board.winner.exist() {
            continue;
        }
        board.draw();
        let cloned = board.clone();
        random_safe_move(&mut board, &mut random_wrapper);

        // TODO
        // 追加のランダム2手で winner が決まっていたら正しく動作せず
        if board.winner.exist() {
            continue;
        }
        board.draw();
        assert_eq!(board.reject_last_one_move(), Ok(()));
        board_state_tests(&board, &cloned);
        test_count += 1;
    }
}
