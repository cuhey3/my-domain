use crate::phases::game_main::GameMain;
use crate::structs::BoPGameData;
use board_games::framework::structs::common_game_data::CommonGameData;
use board_games::{GameData, GameSystem};
use std::cell::RefCell;
use std::rc::Rc;

mod phases;
pub mod structs;

pub fn init_bop(seed: u64) -> GameSystem {
    let mut game_data = BoPGameData::default();
    game_data.set_seed(seed);
    GameSystem {
        phase_id: 0,
        phases: vec![Box::new(GameMain::default())],
        game_data: Rc::new(RefCell::new(game_data)),
        common_game_data: Rc::new(RefCell::new(CommonGameData::default())),
    }
}
