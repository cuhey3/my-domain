use crate::framework::phases::CommonPhase;
use crate::framework::structs::common_draw_data::{CommonDrawData, CommonDrawTask};
use crate::framework::structs::common_game_data::CommonGameData;
use crate::framework::structs::match_setting::MatchSetting;
use crate::framework::{AnswerType, Phase};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct CommonSettingPhase {
    state_position: usize,
    match_setting: MatchSetting,
    common_draw_data: CommonDrawData,
}

impl Phase for CommonSettingPhase {
    fn get_phase_id(&self) -> usize {
        CommonPhase::Setting as usize
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        match self.state_position {
            0 => {
                self.add_common_draw_task(CommonDrawTask::Question(
                    "オンライン対戦しますか？(y/n)".into(),
                ));

                Some((AnswerType::Input, vec![]))
            }
            1 => {
                self.add_common_draw_task(CommonDrawTask::Question(
                    "CPUと対戦しますか？(y/n)".into(),
                ));

                Some((AnswerType::Input, vec![]))
            }
            2 => {
                self.add_common_draw_task(CommonDrawTask::Question(
                    "待ったをありにしますか？(y/n)".into(),
                ));

                Some((AnswerType::Input, vec![]))
            }
            3 => {
                self.add_common_draw_task(CommonDrawTask::Question(
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
                self.match_setting.set_online_mode(flag);

                self.state_position += 1;

                if flag {
                    self.state_position += 1;
                }

                Ok(())
            }
            1 => {
                self.match_setting.set_cpu_mode(flag);

                self.state_position += 1;

                Ok(())
            }
            2 => {
                self.match_setting.set_enable_do_over(flag);

                self.state_position += 1;

                Ok(())
            }
            3 => {
                self.match_setting.set_with_eval_value(flag);

                self.state_position += 1;

                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn next_phase_id(&mut self) -> Option<usize> {
        Some(CommonPhase::Entry as usize)
    }

    fn read_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        Ok(())
    }

    fn write_common_data(&mut self, game_data: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        let mut game_data = game_data.borrow_mut();
        game_data.set_setting(self.match_setting);

        game_data.set_default_cpu_player_index_if_necessary();

        game_data.set_default_online_player_index_if_necessary();

        Ok(())
    }

    fn get_common_draw_data(&mut self) -> &mut CommonDrawData {
        &mut self.common_draw_data
    }
}
