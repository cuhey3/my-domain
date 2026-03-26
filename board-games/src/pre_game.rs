use crate::framework::structs::common_game_data::CommonGameData;
use crate::pre_game::phases::PreGamePhase;
use crate::pre_game::phases::select_game::SelectGamePhase;
use crate::pre_game::structs::PreGameData;
use crate::{GameData, GameSystem};
use std::cell::RefCell;
use std::rc::Rc;

pub mod phases;
pub mod structs;

pub fn init_pre_game(seed: u64) -> GameSystem {
    let mut data = CommonGameData::default();
    data.set_seed(seed);
    GameSystem {
        phase_id: PreGamePhase::SelectGame as usize,
        phases: vec![Box::new(SelectGamePhase::default())],
        game_data: Rc::new(RefCell::new(PreGameData::default())),
        common_game_data: Rc::new(RefCell::new(data)),
    }
}
