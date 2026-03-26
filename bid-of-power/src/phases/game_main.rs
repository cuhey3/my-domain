use crate::phases::BoPPhase;
use crate::structs::board::BoPBoard;
use crate::structs::input::BoPInput;
use crate::structs::{BoPDrawData, BoPDrawTask};
use board_games::GameData;
use board_games::framework::structs::common_draw_data::CommonDrawData;
use board_games::framework::structs::common_game_data::CommonGameData;
use board_games::framework::structs::match_setting::MatchSetting;
use board_games::framework::{AnswerType, DrawData, Phase};
use rand::SeedableRng;
use rand::prelude::SmallRng;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GameMain {
    draw_data: BoPDrawData,
    match_setting: MatchSetting,
    first_player_name: String,
    first_player_cpu_flag: bool,
    first_player_online_flag: bool,
    second_player_name: String,
    second_player_cpu_flag: bool,
    second_player_online_flag: bool,
    common_draw_data: CommonDrawData,
    current_input: BoPInput,
    board: BoPBoard,
    inputs: Vec<BoPInput>,
    rng: Option<SmallRng>,
}

impl Phase for GameMain {
    fn get_phase_id(&self) -> usize {
        BoPPhase::GameMain as usize
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        if self.board.is_first_player_turn() {
            if self.first_player_cpu_flag {
                return Some((AnswerType::Skip, vec![]));
            } else if self.first_player_online_flag {
                self.add_draw_task(BoPDrawTask::Question("入力を待っています...".to_owned()));

                return Some((AnswerType::Wait, vec![]));
            }
        } else {
            if self.second_player_cpu_flag {
                return Some((AnswerType::Skip, vec![]));
            } else if self.second_player_online_flag {
                self.add_draw_task(BoPDrawTask::Question("入力を待っています...".to_owned()));

                return Some((AnswerType::Wait, vec![]));
            }
        }

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
                    "{:?}さん、攻撃しますか？攻撃しないと+1Gold得られます(y/n)",
                    self.current_input.get_player()
                )));

                Some((AnswerType::Input, vec![]))
            }
            _ => panic!(),
        }
    }

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String> {
        let answer = answer.trim().to_owned();

        if self.current_input.has_confirm() {
            if !self.current_input.confirm(&answer) {
                self.current_input.player_keeping_reset();

                return Ok(());
            };
        } else {
            self.current_input.answer_to_input(&answer)?;
        }

        // confirm より先にバリデーション
        let result = self.board.validate_player_input(&self.current_input);

        if result.is_err() {
            self.current_input.player_keeping_reset();

            return result;
        }

        if self.current_input.has_confirm() {
            return Ok(());
        }

        // ここまでで BoPInput が成立
        self.board.add_input(&self.current_input)?;

        if matches!(
            self.current_input,
            BoPInput::Bid(_) | BoPInput::ItemUse(_) | BoPInput::BattleChoose(_)
        ) {
            self.draw_data
                .add_task(BoPDrawTask::ListItems(self.board.get_list_items()));

            self.draw_data
                .add_task(BoPDrawTask::CurrentBids(self.board.get_current_bids()));

            let (info1, info2) = self.board.get_player_info();

            self.draw_data
                .add_task(BoPDrawTask::PlayerInfo(info1, info2))
        }

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

        self.draw_data
            .add_task(BoPDrawTask::ListItems(self.board.get_list_items()));

        Ok(())
    }

    fn read_common_data(&mut self, game_data: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        let mut data = game_data.borrow_mut();

        self.first_player_name = data.get_first_player().get_name().clone();

        self.second_player_name = data.get_second_player().get_name().clone();

        self.first_player_cpu_flag = data.first_player_is_cpu();

        self.second_player_cpu_flag = data.second_player_is_cpu();

        self.first_player_online_flag = data.first_player_is_online();

        self.second_player_online_flag = data.second_player_is_online();

        self.match_setting = *data.get_setting();

        self.rng = Some(SmallRng::seed_from_u64(data.create_seed()));

        Ok(())
    }

    fn write_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        Ok(())
    }

    fn has_draw_task(&mut self) -> bool {
        self.draw_data.has_task()
    }

    fn get_draw_data(&mut self) -> Box<&mut dyn Any> {
        Box::new(&mut self.draw_data)
    }

    fn get_common_draw_data(&mut self) -> &mut CommonDrawData {
        &mut self.common_draw_data
    }

    fn dialog_answer_json(&mut self, json: &str) -> Result<(), String> {
        self.dialog_answer(json.to_owned(), vec![])
    }
}

impl GameMain {
    fn add_draw_task(&mut self, draw_task: BoPDrawTask) {
        self.draw_data.add_task(draw_task);
    }
}
