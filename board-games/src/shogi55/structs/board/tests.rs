use crate::shogi55::structs::board::{PieceInfo, Shogi55Board, Shogi55Move, Shogi55Place};
use crate::shogi55::structs::piece::Piece;
use my_board_game::TwoPlayer;
use std::str::FromStr;
use std::time::Instant;

fn get_all_place_from_in_hand_counts(board: &Shogi55Board) -> Vec<usize> {
    [
        (&TwoPlayer::First, false),
        (&TwoPlayer::First, true),
        (&TwoPlayer::Second, false),
        (&TwoPlayer::Second, true),
    ]
    .iter()
    .map(|(player, flag)| board.get_all_place_from_in_hand(player, *flag).len())
    .collect::<Vec<_>>()
}

fn time_start() -> Instant {
    Instant::now()
}

fn board_from_string(from: &str) -> Shogi55Board {
    let mut board = Shogi55Board::default();
    for (row_index, line) in from.lines().enumerate() {
        for (col_index, square) in line.split(',').enumerate() {
            let mut chars = square.chars();
            let Some(player_sign) = chars.next() else {
                continue;
            };
            let player = match player_sign {
                '▲' => TwoPlayer::First,
                '▽' => TwoPlayer::Second,
                _ => panic!(),
            };
            let piece = Piece::from_str(&chars.next().unwrap().to_string()).unwrap();
            let place = Shogi55Place::new(5 - col_index, row_index + 1);
            board
                .board_inner
                .placed_map_insert(place, PieceInfo::new(&player, &piece));
        }
    }
    board.init_all_possibilities();
    board
}

// 以下盤面で後手の合法手は42玉のみ
// 31玉は指してはいけない
// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// │   │▲角│▲飛│   │   │一
// ├───┼───┼───┼───┼───┤
// │   │▲金│▽王│   │   │二
// ├───┼───┼───┼───┼───┤
// │   │   │   │▲銀│▽飛│三
// ├───┼───┼───┼───┼───┤
// │▲歩│   │   │   │   │四
// ├───┼───┼───┼───┼───┤   歩
// │▲王│▲金│   │▲角│   │五 銀
// └───┴───┴───┴───┴───┘

#[test]
fn test_possible_moves1() {
    let board_string = ",▲角,▲飛,,
,▲金,▽王,,
,,,▲銀,▽飛
▲歩,,,,
▲王,▲金,,▲角,";
    let mut board = board_from_string(board_string);
    board.draw();
    let now = time_start();
    board.set_last_player_for_test(TwoPlayer::First);
    let all_possible_moves_second = board.get_all_possible_moves();
    board.clear_possible_moves_cache();
    board.set_last_player_for_test(TwoPlayer::Second);
    let all_possible_moves_first = board.get_all_possible_moves();
    assert_eq!(get_all_place_from_in_hand_counts(&board), [15, 10, 15, 13]);
    println!("{:#?}", all_possible_moves_first);
    // 普通に数えると23しかないが、3一の飛車は3二だけでなく3三、3四、3五にも指せるロジックになっている
    // (そうしないと3三に逃げた玉を捕まえられない。そもそも王手放置しているのでこの盤面は先手には渡らないが)
    assert_eq!(all_possible_moves_first.len(), 34);
    assert_eq!(all_possible_moves_second.len(), 1);
    assert_eq!(
        all_possible_moves_second[0].from,
        Some(Shogi55Place::new(3, 2))
    );
    assert_eq!(all_possible_moves_second[0].to, Shogi55Place::new(4, 2));
    assert!(now.elapsed().as_micros() < 500);
}

// 合法手は同銀しかない
// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// 歩│   │▽角│▲飛│   │▽王│一
// ├───┼───┼───┼───┼───┤
// │   │   │   │▽銀│▽歩│二
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │   │三
// ├───┼───┼───┼───┼───┤
// │▲王│▲金│▽金│▲銀│   │四
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │▲飛│五 角
// └───┴───┴───┴───┴───┘
#[test]
fn test_possible_moves2() {
    let board_string = ",▽角,▲飛,,▽王
,,,▽銀,▽歩
,,,,
▲王,▲金,▽金,▲銀,
,,,,▲飛";
    let mut board = board_from_string(board_string);
    board.draw();
    let now = time_start();
    board.set_last_player_for_test(TwoPlayer::First);
    let all_possible_moves_second = board.get_all_possible_moves();
    board.clear_possible_moves_cache();
    board.set_last_player_for_test(TwoPlayer::Second);
    let all_possible_moves_first = board.get_all_possible_moves();
    println!("all_possible_moves_first {:#?}", all_possible_moves_first);
    assert_eq!(all_possible_moves_first.len(), 31);
    assert_eq!(all_possible_moves_second.len(), 1);
    assert_eq!(
        all_possible_moves_second[0].from,
        Some(Shogi55Place::new(2, 2))
    );
    assert_eq!(all_possible_moves_second[0].to, Shogi55Place::new(3, 1));
    assert_eq!(get_all_place_from_in_hand_counts(&board), [15, 13, 15, 9]);
    board.set_last_player_for_test(TwoPlayer::First);
    let all_place_from_in_hand = board.get_all_place_from_in_hand(&TwoPlayer::Second, true);
    for place in all_place_from_in_hand {
        println!("test start {:#?}", place);
        let result = board.check_input_in_hand(&place, &Piece::Pawn);
        if place.col == 2 && place.row == 1 {
            assert!(result.is_ok());
        } else {
            assert_eq!(result, Err("玉が取られます".into()));
        }
        println!("test pass {:#?}", place);
    }
    assert!(now.elapsed().as_micros() < 800)
}

// 先手の詰み探索（合法手は32玉のみ）
// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// │   │▽金│   │   │▽王│一
// ├───┼───┼───┼───┼───┤
// │▽角│▲王│   │▽銀│▽歩│二
// ├───┼───┼───┼───┼───┤
// │▲歩│   │▲銀│   │   │三
// ├───┼───┼───┼───┼───┤
// │   │▲金│▲角│   │   │四
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │▲飛│五 飛
// └───┴───┴───┴───┴───┘
#[test]
fn test_possible_moves3() {
    let board_string = ",▽金,,,▽王
▽角,▲王,,▽銀,▽歩
▲歩,,▲銀,,
,▲金,▲角,,
,,,,▲飛";
    let mut board = board_from_string(board_string);
    board.draw();
    let now = time_start();
    board.set_last_player_for_test(TwoPlayer::First);
    let all_possible_moves_second = board.get_all_possible_moves();
    board.clear_possible_moves_cache();
    board.set_last_player_for_test(TwoPlayer::Second);
    let all_possible_moves_first = board.get_all_possible_moves();
    assert_eq!(all_possible_moves_first.len(), 0);
    assert_eq!(all_possible_moves_second.len(), 12);
    assert!(now.elapsed().as_micros() < 600);
    // panic!()
}

// ここから 4二角, 1三銀 と進んだ時に 5一角が "そこには動かせません"になる
// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// │▽飛│   │   │▽金│▽王│一
// ├───┼───┼───┼───┼───┤
// │▽歩│   │   │▽銀│▽歩│二
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │   │三
// ├───┼───┼───┼───┼───┤
// │▲金│   │▲角│▲銀│   │四
// ├───┼───┼───┼───┼───┤
// │▲王│   │   │   │▲飛│五 角
// └───┴───┴───┴───┴───┘
#[test]
fn test_possible_moves4() {
    let board_string = "▽飛,,,▽金,▽王
▽歩,,,▽銀,▽歩
,,,,
▲金,,▲角,▲銀,
▲王,,,,▲飛";
    let mut board = board_from_string(board_string);
    board.pieces_in_hand[TwoPlayer::First.get_index()].push(Piece::Bishop);
    let result = board.safe_move(Shogi55Move {
        player: TwoPlayer::First,
        in_hand: Some(Piece::Bishop),
        from: None,
        to: Shogi55Place::new(4, 2),
        promotion_flag: false,
    });
    assert!(result.is_ok());
    let result = board.safe_move(Shogi55Move {
        player: TwoPlayer::Second,
        in_hand: None,
        from: Some(Shogi55Place::new(2, 2)),
        to: Shogi55Place::new(1, 3),
        promotion_flag: false,
    });
    assert!(result.is_ok());

    board.draw();
    let now = time_start();
    let result = board.check_input_from_to(&Shogi55Place::new(4, 2), &Shogi55Place::new(5, 1));
    assert!(result.is_ok());
    board.set_last_player_for_test(TwoPlayer::First);
    let all_possible_moves_second = board.get_all_possible_moves();
    board.clear_possible_moves_cache();
    board.set_last_player_for_test(TwoPlayer::Second);
    let all_possible_moves_first = board.get_all_possible_moves();
    assert_eq!(all_possible_moves_first.len(), 26);
    assert_eq!(all_possible_moves_second.len(), 10);
    assert!(now.elapsed().as_micros() < 300);
    // panic!()
}

// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// 歩│   │▽角│   │   │▽王│一
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │   │二
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │▽歩│三
// ├───┼───┼───┼───┼───┤
// │▲王│▲角│▽金│▲銀│   │四
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │▲飛│五 飛
// └───┴───┴───┴───┴───┘
#[test]
fn test_possible_moves5() {
    let board_string = ",▽角,,,▽王
,,,,
,,,,▽歩
▲王,▲角,▽金,▲銀,
,,,,▲飛";
    let mut board = board_from_string(board_string);
    board.pieces_in_hand[TwoPlayer::First.get_index()].push(Piece::Pawn);
    board.pieces_in_hand[TwoPlayer::Second.get_index()].push(Piece::Pawn);
    board.draw();
    let now = time_start();
    board.set_last_player_for_test(TwoPlayer::First);
    let all_possible_moves_second = board.get_all_possible_moves();
    board.clear_possible_moves_cache();
    board.set_last_player_for_test(TwoPlayer::Second);
    let all_possible_moves_first = board.get_all_possible_moves();
    assert_eq!(all_possible_moves_first.len(), 34);
    assert_eq!(all_possible_moves_second.len(), 6);
    assert!(now.elapsed().as_micros() < 200);
    // panic!()
}

// この盤面から後手5三飛、先手同歩としたいが"玉が取られます"となったので何か失敗している
// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// │▽飛│▽角│▽銀│▽金│▽王│一
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │▽歩│二
// ├───┼───┼───┼───┼───┤
// │   │   │▲金│▲銀│   │三
// ├───┼───┼───┼───┼───┤
// │▲歩│   │▲角│   │   │四
// ├───┼───┼───┼───┼───┤
// │▲王│   │   │   │▲飛│五
// └───┴───┴───┴───┴───┘
#[test]
fn test_possible_moves6() {
    let board_string = "▽飛,▽角,▽銀,▽金,▽王
,,,,▽歩
,,▲金,▲銀,
▲歩,,▲角,,
▲王,,,,▲飛";
    let mut board = board_from_string(board_string);
    let now = time_start();
    let result = board.safe_move(Shogi55Move {
        player: TwoPlayer::Second,
        in_hand: None,
        from: Some(Shogi55Place::new(5, 1)),
        to: Shogi55Place::new(5, 3),
        promotion_flag: false,
    });
    board.draw();
    println!("{:?}", result);
    let result = board.check_input_from_to(&Shogi55Place::new(5, 4), &Shogi55Place::new(5, 3));
    assert!(result.is_ok());
    let all_possible_moves_first = board.get_all_possible_moves();
    println!("{:#?}", all_possible_moves_first);
    assert_eq!(all_possible_moves_first.len(), 21);
    assert!(now.elapsed().as_micros() < 300);
    // panic!()
}

// 1二歩は打ち歩詰め
// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// 歩│   │   │   │▽飛│▽王│一
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │   │二
// ├───┼───┼───┼───┼───┤
// │   │   │▽銀│▲銀│▽歩│三
// ├───┼───┼───┼───┼───┤
// │▲王│▲角│▽金│▲金│   │四
// ├───┼───┼───┼───┼───┤
// │   │   │   │   │▲飛│五 飛
// └───┴───┴───┴───┴───┘
#[test]
fn test_possible_moves7() {
    let board_string = ",,,▽飛,▽王
,,,,
,,▽銀,▲銀,▽歩
▲王,▲角,▽金,▲金,
,,,,▲飛";
    let mut board = board_from_string(board_string);
    board.draw();
    let now = time_start();
    let possible_moves_by_king = board
        .board_inner
        .get_possible_moves_of_king(&TwoPlayer::Second);
    let checked_with_reason = board
        .board_inner
        .get_checking_with_precondition(&TwoPlayer::Second, true);
    assert_eq!(get_all_place_from_in_hand_counts(&board), [15, 12, 15, 9]);
    println!("{:#?}", possible_moves_by_king);
    println!("{:#?}", checked_with_reason);
    board.set_last_player_for_test(TwoPlayer::Second);
    assert_eq!(
        board.check_input_in_hand(&Shogi55Place::new(1, 2), &Piece::Pawn),
        Err("打ち歩詰めです".into())
    );
    assert!(now.elapsed().as_micros() < 600)
    // panic!()
}

// 54金打のあとに同玉がなぜか"玉が取られます" 多分入力チェックの問題（ではなく多分合法手の問題、下参照）
// 評価値上は詰みとかではなかった
// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// │▽飛│   │   │   │▽王│一
// ├───┼───┼───┼───┼───┤
// │▲歩│   │   │▽銀│▽歩│二
// ├───┼───┼───┼───┼───┤
// │   │   │   │▽飛│   │三
// ├───┼───┼───┼───┼───┤
// │▽金│▲角│   │   │   │四
// ├───┼───┼───┼───┼───┤   金
// │   │▲王│   │▲角│   │五 銀
// └───┴───┴───┴───┴───┘

// 下記盤面から53飛ではなく41飛、当然の42銀に44金打で 「評価値: -10000」同玉が "玉が取られます" となるバグ
// 入力チェックの問題ではなさそう
// 5   4   3   2   1
// ┌───┬───┬───┬───┬───┐
// 金 │▽飛│   │   │   │▽王│一
// ├───┼───┼───┼───┼───┤
// │   │   │   │▽金│   │二
// ├───┼───┼───┼───┼───┤
// │▲歩│▲銀   │   │   │▽歩│三
// ├───┼───┼───┼───┼───┤
// │   │   │   │▲角│   │四
// ├───┼───┼───┼───┼───┤
// │▲王│   │▲飛│   │▽全│五 角
// └───┴───┴───┴───┴───┘
