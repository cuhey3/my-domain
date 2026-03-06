pub mod board;
pub mod search_checkmate;
pub mod simulate;

use crate::Connect4DrawData;
use crate::framework::{GameData, MatchMode};
use rand::rngs::SmallRng;

#[derive(Default)]
pub struct Connect4Player {
    name: String,
    id: u64,
}

impl Connect4Player {
    pub fn new(name: String, id: u64) -> Self {
        Connect4Player { name, id }
    }
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

#[derive(Default)]
pub struct Connect4Data {
    players: [Connect4Player; 2],
    cpu_player_index: Option<usize>,
    setting: Connect4Setting,
    draw_data: Connect4DrawData,
    rng: Option<SmallRng>,
}

impl GameData for Connect4Data {
    fn get_rng(&mut self) -> &mut Option<SmallRng> {
        &mut self.rng
    }

    fn set_rng(&mut self, rng: Option<SmallRng>) {
        self.rng = rng;
    }
}

impl Connect4Data {
    pub fn set_first_player(&mut self, first_player: Connect4Player) {
        self.players[0] = first_player;
    }

    pub fn set_second_player(&mut self, second_player: Connect4Player) {
        self.players[1] = second_player;
    }

    pub fn get_first_player(&mut self) -> &mut Connect4Player {
        &mut self.players[0]
    }
    pub fn get_second_player(&mut self) -> &mut Connect4Player {
        &mut self.players[1]
    }
    pub fn swap_player(&mut self) {
        self.players.swap(0, 1);
        self.cpu_player_index = Some(0);
    }
    pub fn has_cpu(&self) -> bool {
        matches!(self.setting.match_mode, MatchMode::Computer)
    }

    pub fn set_default_cpu_player_index_if_necessary(&mut self) {
        if matches!(self.setting.match_mode, MatchMode::Computer) {
            self.cpu_player_index = Some(1);
        }
    }

    fn player_is_cpu(&self, index: usize) -> bool {
        match self.setting.match_mode {
            MatchMode::Computer => match self.cpu_player_index {
                Some(i) => i == index,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn first_player_is_cpu(&self) -> bool {
        self.player_is_cpu(0)
    }
    pub fn second_player_is_cpu(&self) -> bool {
        self.player_is_cpu(1)
    }
    pub fn set_setting(&mut self, connect4_setting: Connect4Setting) {
        self.setting = connect4_setting;
    }

    pub fn get_setting(&self) -> &Connect4Setting {
        &self.setting
    }
}

#[derive(Default, Clone, Copy)]
pub struct Connect4Setting {
    match_mode: MatchMode,
    // 待った可否
    enable_do_over: bool,
    // 評価値表示有無
    with_eval_value: bool,
}

impl Connect4Setting {
    pub fn set_enable_do_over(&mut self, enable_do_over: bool) {
        self.enable_do_over = enable_do_over;
    }
    pub fn set_with_eval_value(&mut self, with_eval_value: bool) {
        self.with_eval_value = with_eval_value;
    }
    pub fn set_cpu_mode(&mut self, has_cpu: bool) {
        if has_cpu {
            self.match_mode = MatchMode::Computer;
        } else {
            self.match_mode = MatchMode::Offline;
        }
    }
    pub fn get_enable_do_over(&self) -> bool {
        self.enable_do_over
    }

    pub fn get_with_eval_value(&self) -> bool {
        self.with_eval_value
    }
}
