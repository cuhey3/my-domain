mod connect4;
pub mod framework;
pub mod pre_game;
mod shogi55;

pub use crate::framework::GameData;
pub use crate::framework::GameSystem;
pub use crate::framework::input_util::convert_input;
pub use connect4::draw_data::Connect4DrawData;
pub use connect4::draw_data::Connect4DrawTask;
pub use connect4::init_connect4;
pub use connect4::structs::Connect4Data;
pub use connect4::structs::board::Connect4Board;
pub use shogi55::draw_data;
pub use shogi55::init_shogi55;
pub use shogi55::structs::board::Shogi55Board;
pub use shogi55::structs::board::Shogi55Place;

#[derive(Clone, Debug)]
pub enum BoardGames {
    PreGame,
    Connect4,
    Shogi55,
    BoP,
}

impl BoardGames {
    pub fn from_usize(index: usize) -> Self {
        match index {
            0 => Self::PreGame,
            1 => Self::Connect4,
            2 => Self::Shogi55,
            3 => Self::BoP,
            _ => panic!("Invalid board game index"),
        }
    }

    pub fn index_is_valid(index: usize) -> bool {
        index > 0 && index < 4
    }
}
