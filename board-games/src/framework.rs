pub mod input_util;
pub mod phases;
pub mod structs;

use crate::BoardGames;
use crate::framework::structs::common_draw_data::{CommonDrawData, CommonDrawTask};
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use structs::common_game_data::CommonGameData;

pub struct GameSystem {
    pub phase_id: usize,
    pub phases: Vec<Box<dyn Phase>>,
    pub game_data: Rc<RefCell<dyn Any>>,
    pub common_game_data: Rc<RefCell<CommonGameData>>,
}

impl GameSystem {
    pub fn get_phase(&mut self) -> Option<&mut Box<dyn Phase>> {
        self.phases
            .iter_mut()
            .find(|phase| phase.get_phase_id() == self.phase_id)
    }
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
    WaitWithMessage(String),
    NoWaitWithMessage(String),
}

pub trait Phase {
    fn get_phase_id(&self) -> usize;

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)>;

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String>;

    fn next_phase_id(&mut self) -> Option<usize>;

    fn read_data(&mut self, _: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        Ok(())
    }

    fn read_common_data(&mut self, _: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        Ok(())
    }

    fn write_data(&mut self, _: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        Ok(())
    }

    fn write_common_data(&mut self, _: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        Ok(())
    }

    fn has_draw_task(&mut self) -> bool {
        false
    }

    fn get_draw_data(&mut self) -> Box<&mut dyn Any> {
        unimplemented!()
    }

    fn get_common_draw_data(&mut self) -> &mut CommonDrawData;

    fn add_common_draw_task(&mut self, common_draw_task: CommonDrawTask) {
        self.get_common_draw_data().add_task(common_draw_task);
    }

    fn is_required_matching(&self) -> bool {
        false
    }

    fn set_is_player_a(&mut self, _: bool) {
        todo!()
    }

    fn dialog_answer_json(&mut self, _: &str) -> Result<(), String> {
        todo!()
    }

    fn next_game_id(&mut self) -> Option<BoardGames> {
        None
    }
}

pub enum Constants {
    PlayerA = 0,
    PlayerB,
}

pub trait GameData {
    fn get_rng(&mut self) -> &mut Option<SmallRng> {
        unimplemented!()
    }
    fn set_rng(&mut self, _: Option<SmallRng>) {
        unimplemented!()
    }

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
