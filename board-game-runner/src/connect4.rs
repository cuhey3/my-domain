use board_games::{Connect4Board, Connect4DrawData, Connect4DrawTask};
use my_board_game::{Drawer, TwoPlayer};
use std::any::Any;

#[derive(Default)]
pub struct Connect4Drawer {}
impl Connect4Drawer {
    pub fn draw_board(board: &Connect4Board) {
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
}

impl Drawer for Connect4Drawer {
    fn draw(&mut self, draw_data: Box<&mut dyn Any>) {
        let draw_data = draw_data.downcast_mut::<Connect4DrawData>().unwrap();
        while let Some(task) = draw_data.take_task() {
            match task {
                Connect4DrawTask::Question(message) => println!("{}", message),
                Connect4DrawTask::Message(message) => println!("{}", message),
                Connect4DrawTask::EvaluateValue(point) => println!("評価値: {}", point),
                Connect4DrawTask::Board(board) => Connect4Drawer::draw_board(&board),
                _ => {}
            }
        }
    }

    fn draw_error(&mut self, error: String) {
        println!("error: {}", error);
    }
}
