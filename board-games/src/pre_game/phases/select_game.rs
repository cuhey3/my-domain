use crate::BoardGames;
use crate::framework::structs::common_draw_data::{CommonDrawData, CommonDrawTask};
use crate::framework::{AnswerType, Phase};
use crate::pre_game::phases::PreGamePhase;

#[derive(Default)]
pub struct SelectGamePhase {
    common_draw_data: CommonDrawData,
    next_game_id: Option<BoardGames>,
}

impl Phase for SelectGamePhase {
    fn get_phase_id(&self) -> usize {
        PreGamePhase::SelectGame as usize
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        if self.next_game_id.is_some() {
            return None;
        }
        self.add_common_draw_task(CommonDrawTask::Message(
            "ゲームを選択してください (1: コネクトフォー 2: 55将棋 3: BoP)".to_owned(),
        ));
        Some((AnswerType::Input, vec![]))
    }

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String> {
        let answer = answer.trim();
        if !matches!(answer, "1" | "2" | "3") {
            self.add_common_draw_task(CommonDrawTask::ErrorMessage(
                "1-3 を入力してください".into(),
            ));
            return Err(format!("Invalid answer: {}", answer));
        }
        let board_game = BoardGames::from_usize(answer.parse::<usize>().unwrap());
        self.add_common_draw_task(CommonDrawTask::Message(format!(
            "{board_game:?} を選択しました"
        )));
        self.next_game_id = Some(board_game);
        Ok(())
    }

    fn next_phase_id(&mut self) -> Option<usize> {
        None
    }

    fn get_common_draw_data(&mut self) -> &mut CommonDrawData {
        &mut self.common_draw_data
    }

    fn next_game_id(&mut self) -> Option<BoardGames> {
        self.next_game_id.clone()
    }
}
