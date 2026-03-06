use board_games::Shogi55Board;
use board_games::draw_data::{Shogi55DrawData, Shogi55DrawTask};
use board_games::framework::{Drawer, TwoPlayer};
use std::any::Any;

#[derive(Default)]
pub struct Shogi55Drawer {}

impl Shogi55Drawer {
    pub fn get_draw_data(board: &Shogi55Board) -> Vec<Vec<String>> {
        let mut data = vec![vec!["   ".into(); 6]; 6];
        board.get_placed_map().iter().for_each(|(place, info)| {
            data[place.get_row()][place.get_col()] = format!(
                "{}{}",
                match info.get_player() {
                    TwoPlayer::First => "▲",
                    TwoPlayer::Second => "▽",
                    TwoPlayer::None => " ",
                },
                info.get_piece().kanji()
            );
        });
        data
    }

    pub fn draw_board(board: &Shogi55Board) {
        let board_strings = Shogi55Drawer::get_draw_data(board);
        let mut hand_strings = [["  "; 10]; 2];
        for (i, item) in hand_strings.iter_mut().enumerate() {
            // ソートされてる前提だからここではソートしない
            // self.pieces_in_hand[i].sort_by_key(|p|*p as isize);
            let pieces_len = board.get_piece_in_hand()[i].len();
            for j in 0..pieces_len {
                if i == 1 {
                    item[j] = board.get_piece_in_hand()[i][j].kanji();
                } else {
                    item[10 - pieces_len + j] = board.get_piece_in_hand()[i][j].kanji();
                }
            }
        }
        println!(
            "    5   4   3   2   1
  ┌───┬───┬───┬───┬───┐   {}
{}│{}│{}│{}│{}│{}│一 {}
{}├───┼───┼───┼───┼───┤   {}
{}│{}│{}│{}│{}│{}│二 {}
{}├───┼───┼───┼───┼───┤   {}
{}│{}│{}│{}│{}│{}│三 {}
{}├───┼───┼───┼───┼───┤   {}
{}│{}│{}│{}│{}│{}│四 {}
{}├───┼───┼───┼───┼───┤   {}
{}│{}│{}│{}│{}│{}│五 {}
{}└───┴───┴───┴───┴───┘",
            hand_strings[0][0],
            hand_strings[1][0],
            board_strings[1][5],
            board_strings[1][4],
            board_strings[1][3],
            board_strings[1][2],
            board_strings[1][1],
            hand_strings[0][1],
            hand_strings[1][1],
            hand_strings[0][2],
            hand_strings[1][2],
            board_strings[2][5],
            board_strings[2][4],
            board_strings[2][3],
            board_strings[2][2],
            board_strings[2][1],
            hand_strings[0][3],
            hand_strings[1][3],
            hand_strings[0][4],
            hand_strings[1][4],
            board_strings[3][5],
            board_strings[3][4],
            board_strings[3][3],
            board_strings[3][2],
            board_strings[3][1],
            hand_strings[0][5],
            hand_strings[1][5],
            hand_strings[0][6],
            hand_strings[1][6],
            board_strings[4][5],
            board_strings[4][4],
            board_strings[4][3],
            board_strings[4][2],
            board_strings[4][1],
            hand_strings[0][7],
            hand_strings[1][7],
            hand_strings[0][8],
            hand_strings[1][8],
            board_strings[5][5],
            board_strings[5][4],
            board_strings[5][3],
            board_strings[5][2],
            board_strings[5][1],
            hand_strings[0][9],
            hand_strings[1][9],
        )
    }
}

impl Drawer for Shogi55Drawer {
    fn draw(&mut self, draw_data: Box<&mut dyn Any>) {
        let draw_data = draw_data.downcast_mut::<Shogi55DrawData>().unwrap();
        while let Some(task) = draw_data.take_task() {
            match task {
                Shogi55DrawTask::Question(message) => println!("{}", message),
                Shogi55DrawTask::Message(message) => println!("{}", message),
                Shogi55DrawTask::EvaluateValue(point) => println!("評価値: {}", point),
                Shogi55DrawTask::Board(board) => Shogi55Drawer::draw_board(&board),
                _ => {}
            }
        }
    }

    fn draw_error(&mut self, error: String) {
        println!("error: {}", error);
    }
}
