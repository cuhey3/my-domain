use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GameSystem {
    pub phase_id: usize,
    pub phases: Vec<Box<dyn Phase>>,
    pub game_data: Rc<RefCell<dyn Any>>,
}

impl GameSystem {
    pub fn get_phase(&mut self) -> Option<&mut Box<dyn Phase>> {
        self.phases
            .iter_mut()
            .find(|phase| phase.get_phase_id() == self.phase_id)
    }
}

pub enum PhaseType {
    Setting,
    Entry,
    DecideFirstPlayer,
    GameMain,
}

pub trait DrawData {
    fn has_task(&self) -> bool;
}

pub trait Drawer {
    fn draw(&mut self, draw_data: Box<&mut dyn Any>);
    fn draw_error(&mut self, error: String);
    fn clear_error(&mut self) {}
}

pub enum AnswerType {
    Input,
    Skip,
    Wait,
}

pub trait Phase {
    fn get_phase_id(&self) -> usize;
    fn phase_type(&self) -> Option<PhaseType>;
    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)>;
    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String>;
    fn next_phase_id(&mut self) -> Option<usize>;
    fn read_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String>;
    fn write_data(&self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String>;
    fn get_draw_data(&mut self) -> Box<&mut dyn Any>;
}

pub enum Constants {
    PlayerA = 0,
    PlayerB,
}

#[derive(Default, Clone, Copy)]
pub enum MatchMode {
    Computer,
    #[default]
    Offline,
    Casual,
    Ranked,
}

pub trait GameData {
    fn get_rng(&mut self) -> &mut Option<SmallRng>;
    fn set_rng(&mut self, rng: Option<SmallRng>);
    fn set_seed(&mut self, seed: u64) {
        self.set_rng(Some(SmallRng::seed_from_u64(seed)));
    }
    fn create_seed(&mut self) -> u64 {
        self.get_rng().as_mut().unwrap().next_u64()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum TwoPlayer {
    #[default]
    None = 0,
    First,
    Second,
}

impl TwoPlayer {
    pub fn next(&self) -> TwoPlayer {
        match self {
            TwoPlayer::First => TwoPlayer::Second,
            _ => TwoPlayer::First,
        }
    }
    pub fn get_index(&self) -> usize {
        match self {
            TwoPlayer::First => 0,
            TwoPlayer::Second => 1,
            _ => panic!(),
        }
    }
    pub fn exist(&self) -> bool {
        !matches!(self, TwoPlayer::None)
    }
}
