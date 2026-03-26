mod connect4;
pub mod pre_game;
mod runner;
mod shogi55;

use crate::runner::BoardGameRunner;
use board_games::BoardGames;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut board_game_runner = BoardGameRunner::new_with_name(BoardGames::PreGame);
    // let mut board_game_runner = BoardGameRunner::new_for_dev();
    let result = board_game_runner.run().await;
    println!("{:?}", result);
    Ok(())
}
