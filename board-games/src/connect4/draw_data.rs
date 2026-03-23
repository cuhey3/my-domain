use crate::connect4::structs::board::Connect4Board;
use crate::framework::DrawData;

#[derive(Clone)]
pub enum Connect4DrawTask {
    Board(Connect4Board),
    PlayerInfo,
    Question(String),
    Message(String),
    GameResult(String),
    ErrorMessage(String),
    DebugMessage(String),
    EvaluateValue(String),
}
#[derive(Default, Clone)]
pub struct Connect4DrawData {
    tasks: Vec<Connect4DrawTask>,
}

impl DrawData for Connect4DrawData {
    fn has_task(&self) -> bool {
        !self.tasks.is_empty()
    }
}

impl Connect4DrawData {
    pub fn add_task(&mut self, task: Connect4DrawTask) {
        self.tasks.push(task)
    }
    pub fn take_task(&mut self) -> Option<Connect4DrawTask> {
        if self.tasks.is_empty() {
            None
        } else {
            Some(self.tasks.remove(0))
        }
    }
}
