use crate::draw_data::Shogi55DrawData;

pub mod board;
pub mod piece;

pub mod simulate;

mod board_inner;
mod piece_info;
pub mod possibility;

#[derive(Default)]
pub struct Shogi55Data {
    draw_data: Shogi55DrawData,
}

impl Shogi55Data {
    pub fn get_draw_data(&self) -> &Shogi55DrawData {
        &self.draw_data
    }
}
