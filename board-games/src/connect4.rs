pub mod draw_data;
mod phases;
pub mod structs;

use crate::connect4::phases::game_main::GameMainPhase;
use crate::connect4::structs::Connect4Data;
use crate::framework::phases::CommonPhase;
use crate::framework::phases::decide_first_player::CommonDecideFirstPlayerPhase;
use crate::framework::phases::entry::CommonEntryPhase;
use crate::framework::phases::online_decide_first_player::CommonOnlineDecideFirstPlayerPhase;
use crate::framework::phases::setting::CommonSettingPhase;
use crate::framework::{GameData, GameSystem};
use std::cell::RefCell;
use std::rc::Rc;
use crate::framework::structs::common_game_data::CommonGameData;

pub fn init_connect4(seed: u64) -> GameSystem {
    let mut data = CommonGameData::default();
    data.set_seed(seed);
    GameSystem {
        phase_id: CommonPhase::Setting as usize,
        phases: vec![
            Box::new(CommonSettingPhase::default()),
            Box::new(CommonEntryPhase::default()),
            Box::new(CommonDecideFirstPlayerPhase::default()),
            Box::new(CommonOnlineDecideFirstPlayerPhase::default()),
            Box::new(GameMainPhase::default()),
        ],
        game_data: Rc::new(RefCell::new(Connect4Data::default())),
        common_game_data: Rc::new(RefCell::new(data)),
    }
}
