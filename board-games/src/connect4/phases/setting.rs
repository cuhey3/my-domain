use crate::connect4::draw_data::{Connect4DrawData, Connect4DrawTask};
use crate::connect4::phases::Connect4Phase;
use crate::connect4::structs::{Connect4Data, Connect4Setting};
use my_board_game::{AnswerType, Phase, PhaseType};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct SettingPhase {
    state_position: usize,
    connect4_setting: Connect4Setting,
    draw_data: Connect4DrawData,
}

impl Phase for SettingPhase {
    fn get_phase_id(&self) -> usize {
        Connect4Phase::Setting as usize
    }

    fn phase_type(&self) -> Option<PhaseType> {
        Some(PhaseType::Setting)
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        match self.state_position {
            0 => {
                self.add_draw_task(Connect4DrawTask::Question(
                    "CPUと対戦しますか？(y/n)".into(),
                ));
                Some((AnswerType::Input, vec![]))
            }
            1 => {
                self.add_draw_task(Connect4DrawTask::Question(
                    "待ったをありにしますか？(y/n)".into(),
                ));
                Some((AnswerType::Input, vec![]))
            }
            2 => {
                self.add_draw_task(Connect4DrawTask::Question(
                    "評価値を表示しますか？(y/n)".into(),
                ));
                Some((AnswerType::Input, vec![]))
            }
            _ => None,
        }
    }

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String> {
        let answer = answer.trim();
        let flag = match answer {
            "y" => true,
            "n" => false,
            _ => return Err("y か n で入力してください".to_owned()),
        };
        match self.state_position {
            0 => {
                self.connect4_setting.set_cpu_mode(flag);
                self.state_position += 1;
                Ok(())
            }
            1 => {
                self.connect4_setting.set_enable_do_over(flag);
                self.state_position += 1;
                Ok(())
            }
            2 => {
                self.connect4_setting.set_with_eval_value(flag);
                self.state_position += 1;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn next_phase_id(&mut self) -> Option<usize> {
        Some(Connect4Phase::Entry as usize)
    }

    fn read_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if let Some(game_data) = game_data.borrow_mut().downcast_mut::<Connect4Data>() {
            Ok(())
        } else {
            Err("downcast error".into())
        }
    }

    fn write_data(&self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if let Some(game_data) = game_data.borrow_mut().downcast_mut::<Connect4Data>() {
            game_data.set_setting(self.connect4_setting);
            game_data.set_default_cpu_player_index_if_necessary();
            Ok(())
        } else {
            Err("downcast error".into())
        }
    }

    fn get_draw_data(&mut self) -> Box<&mut dyn Any> {
        Box::new(&mut self.draw_data)
    }
}

impl SettingPhase {
    fn add_draw_task(&mut self, connect4_draw_task: Connect4DrawTask) {
        self.draw_data.add_task(connect4_draw_task);
    }
}
