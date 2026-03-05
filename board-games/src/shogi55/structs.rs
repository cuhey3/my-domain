use my_board_game::GameData;
use rand::prelude::SmallRng;

pub mod board;
pub mod piece;

pub mod simulate;

mod board_inner;
mod piece_info;
pub mod possibility;

#[derive(Default)]
pub struct Shogi55Data {
    rng: Option<SmallRng>,
}

impl GameData for Shogi55Data {
    fn get_rng(&mut self) -> &mut Option<SmallRng> {
        &mut self.rng
    }

    fn set_rng(&mut self, rng: Option<SmallRng>) {
        self.rng = rng;
    }
}
