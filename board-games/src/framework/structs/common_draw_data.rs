use crate::framework::DrawData;

#[derive(Default, Clone)]
pub struct CommonDrawData {
    tasks: Vec<CommonDrawTask>,
}

#[derive(Clone)]
pub enum CommonDrawTask {
    PlayerInfo,
    Question(String),
    Message(String),
    GameResult(String),
    ErrorMessage(String),
    DebugMessage(String),
    EvaluateValue(String),
}

impl DrawData for CommonDrawData {
    fn has_task(&self) -> bool {
        !self.tasks.is_empty()
    }
}

impl CommonDrawData {
    pub fn add_task(&mut self, task: CommonDrawTask) {
        self.tasks.push(task)
    }
    pub fn take_task(&mut self) -> Option<CommonDrawTask> {
        if self.tasks.is_empty() {
            None
        } else {
            Some(self.tasks.remove(0))
        }
    }
}
