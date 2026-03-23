use crate::connect4::Connect4Data;
use crate::connect4::draw_data::{Connect4DrawData, Connect4DrawTask};
use crate::connect4::phases::Connect4Phase;
use crate::framework::{AnswerType, GameData, Phase, PhaseType};
use board_game_if::structs::decide_first_player::DecideFirstPlayerSequence;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct OnlineDecideFirstPlayerPhase {
    state_position: usize,
    a_name: String,
    b_name: String,
    own_user_id: u64,
    draw_data: Connect4DrawData,
    is_player_a: bool,
    rng: Option<SmallRng>,
    decide_first_player_sequence: DecideFirstPlayerSequence,
}

impl Phase for OnlineDecideFirstPlayerPhase {
    fn get_phase_id(&self) -> usize {
        Connect4Phase::OnlineDecideFirstPlayer as usize
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
        Some(Connect4Phase::GameMain as usize)
    }

    fn read_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if let Some(game_data) = game_data.borrow_mut().downcast_mut::<Connect4Data>() {
            game_data.get_first_player().get_name();

            self.a_name = game_data.get_first_player().get_name();

            self.b_name = game_data.get_second_player().get_name();

            self.own_user_id = game_data.get_first_player().get_id();

            self.rng = Some(SmallRng::seed_from_u64(game_data.create_seed()));

            let own_seed = self.rng.as_mut().unwrap().next_u64();

            self.decide_first_player_sequence =
                DecideFirstPlayerSequence::new(self.own_user_id, own_seed, self.is_player_a);

            Ok(())
        } else {
            Err("downcast error".into())
        }
    }

    fn write_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if self.decide_first_player_sequence.is_complete() {
            let expression = if self.decide_first_player_sequence.is_swap_required()? {
                "後手"
            } else {
                "先手"
            };

            self.add_draw_task(Connect4DrawTask::Question(format!(
                "抽選の結果、あなたは {} です",
                expression
            )));
        }
        if let Some(game_data) = game_data.borrow_mut().downcast_mut::<Connect4Data>() {
            if self.decide_first_player_sequence.is_swap_required()? {
                game_data.swap_player();
            }

            Ok(())
        } else {
            Err("downcast error".into())
        }
    }

    fn get_draw_data(&mut self) -> Box<&mut dyn Any> {
        Box::new(&mut self.draw_data)
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

impl OnlineDecideFirstPlayerPhase {
    fn add_draw_task(&mut self, connect4_draw_task: Connect4DrawTask) {
        self.draw_data.add_task(connect4_draw_task);
    }
}
