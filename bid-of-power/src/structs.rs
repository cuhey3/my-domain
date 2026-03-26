pub mod board;
pub mod input;
mod items;

use crate::structs::board::PlayerInfo;
use crate::structs::input::BidInput;
use crate::structs::items::Item;
use board_games::GameData;
use board_games::framework::{DrawData, Drawer};
use rand::rngs::SmallRng;
use std::any::Any;

#[derive(Default)]
pub struct BoPGameData {
    draw_data: BoPDrawData,
    rng: Option<SmallRng>,
}

impl GameData for BoPGameData {
    fn get_rng(&mut self) -> &mut Option<SmallRng> {
        &mut self.rng
    }

    fn set_rng(&mut self, rng: Option<SmallRng>) {
        self.rng = rng;
    }
}

#[derive(Default)]
pub struct BoPDrawData {
    tasks: Vec<BoPDrawTask>,
}

impl DrawData for BoPDrawData {
    fn has_task(&self) -> bool {
        !self.tasks.is_empty()
    }
}

impl BoPDrawData {
    pub fn add_task(&mut self, task: BoPDrawTask) {
        self.tasks.push(task)
    }
    pub fn take_task(&mut self) -> Option<BoPDrawTask> {
        if self.tasks.is_empty() {
            None
        } else {
            Some(self.tasks.remove(0))
        }
    }
}

pub enum BoPDrawTask {
    Message(String),
    Question(String),
    ListItems(Vec<Item>),
    CurrentBids(Vec<BidInput>),
    PlayerInfo(PlayerInfo, PlayerInfo),
}

#[derive(Default)]
pub struct BoPDrawer {}

impl Drawer for BoPDrawer {
    fn draw(&mut self, draw_data: Box<&mut dyn Any>) {
        let draw_data = draw_data.downcast_mut::<BoPDrawData>().unwrap();

        while let Some(task) = draw_data.take_task() {
            match task {
                BoPDrawTask::Question(message) => println!("{}", message),
                BoPDrawTask::Message(message) => println!("{}", message),
                BoPDrawTask::ListItems(items) => {
                    for item in items {
                        println!("{:?}", item);
                    }
                }
                BoPDrawTask::CurrentBids(items) => {
                    for item in items {
                        println!("{:?}", item);
                    }
                }
                BoPDrawTask::PlayerInfo(info1, info2) => {
                    println!("{:?}", info1);
                    println!("{:?}", info2);
                }
            }
        }
    }

    fn draw_error(&mut self, error: String) {
        println!("draw error: {}", error);
    }
}
