use crate::structs::board::BoPBoard;
use crate::structs::input::{BidInput, BoPInput, DisplayItemInput};
use crate::structs::{BoPDrawData, BoPDrawTask};
use board_games::framework::{AnswerType, Phase, PhaseType, TwoPlayer};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GameMain {
    draw_data: BoPDrawData,
    current_input: BoPInput,
    board: BoPBoard,
    inputs: Vec<BoPInput>,
}

impl Phase for GameMain {
    fn get_phase_id(&self) -> usize {
        0
    }

    fn phase_type(&self) -> Option<PhaseType> {
        todo!()
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        if self.current_input.has_confirm() {
            self.draw_data
                .add_task(BoPDrawTask::Question("よろしいですか？(y/n)".into()));
            return Some((AnswerType::Input, vec![]));
        }
        match self.current_input {
            BoPInput::Bid(_) => {
                self.draw_data.add_task(BoPDrawTask::Question(format!(
                    "{:?}さん、入札するアイテムを選んでください(No,価格)",
                    self.current_input.get_player()
                )));
                Some((AnswerType::Input, vec![]))
            }
            BoPInput::ItemUse(_) => {
                self.draw_data.add_task(BoPDrawTask::Question(format!(
                    "{:?}さん、使用するアイテムを選択してください(No/n)",
                    self.current_input.get_player()
                )));
                Some((AnswerType::Input, vec![]))
            }
            BoPInput::BattleChoose(_) => {
                self.draw_data.add_task(BoPDrawTask::Question(format!(
                    "{:?}さん、プレイヤー1さん、攻撃しますか？攻撃しないと+1Gold得られます(y/n)",
                    self.current_input.get_player()
                )));
                Some((AnswerType::Input, vec![]))
            }
            _ => panic!(),
        }
    }

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String> {
        if self.current_input.has_confirm() {
            self.current_input.confirm(&answer);
        } else {
            self.current_input.answer_to_input(&answer)?;
        }
        // confirm より先にバリデーション
        let result = self.board.validate_player_input(&self.current_input);
        if result.is_err() {
            self.current_input = self.board.get_next_input();
            return result;
        }

        if self.current_input.has_confirm() {
            return Ok(());
        }

        // ここまでで BoPInput が成立
        self.board.add_input(&self.current_input);
        self.inputs.push(self.current_input.clone());
        if self.board.has_winner() {
            // TODO
            // 終端処理
        } else {
            self.current_input = self.board.get_next_input();
        }
        Ok(())
    }

    fn next_phase_id(&mut self) -> Option<usize> {
        todo!()
    }

    fn read_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        self.board.init();
        self.current_input = self.board.get_next_input();
        Ok(())
    }

    fn write_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        Ok(())
    }

    fn get_draw_data(&mut self) -> Box<&mut dyn Any> {
        Box::new(&mut self.draw_data)
    }
}
