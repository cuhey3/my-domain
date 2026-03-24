mod connect4;
mod runner;
mod shogi55;

use crate::BoardGames::{Connect4, Shogi55};
use crate::runner::BoardGameRunner;

enum BoardGames {
    Connect4,
    Shogi55,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut board_game_runner = BoardGameRunner::new_with_name(Shogi55);
    // let mut board_game_runner = BoardGameRunner::new_for_dev();
    let result = board_game_runner.run().await;
    println!("{:?}", result);
    Ok(())
}
