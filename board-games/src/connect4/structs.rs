pub mod board;
pub mod search_checkmate;
pub mod simulate;

use crate::Connect4DrawData;

#[derive(Default)]
pub struct Connect4Data {
    draw_data: Connect4DrawData,
}

impl Connect4Data {
    pub fn get_draw_data(&self) -> &Connect4DrawData {
        &self.draw_data
    }
}
