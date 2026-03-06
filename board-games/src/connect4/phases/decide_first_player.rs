use crate::connect4::Connect4Data;
use crate::connect4::draw_data::{Connect4DrawData, Connect4DrawTask};
use crate::connect4::phases::Connect4Phase;
use my_board_game::{AnswerType, GameData, Phase, PhaseType};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct DecideFirstPlayerPhase {
    state_position: usize,
    a_name: String,
    b_name: String,
    swap_flag: bool,
    draw_data: Connect4DrawData,
    rng: Option<SmallRng>,
}

impl Phase for DecideFirstPlayerPhase {
    fn get_phase_id(&self) -> usize {
        Connect4Phase::DecideFirstPlayer as usize
    }

    fn phase_type(&self) -> Option<PhaseType> {
        Some(PhaseType::DecideFirstPlayer)
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        match self.state_position {
            0 => {
                self.add_draw_task(Connect4DrawTask::Question("先手を決定します".into()));
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
                self.add_draw_task(Connect4DrawTask::Question(text));
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
        Some(Connect4Phase::GameMain as usize)
    }

    fn read_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if let Some(game_data) = game_data.borrow_mut().downcast_mut::<Connect4Data>() {
            game_data.get_first_player().get_name();
            self.a_name = game_data.get_first_player().get_name();
            self.b_name = game_data.get_second_player().get_name();
            self.rng = Some(SmallRng::seed_from_u64(game_data.create_seed()));
            Ok(())
        } else {
            Err("downcast error".into())
        }
    }

    fn write_data(&self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if let Some(game_data) = game_data.borrow_mut().downcast_mut::<Connect4Data>() {
            if self.swap_flag {
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
}

impl DecideFirstPlayerPhase {
    fn toss(&mut self) {
        self.swap_flag = self.rng.as_mut().unwrap().random();
    }
}

#[cfg(test)]
mod tests {
    use crate::connect4::phases::decide_first_player::DecideFirstPlayerPhase;
    use crate::connect4::structs::Connect4Data;
    use my_board_game::{GameData, Phase};
    use rand::rngs::SmallRng;
    use rand::{RngCore, SeedableRng};
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn get_phase() -> DecideFirstPlayerPhase {
        let mut phase = DecideFirstPlayerPhase::default();
        phase.rng = Some(SmallRng::seed_from_u64(0));
        phase
    }
    fn get_data() -> Rc<RefCell<dyn Any>> {
        let mut data = Connect4Data::default();
        data.set_seed(SmallRng::seed_from_u64(0).next_u64());
        Rc::new(RefCell::new(data))
    }

    // set up game data for test
    fn set_player_name(any_data: Rc<RefCell<dyn Any>>, player_a_name: &str, player_b_name: &str) {
        if let Some(game_data) = any_data.borrow_mut().downcast_mut::<Connect4Data>() {
            game_data.get_first_player().set_name(player_a_name);
            game_data.get_second_player().set_name(player_b_name);
        } else {
            panic!()
        }
    }
    #[test]
    fn test_swap() {
        let mut phase = get_phase();
        let any_data = get_data();
        let player_a_name = "player_a_name";
        let player_b_name = "player_b_name";

        set_player_name(any_data.clone(), player_a_name, player_b_name);

        // phase logic
        phase.read_data(&any_data).unwrap();
        phase.swap_flag = true;
        phase.write_data(&any_data).unwrap();

        // test phase result
        if let Some(game_data) = any_data.borrow_mut().downcast_mut::<Connect4Data>() {
            assert_eq!(game_data.get_first_player().get_name(), player_b_name);
            assert_eq!(game_data.get_second_player().get_name(), player_a_name);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_no_swap() {
        let mut phase = get_phase();
        let any_data = get_data();
        let player_a_name = "player_a_name";
        let player_b_name = "player_b_name";

        set_player_name(any_data.clone(), player_a_name, player_b_name);

        // phase logic
        phase.read_data(&any_data).unwrap();
        phase.write_data(&any_data).unwrap();

        // test phase result
        if let Some(game_data) = any_data.borrow_mut().downcast_mut::<Connect4Data>() {
            assert_eq!(game_data.get_first_player().get_name(), player_a_name);
            assert_eq!(game_data.get_second_player().get_name(), player_b_name);
        }
    }

    #[test]
    fn test_toss() {
        let mut phase = get_phase();
        phase.state_position = 1;

        // 初期は false
        assert!(!phase.swap_flag);

        // false の間 dialog_question()
        // 内部で toss() を呼びいずれ true になる
        while let false = phase.swap_flag {
            phase.dialog_question();
        }

        // true の間 dialog_question()
        // 内部で toss() を呼びいずれ false になる
        while let true = phase.swap_flag {
            phase.dialog_question();
        }

        assert!(!phase.swap_flag);
    }
}

impl DecideFirstPlayerPhase {
    fn add_draw_task(&mut self, connect4_draw_task: Connect4DrawTask) {
        self.draw_data.add_task(connect4_draw_task);
    }
}
