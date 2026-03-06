use crate::draw_data::Shogi55DrawData;
use crate::framework::GameData;
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
    draw_data: Shogi55DrawData,
}

impl Shogi55Data {
    pub fn get_draw_data(&self) -> &Shogi55DrawData {
        &self.draw_data
    }
}

impl GameData for Shogi55Data {
    fn get_rng(&mut self) -> &mut Option<SmallRng> {
        &mut self.rng
    }

    fn set_rng(&mut self, rng: Option<SmallRng>) {
        self.rng = rng;
    }
}
