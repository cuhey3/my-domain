use crate::shogi55::structs::board::Shogi55Board;
use my_board_game::DrawData;

#[derive(Default, Clone)]
pub struct Shogi55DrawData {
    tasks: Vec<Shogi55DrawTask>,
}

#[derive(Clone)]
pub enum Shogi55DrawTask {
    Board(Shogi55Board),
    InHand(Shogi55Board),
    PlayerInfo,
    Question(String),
    Message(String),
    ErrorMessage(String),
    DebugMessage(String),
    EvaluateValue(i32),
}

impl DrawData for Shogi55DrawData {
    fn has_task(&self) -> bool {
        !self.tasks.is_empty()
    }
}

impl Shogi55DrawData {
    pub fn add_task(&mut self, task: Shogi55DrawTask) {
        self.tasks.push(task)
    }
    pub fn take_task(&mut self) -> Option<Shogi55DrawTask> {
        if self.tasks.is_empty() {
            None
        } else {
            Some(self.tasks.remove(0))
        }
    }
}
