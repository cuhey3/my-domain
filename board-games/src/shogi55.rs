use crate::shogi55::phases::Shogi55Phase;
use crate::shogi55::phases::game_main::GameMainPhase;
use crate::shogi55::structs::Shogi55Data;
use my_board_game::{GameData, GameSystem};
use std::cell::RefCell;
use std::rc::Rc;

pub mod draw_data;
mod phases;
pub mod structs;

pub fn init_shogi55(seed: u64) -> GameSystem {
    let mut data = Shogi55Data::default();
    data.set_seed(seed);
    GameSystem {
        phase_id: Shogi55Phase::GameMain as usize,
        phases: vec![Box::new(GameMainPhase::default())],
        game_data: Rc::new(RefCell::new(data)),
    }
}
