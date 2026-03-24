use rand::prelude::SmallRng;
use crate::framework::structs::common_player::CommonPlayer;
use crate::framework::structs::match_setting::{MatchMode, MatchSetting};
use crate::GameData;

#[derive(Default)]
pub struct CommonGameData {
    match_setting: MatchSetting,
    players: [CommonPlayer; 2],
    cpu_player_index: Option<usize>,
    online_player_index: Option<usize>,
    rng: Option<SmallRng>,
}

impl CommonGameData {
    pub fn set_setting(&mut self, match_setting: MatchSetting) {
        self.match_setting = match_setting;
    }

    pub fn set_default_cpu_player_index_if_necessary(&mut self) {
        if matches!(self.match_setting.get_match_mode(), MatchMode::Computer) {
            self.cpu_player_index = Some(1);
        }
    }

    pub fn set_default_online_player_index_if_necessary(&mut self) {
        if matches!(self.match_setting.get_match_mode(), MatchMode::Casual) {
            self.online_player_index = Some(1);
        }
    }

    pub fn has_cpu(&self) -> bool {
        matches!(self.match_setting.get_match_mode(), MatchMode::Computer)
    }

    pub fn is_online(&self) -> bool {
        matches!(self.match_setting.get_match_mode(), MatchMode::Casual)
    }

    fn player_is_cpu(&self, index: usize) -> bool {
        match self.match_setting.get_match_mode() {
            MatchMode::Computer => match self.cpu_player_index {
                Some(i) => i == index,
                _ => false,
            },
            _ => false,
        }
    }

    fn player_is_online(&self, index: usize) -> bool {
        match self.match_setting.get_match_mode() {
            MatchMode::Casual => match self.online_player_index {
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

    pub fn first_player_is_online(&self) -> bool {
        self.player_is_online(0)
    }

    pub fn second_player_is_online(&self) -> bool {
        self.player_is_online(1)
    }

    pub fn get_setting(&self) -> &MatchSetting {
        &self.match_setting
    }

    pub fn set_first_player(&mut self, first_player: CommonPlayer) {
        self.players[0] = first_player;
    }

    pub fn get_first_player(&mut self) -> &mut CommonPlayer {
        &mut self.players[0]
    }

    pub fn set_second_player(&mut self, first_player: CommonPlayer) {
        self.players[1] = first_player;
    }

    pub fn get_second_player(&mut self) -> &mut CommonPlayer {
        &mut self.players[1]
    }

    pub fn swap_player(&mut self) {
        self.players.swap(0, 1);
        self.cpu_player_index = Some(0);
        self.online_player_index = Some(0);
    }
}

impl GameData for CommonGameData {
    fn get_rng(&mut self) -> &mut Option<SmallRng> {
        &mut self.rng
    }

    fn set_rng(&mut self, rng: Option<SmallRng>) {
        self.rng = rng;
    }
}