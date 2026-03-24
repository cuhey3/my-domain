pub mod decide_first_player;
pub mod entry;
pub mod online_decide_first_player;
pub mod setting;

pub enum CommonPhase {
    Setting = 0,
    Entry,
    DecideFirstPlayer,
    OnlineDecideFirstPlayer,
    GameMain,
}
