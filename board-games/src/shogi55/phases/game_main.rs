use crate::draw_data::Shogi55DrawTask;
use crate::shogi55::draw_data::Shogi55DrawData;
use crate::shogi55::phases::Shogi55Phase;
use crate::shogi55::structs::Shogi55Data;
use crate::shogi55::structs::board::{Shogi55Board, Shogi55Move, Shogi55Place};
use crate::shogi55::structs::piece::Piece;
use crate::shogi55::structs::simulate::Shogi55Simulate;
use my_board_game::{AnswerType, GameData, Phase, PhaseType, TwoPlayer};
use rand::prelude::SmallRng;
use rand::{RngCore, SeedableRng};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Default)]
pub struct GameMainPhase {
    is_first_player_turn: bool,
    board: Shogi55Board,
    first_player_name: String,
    second_player_name: String,
    move_input: MoveInput,
    rng: Option<SmallRng>,
    state: usize,
    draw_data: Shogi55DrawData,
    eval_value: i32,
}

#[derive(Default)]
struct MoveInput {
    in_hand_piece: Option<Piece>,
    from_square_index: Option<Shogi55Place>,
    to_square_index: Option<Shogi55Place>,
    promotion_flag: Option<bool>,
}

impl MoveInput {
    fn reset(&mut self) {
        *self = MoveInput::default();
    }

    fn input_answer_to(&mut self, answer: &str, board: &Shogi55Board) -> Result<(), String> {
        let parsed = answer
            .parse::<usize>()
            .map_err(|_| format!("入力が不正です: {answer}"))?;
        let place = MoveInput::parse_place(parsed)?;
        // 盤上の駒の移動チェック
        if let Some(from_index) = self.from_square_index {
            board.check_input_from_to(&from_index, &place)?;
        } else {
            board.check_input_in_hand(&place, &self.in_hand_piece.unwrap())?;
        }
        self.to_square_index = Some(place);
        Ok(())
    }

    fn input_answer_from(&mut self, answer: &str, board: &Shogi55Board) -> Result<(), String> {
        if let Ok(parsed) = answer.parse::<usize>() {
            let place = MoveInput::parse_place(parsed)?;
            board.check_input_from(&place)?;
            self.from_square_index = Some(place);
        } else if Piece::shogi55_in_hand_kanji_set().contains(&answer) {
            self.in_hand_piece = Some(Piece::from_str(answer)?);
        } else {
            return Err(format!("入力が不正です: {answer}"));
        }
        Ok(())
    }

    fn input_answer_promotion_flag(&mut self, answer: &str) -> Result<(), String> {
        let promotion_flag = if answer == "y" {
            true
        } else if answer == "n" {
            false
        } else {
            return Err("yかnで入力してください".into());
        };
        self.promotion_flag = Some(promotion_flag);
        Ok(())
    }

    fn parse_place(parsed: usize) -> Result<Shogi55Place, String> {
        let row_index = parsed % 10;
        if !(1..=5).contains(&row_index) {
            return Err(format!("入力が不正です: {parsed}"));
        }
        let col_index = parsed / 10;
        if !(1..=5).contains(&col_index) {
            return Err(format!("入力が不正です: {parsed}"));
        }
        Ok(Shogi55Place::new(col_index, row_index))
    }

    // fn parse_full_input(answer: &str, board: &mut Shogi55Board) -> Result<Shogi55Move, String> {
    //     let Some(col) = answer.chars().nth(0) else {
    //         return Err("入力が不正です".into())
    //     };
    //     let Ok(col_index) = col.to_string().parse::<usize>() else {
    //         return Err("入力が不正です".into())
    //     };
    //     let Some(row) = answer.chars().nth(1) else {
    //         return Err("入力が不正です".into())
    //     };
    //     let Ok(row_index) = row.to_string().parse::<usize>() else {
    //         return Err("入力が不正です".into())
    //     };
    //     let place = MoveInput::parse_place(col_index*10 + row_index)?;
    //     let Some(piece_char) = answer.chars().nth(2) else {
    //         return Err("入力が不正です".into())
    //     };
    //     let piece = Piece::from_str(&piece_char.to_string()) else {
    //         return Err("入力が不正です".into())
    //     };
    //     let mut promotion_flag = false;
    //     let mut in_hand_flag = false;
    //     if let Some(additional) = answer.chars().nth(3) {
    //         match additional {
    //             '成' => promotion_flag = true,
    //             '打' => in_hand_flag = true,
    //             _ => return Err("入力が不正です".into())
    //         }
    //     }
    //     let possible_moves = board.get_all_possible_moves();
    //     let possible_moves = possible_moves.iter().filter(|_move| {
    //        _move.is_match_for_input(&place, in_hand_flag, promotion_flag)
    //     }).collect::<Vec<_>>();
    //     if possible_moves.is_empty() {
    //         return Err("入力が不正です".into());
    //     }
    //     if possible_moves.len() == 1 {
    //         let possible_move = *possible_moves.first().unwrap().clone();
    //         if promotion_flag {
    //             let from = possible_move.get_from().unwrap();
    //             board.check_input_from(&from)?;
    //         }
    //     }
    // }
}

impl Phase for GameMainPhase {
    fn get_phase_id(&self) -> usize {
        Shogi55Phase::GameMain as usize
    }

    fn phase_type(&self) -> Option<PhaseType> {
        Some(PhaseType::GameMain)
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        let player_exp = if self.board.get_next_player() == TwoPlayer::First {
            "先手"
        } else {
            "後手"
        };
        match self.state {
            0 => {
                self.add_draw_task(Shogi55DrawTask::Board(self.board.clone()));
                self.add_draw_task(Shogi55DrawTask::EvaluateValue(self.eval_value));
                self.add_draw_task(Shogi55DrawTask::Question(format!(
                    "{player_exp}: どの駒を動かしますか(持ち駒の漢字、または数字二桁)"
                )));
                Some((AnswerType::Input, vec![]))
            }
            1 => {
                self.add_draw_task(Shogi55DrawTask::Question(format!(
                    "{player_exp}: どこへ指しますか(数字二桁)"
                )));
                Some((AnswerType::Input, vec![]))
            }
            2 => {
                self.add_draw_task(Shogi55DrawTask::Question(format!(
                    "{player_exp}: 駒を成りますか？(y/n)"
                )));
                Some((AnswerType::Input, vec![]))
            }
            3 => Some((AnswerType::Skip, vec![])),
            _ => panic!(),
        }
    }

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String> {
        let answer = answer.trim();
        if answer == "c" {
            self.move_input.reset();
            self.state = 0;
            return Ok(());
        }
        match self.state {
            0 => {
                self.move_input.input_answer_from(answer, &self.board)?;
                self.state = 1;
                return Ok(());
            }
            1 => {
                self.move_input.input_answer_to(answer, &self.board)?;
                if let Some(from) = &self.move_input.from_square_index {
                    let (is_required, promotion_flag) =
                        self.board.check_is_required_promotion_input(
                            from,
                            &self.move_input.to_square_index.unwrap(),
                        )?;
                    if is_required {
                        self.state = 2;
                        return Ok(());
                    }
                    self.move_input.promotion_flag = Some(promotion_flag);
                }
            }
            2 => {
                self.move_input.input_answer_promotion_flag(answer)?;
            }
            3 => {
                let mut simulate = Shogi55Simulate::get_simulate(
                    &self.board,
                    self.rng.as_mut().unwrap().next_u64(),
                );
                simulate.simulate();
                let (_move, point) = simulate.get_best_move_with_eval_value();
                self.eval_value = point;
                self.board.safe_move(_move)?;
                // if self.board.is_checkmated() {
                //     println!("詰みです")
                // };
                self.state = 0;
                return Ok(());
            }
            _ => panic!(),
        }
        let MoveInput {
            in_hand_piece,
            from_square_index,
            to_square_index,
            promotion_flag,
            ..
        } = self.move_input;
        self.board.safe_move(Shogi55Move::from_input(
            self.board.get_next_player(),
            in_hand_piece,
            from_square_index,
            to_square_index.unwrap(),
            promotion_flag.unwrap_or(false),
        ))?;
        self.move_input.reset();
        // if self.board.is_checkmated() {
        //     println!("詰みです")
        // };
        self.draw_data
            .add_task(Shogi55DrawTask::Board(self.board.clone()));
        self.draw_data
            .add_task(Shogi55DrawTask::Message("CPUが考えています...".into()));
        self.state = 3;
        Ok(())
    }

    fn next_phase_id(&mut self) -> Option<usize> {
        todo!()
    }

    fn read_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if let Some(data) = game_data.borrow_mut().downcast_mut::<Shogi55Data>() {
            self.rng = Some(SmallRng::seed_from_u64(data.create_seed()));
            self.draw_data = data.get_draw_data().clone();
            self.board.init();
            Ok(())
        } else {
            Err("downcast error".into())
        }
    }

    fn write_data(&self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        todo!()
    }

    fn get_draw_data(&mut self) -> Box<&mut dyn Any> {
        Box::new(&mut self.draw_data)
    }
}

impl GameMainPhase {
    fn add_draw_task(&mut self, shogi55draw_task: Shogi55DrawTask) {
        self.draw_data.add_task(shogi55draw_task);
    }
}
