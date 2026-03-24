use crate::connect4::phases::Connect4Phase;
use crate::connect4::structs::board::Connect4Board;
use crate::connect4::structs::simulate::Connect4Simulate;

use crate::connect4::draw_data::{Connect4DrawData, Connect4DrawTask};
use crate::connect4::structs::search_checkmate::SearchCheckmate;
use crate::framework::structs::common_draw_data::CommonDrawData;
use crate::framework::structs::match_setting::MatchSetting;
use crate::framework::{
    AnswerType, DrawData, GameData, Phase, PhaseType, TwoPlayer,
};
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::framework::structs::common_game_data::CommonGameData;

#[derive(Default)]
pub struct GameMainPhase {
    connect4_setting: MatchSetting,
    board: Connect4Board,
    first_player_name: String,
    first_player_cpu_flag: bool,
    first_player_online_flag: bool,
    second_player_name: String,
    second_player_cpu_flag: bool,
    second_player_online_flag: bool,
    draw_data: Connect4DrawData,
    common_draw_data: CommonDrawData,
    rng: Option<SmallRng>,
}

impl Phase for GameMainPhase {
    fn get_phase_id(&self) -> usize {
        Connect4Phase::GameMain as usize
    }

    fn phase_type(&self) -> Option<PhaseType> {
        Some(PhaseType::GameMain)
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        let enable_do_over = self.connect4_setting.get_enable_do_over();

        if self.board.winner().exist() {
            return if enable_do_over {
                self.add_draw_task(Connect4DrawTask::Question(
                    "待ったしますか？(cを入力)".into(),
                ));
                Some((AnswerType::Input, vec![]))
            } else {
                None
            };
        }

        let additional_dialog = if enable_do_over {
            " 待った(cを入力)"
        } else {
            ""
        };

        self.add_draw_task(Connect4DrawTask::Board(self.board.clone()));

        if self.connect4_setting.get_with_eval_value() {
            let mut simulate = self.get_simulate();

            self.add_draw_task(Connect4DrawTask::EvaluateValue(simulate.show_result()));
        }

        if self.board.is_first_player_turn() {
            if self.first_player_cpu_flag {
                Some((AnswerType::Skip, vec![]))
            } else if self.first_player_online_flag {
                self.add_draw_task(Connect4DrawTask::Question(
                    "入力を待っています...".to_owned(),
                ));

                Some((AnswerType::Wait, vec![]))
            } else {
                self.add_draw_task(Connect4DrawTask::Question(format!(
                    "先手: {}さんの番です(1-7を入力){}",
                    self.first_player_name, additional_dialog
                )));

                Some((AnswerType::Input, vec![]))
            }
        } else {
            if self.second_player_cpu_flag {
                Some((AnswerType::Skip, vec![]))
            } else if self.second_player_online_flag {
                self.add_draw_task(Connect4DrawTask::Question(
                    "入力を待っています...".to_owned(),
                ));

                Some((AnswerType::Wait, vec![]))
            } else {
                self.add_draw_task(Connect4DrawTask::Question(format!(
                    "後手: {}さんの番です(1-7を入力){}",
                    self.second_player_name, additional_dialog
                )));

                Some((AnswerType::Input, vec![]))
            }
        }
    }

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String> {
        let answer = answer.trim();

        if self.board.is_first_player_turn() {
            if self.first_player_cpu_flag {
                self.do_cpu_move().expect("CPU操作が失敗しました");

                return Ok(());
            }
        } else if self.second_player_cpu_flag {
            self.do_cpu_move().expect("CPU操作が失敗しました");

            return Ok(());
        }

        let enable_do_over = self.connect4_setting.get_enable_do_over();

        // TODO
        // ゲーム完了時に待ったしないで処理を抜ける方法がない
        if answer == "c" && enable_do_over {
            println!("待ったしました");

            Ok(self.board.reject_last_2_move()?)
        } else {
            self.do_player_move(answer)
        }
    }

    fn next_phase_id(&mut self) -> Option<usize> {
        None
    }

    fn read_common_data(&mut self, game_data: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        let mut data = game_data.borrow_mut();
        self.first_player_name = data.get_first_player().get_name().clone();

        self.second_player_name = data.get_second_player().get_name().clone();

        self.first_player_cpu_flag = data.first_player_is_cpu();

        self.second_player_cpu_flag = data.second_player_is_cpu();

        self.first_player_online_flag = data.first_player_is_online();

        self.second_player_online_flag = data.second_player_is_online();

        self.connect4_setting = *data.get_setting();

        self.rng = Some(SmallRng::seed_from_u64(data.create_seed()));

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
impl GameMainPhase {
    fn get_simulate(&mut self) -> Connect4Simulate {
        let nest_count = match self.board.stone_count {
            count if count < 4 => 3,
            count if count < 8 => 5,
            count if count < 16 => 7,
            count if count < 24 => 9,
            _ => 11,
        };

        let mut search_checkmate = SearchCheckmate::new(self.board.clone(), nest_count);

        let result = search_checkmate.search();

        let mut simulate =
            Connect4Simulate::new(&self.board, self.rng.as_mut().unwrap().next_u64(), result);

        simulate.simulate();

        simulate
    }
    fn do_cpu_move(&mut self) -> Result<(), String> {
        let simulate = self.get_simulate();

        let Some(best_move) = simulate.get_best_move_with_checkmate_search() else {
            todo!()
        };

        self.board.safe_move(best_move)?;

        self.do_move_end();

        Ok(())
    }

    fn do_player_move(&mut self, answer: &str) -> Result<(), String> {
        let input_index: usize = answer
            .parse()
            .map_err(|_| format!("入力が不正です: {}", answer))?;

        self.board.safe_move(input_index)?;

        self.do_move_end();

        Ok(())
    }

    fn do_move_end(&mut self) {
        let winner = self.board.winner();

        if winner.exist() {
            self.add_draw_task(Connect4DrawTask::Board(self.board.clone()));

            self.add_draw_task(Connect4DrawTask::GameResult(format!(
                "{}の勝ちです",
                if winner == TwoPlayer::First {
                    &self.first_player_name
                } else {
                    &self.second_player_name
                }
            )))
        }
    }

    fn add_draw_task(&mut self, connect4_draw_task: Connect4DrawTask) {
        self.draw_data.add_task(connect4_draw_task);
    }
}
