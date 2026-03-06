mod connect4;
mod runner;
mod shogi55;

use crate::BoardGames::{Connect4, Shogi55};
use crate::runner::BoardGameRunner;
enum BoardGames {
    Connect4,
    Shogi55,
}

fn main() {
    let mut board_game_runner = BoardGameRunner::new_with_name(Shogi55);
    let result = board_game_runner.run();
    println!("{:?}", result);
}
