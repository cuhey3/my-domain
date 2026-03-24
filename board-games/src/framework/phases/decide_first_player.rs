use crate::framework::phases::CommonPhase;
use crate::framework::structs::common_draw_data::{CommonDrawData, CommonDrawTask};
use crate::framework::{AnswerType, GameData, Phase, PhaseType};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::rc::Rc;
use crate::framework::structs::common_game_data::CommonGameData;

#[derive(Default)]
pub struct CommonDecideFirstPlayerPhase {
    state_position: usize,
    a_name: String,
    b_name: String,
    swap_flag: bool,
    common_draw_data: CommonDrawData,
    rng: Option<SmallRng>,
}

impl Phase for CommonDecideFirstPlayerPhase {
    fn get_phase_id(&self) -> usize {
        CommonPhase::DecideFirstPlayer as usize
    }

    fn phase_type(&self) -> Option<PhaseType> {
        Some(PhaseType::DecideFirstPlayer)
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        match self.state_position {
            0 => {
                self.add_common_draw_task(CommonDrawTask::Question("先手を決定します".into()));
                Some((AnswerType::Input, vec![]))
            }
            1 => {
                self.toss();
                let text = format!(
                    "抽選の結果: {} の先手",
                    if self.swap_flag {
                        &self.b_name
                    } else {
                        &self.a_name
                    }
                );
                self.add_common_draw_task(CommonDrawTask::Question(text));
                Some((AnswerType::Input, vec![]))
            }
            _ => None,
        }
    }

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String> {
        self.state_position += 1;
        Ok(())
    }

    fn next_phase_id(&mut self) -> Option<usize> {
        Some(CommonPhase::GameMain as usize)
    }

    fn read_common_data(&mut self, game_data: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        let mut game_data = game_data.borrow_mut();
        game_data.get_first_player().get_name();

        self.a_name = game_data.get_first_player().get_name();

        self.b_name = game_data.get_second_player().get_name();

        self.rng = Some(SmallRng::seed_from_u64(game_data.create_seed()));

        Ok(())
    }

    fn write_common_data(&mut self, game_data: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        if self.swap_flag {
            let mut game_data = game_data.borrow_mut();
            game_data.swap_player();
        }

        Ok(())
    }

    fn get_common_draw_data(&mut self) -> &mut CommonDrawData {
        &mut self.common_draw_data
    }
}

impl CommonDecideFirstPlayerPhase {
    fn toss(&mut self) {
        self.swap_flag = self.rng.as_mut().unwrap().random();
    }

    fn add_common_draw_task(&mut self, common_draw_task: CommonDrawTask) {
        self.common_draw_data.add_task(common_draw_task);
    }
}
