use crate::framework::phases::CommonPhase;
use crate::framework::structs::common_draw_data::{CommonDrawData, CommonDrawTask};
use crate::framework::{AnswerType, GameData, Phase, PhaseType};
use board_game_if::structs::decide_first_player::DecideFirstPlayerSequence;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::cell::RefCell;
use std::rc::Rc;
use crate::framework::structs::common_game_data::CommonGameData;

#[derive(Default)]
pub struct CommonOnlineDecideFirstPlayerPhase {
    state_position: usize,
    a_name: String,
    b_name: String,
    own_user_id: u64,
    common_draw_data: CommonDrawData,
    is_player_a: bool,
    rng: Option<SmallRng>,
    decide_first_player_sequence: DecideFirstPlayerSequence,
}

impl Phase for CommonOnlineDecideFirstPlayerPhase {
    fn get_phase_id(&self) -> usize {
        CommonPhase::OnlineDecideFirstPlayer as usize
    }

    fn phase_type(&self) -> Option<PhaseType> {
        Some(PhaseType::DecideFirstPlayer)
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        let json = self
            .decide_first_player_sequence
            .get_question_json()
            .map_err(|err| format!("getting question failed: {}", err))
            .unwrap();

        if json.is_empty() {
            if self.decide_first_player_sequence.is_complete() {
                None
            } else {
                Some((AnswerType::Wait, vec![]))
            }
        } else {
            if self.decide_first_player_sequence.is_complete() {
                Some((AnswerType::NoWaitWithMessage(json), vec![]))
            } else {
                Some((AnswerType::WaitWithMessage(json), vec![]))
            }
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

        self.own_user_id = game_data.get_first_player().get_id();

        self.rng = Some(SmallRng::seed_from_u64(game_data.create_seed()));

        let own_seed = self.rng.as_mut().unwrap().next_u64();

        self.decide_first_player_sequence =
            DecideFirstPlayerSequence::new(self.own_user_id, own_seed, self.is_player_a);

        Ok(())
    }

    fn write_common_data(&mut self, game_data: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        if self.decide_first_player_sequence.is_complete() {
            let expression = if self.decide_first_player_sequence.is_swap_required()? {
                "後手"
            } else {
                "先手"
            };

            self.add_common_draw_task(CommonDrawTask::Question(format!(
                "抽選の結果、あなたは {} です",
                expression
            )));
        }
        if self.decide_first_player_sequence.is_swap_required()? {
            let mut game_data = game_data.borrow_mut();
            game_data.swap_player();
        }

        Ok(())
    }

    fn get_common_draw_data(&mut self) -> &mut CommonDrawData {
        &mut self.common_draw_data
    }

    fn is_required_matching(&self) -> bool {
        true
    }

    fn set_is_player_a(&mut self, is_player_a: bool) {
        self.is_player_a = is_player_a;
    }

    fn dialog_answer_json(&mut self, json: &str) -> Result<(), String> {
        self.decide_first_player_sequence.set_answer_json(json)?;

        Ok(())
    }
}

impl CommonOnlineDecideFirstPlayerPhase {
    fn add_common_draw_task(&mut self, common_draw_task: CommonDrawTask) {
        self.common_draw_data.add_task(common_draw_task);
    }
}
