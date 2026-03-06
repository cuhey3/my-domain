pub mod draw_data;
mod phases;
pub mod structs;

use crate::connect4::phases::{
    decide_first_player::DecideFirstPlayerPhase, entry::EntryPhase, game_main::GameMainPhase,
    setting::SettingPhase,
};
use crate::connect4::structs::Connect4Data;
use crate::framework::{GameData, GameSystem};
use phases::Connect4Phase;
use std::cell::RefCell;
use std::rc::Rc;
// 0. 設定入力
// 1. エントリー（ゆくゆくはマッチング処理）
// (1-a. Setting のすり合わせ)
// 2. 順番決め
// 3. 初期状態構築
// 4. プレイヤー着手
// 5. 条件判定/状態遷移(4と5を繰り返す)
// 6. ゲーム終了

pub fn init_connect4(seed: u64) -> GameSystem {
    let mut data = Connect4Data::default();
    data.set_seed(seed);
    GameSystem {
        phase_id: Connect4Phase::Setting as usize,
        phases: vec![
            Box::new(SettingPhase::default()),
            Box::new(EntryPhase::default()),
            Box::new(DecideFirstPlayerPhase::default()),
            Box::new(GameMainPhase::default()),
        ],
        game_data: Rc::new(RefCell::new(data)),
    }
}
