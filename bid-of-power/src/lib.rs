use crate::phases::game_main::GameMain;
use crate::structs::BoPGameData;
use board_games::framework::phases::CommonPhase;
use board_games::framework::phases::decide_first_player::CommonDecideFirstPlayerPhase;
use board_games::framework::phases::entry::CommonEntryPhase;
use board_games::framework::phases::online_decide_first_player::CommonOnlineDecideFirstPlayerPhase;
use board_games::framework::phases::setting::CommonSettingPhase;
use board_games::framework::structs::common_game_data::CommonGameData;
use board_games::{GameData, GameSystem};
use std::cell::RefCell;
use std::rc::Rc;

mod phases;
pub mod structs;

pub fn init_bop(seed: u64) -> GameSystem {
    let mut data = CommonGameData::default();

    data.set_seed(seed);

    GameSystem {
        phase_id: CommonPhase::Setting as usize,
        phases: vec![
            Box::new(CommonSettingPhase::default()),
            Box::new(CommonEntryPhase::default()),
            Box::new(CommonDecideFirstPlayerPhase::default()),
            Box::new(CommonOnlineDecideFirstPlayerPhase::default()),
            Box::new(GameMain::default()),
        ],
        game_data: Rc::new(RefCell::new(BoPGameData::default())),
        common_game_data: Rc::new(RefCell::new(data)),
    }
}
