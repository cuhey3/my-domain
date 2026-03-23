pub mod decide_first_player;
pub mod entry;
pub mod game_main;
pub mod online_decide_first_player;
pub mod setting;

pub enum Connect4Phase {
    Setting = 0,
    Entry,
    DecideFirstPlayer,
    OnlineDecideFirstPlayer,
    GameMain,
}
