use crate::connect4::phases::Connect4Phase;
use crate::connect4::structs::{Connect4Data, Connect4Player};
use my_board_game::{Constants, DrawTask, Phase, PhaseType};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct EntryPhase {
    state_position: usize,
    connect4_player_a: Option<PlayerInput>,
    connect4_player_b: Option<PlayerInput>,
    has_cpu: bool,
}

impl Phase for EntryPhase {
    fn get_phase_id(&self) -> usize {
        Connect4Phase::Entry as usize
    }

    fn phase_type(&self) -> Option<PhaseType> {
        Some(PhaseType::Entry)
    }

    fn dialog_question(&mut self) -> Option<(String, Vec<isize>)> {
        match self.state_position {
            0 => Some((
                "一人目の名前を入力してください".into(),
                vec![Constants::PlayerA as isize],
            )),
            1 => Some((
                "一人目のidを入力してください".into(),
                vec![Constants::PlayerA as isize],
            )),
            2 => Some((
                "二人目の名前を入力してください".into(),
                vec![Constants::PlayerB as isize],
            )),
            3 => Some((
                "二人目のidを入力してください".into(),
                vec![Constants::PlayerB as isize],
            )),
            _ => None,
        }
    }

    fn dialog_answer(&mut self, answer: String, args: Vec<isize>) -> Result<(), String> {
        let answer = answer.trim();
        match self.state_position {
            0 => {
                self.entry(&args)?;
                self.set_name(answer, &args)?;
                self.state_position += 1;
                Ok(())
            }
            1 => {
                let id: u64 = answer.parse().map_err(|_| "parse error".to_owned())?;
                self.set_player_id(id, &args)?;
                self.state_position += 1;
                if self.has_cpu {
                    self.connect4_player_b = Some(PlayerInput {
                        name: Some("CPU".into()),
                        id: Some(0),
                    });
                    self.state_position += 3;
                }
                Ok(())
            }
            2 => {
                self.entry(&args)?;
                self.set_name(answer, &args)?;
                self.state_position += 1;
                Ok(())
            }
            3 => {
                let id: u64 = answer.parse().map_err(|_| "parse error".to_owned())?;
                self.set_player_id(id, &args)?;
                self.state_position += 1;
                Ok(())
            }
            // 4 => {
            //     if answer == "y" {
            //         self.has_cpu = true;
            //     } else if answer == "n" {
            //         self.has_cpu = false;
            //     } else {
            //         return Err("y か n で入力してください".to_owned());
            //     }
            //     self.state_position += 1;
            //     Ok(())
            // }
            _ => Ok(()),
        }
    }

    fn next_phase_id(&mut self) -> Option<usize> {
        Some(Connect4Phase::DecideFirstPlayer as usize)
    }

    fn read_data(&mut self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if let Some(game_data) = game_data.borrow_mut().downcast_mut::<Connect4Data>() {
            self.has_cpu = game_data.has_cpu();
            Ok(())
        } else {
            Err("downcast error".into())
        }
    }

    fn write_data(&self, game_data: &Rc<RefCell<dyn Any>>) -> Result<(), String> {
        if let Some(game_data) = game_data.borrow_mut().downcast_mut::<Connect4Data>() {
            let Some(connect4_player) = &self.connect4_player_a else {
                return Err("connect4_player_a is none.".to_owned());
            };
            let player = connect4_player.create_connect4_player()?;
            game_data.set_first_player(player);
            let Some(connect4_player) = &self.connect4_player_b else {
                return Err("connect4_player_b is none.".to_owned());
            };
            let player = connect4_player.create_connect4_player()?;
            game_data.set_second_player(player);
            Ok(())
        } else {
            Err("downcast error".into())
        }
    }

    fn get_draw_tasks(&mut self) -> Vec<Box<dyn DrawTask>> {
        todo!()
    }
}

impl EntryPhase {
    fn entry(&mut self, args: &[isize]) -> Result<(), String> {
        match args.first() {
            None => Err("Arguments require input of player A/B.".to_owned()),
            Some(i) if *i == Constants::PlayerA as isize => {
                if self.connect4_player_a.is_none() {
                    self.connect4_player_a = Some(PlayerInput::default());
                    Ok(())
                } else {
                    Err("Player A already entered.".to_owned())
                }
            }
            Some(i) if *i == Constants::PlayerB as isize => {
                if self.connect4_player_b.is_none() {
                    self.connect4_player_b = Some(PlayerInput::default());
                    Ok(())
                } else {
                    Err("Player B already entered.".to_owned())
                }
            }
            _ => Err("The argument does not indicate player A/B.".to_owned()),
        }
    }

    fn check_player(&self, _: &Vec<isize>) -> Result<(), String> {
        if let Some(player_a) = self.connect4_player_a.as_ref() {
            player_a.check_fulfilled()?;
        } else {
            return Err("Player A is not entered.".to_owned());
        }
        if let Some(player_b) = self.connect4_player_b.as_ref() {
            player_b.check_fulfilled()?;
        } else {
            return Err("Player B is not entered.".to_owned());
        }
        Ok(())
    }
    fn set_name(&mut self, name: &str, args: &[isize]) -> Result<(), String> {
        match args.first() {
            None => Err("Arguments require input of player A/B.".to_owned()),
            Some(i) if *i == Constants::PlayerA as isize => match self.connect4_player_a.as_mut() {
                None => Err("Player A is not entered.".to_owned()),
                Some(player_a) => {
                    player_a.set_name(name.to_owned());
                    Ok(())
                }
            },
            Some(i) if *i == Constants::PlayerB as isize => match self.connect4_player_b.as_mut() {
                None => Err("Player B is not entered.".to_owned()),
                Some(player_b) => {
                    player_b.set_name(name.to_owned());
                    Ok(())
                }
            },
            _ => Err("The argument does not indicate player A/B.".to_owned()),
        }
    }

    fn set_player_id(&mut self, id: u64, args: &[isize]) -> Result<(), String> {
        match args.first() {
            None => Err("Arguments require input of player A/B.".to_owned()),
            Some(i) if *i == Constants::PlayerA as isize => match self.connect4_player_a.as_mut() {
                None => Err("Player A is not entered.".to_owned()),
                Some(player_a) => {
                    player_a.set_id(id);
                    Ok(())
                }
            },
            Some(i) if *i == Constants::PlayerB as isize => match self.connect4_player_b.as_mut() {
                None => Err("Player B is not entered.".to_owned()),
                Some(player_b) => {
                    player_b.set_id(id);
                    Ok(())
                }
            },
            _ => Err("The argument does not indicate player A/B.".to_owned()),
        }
    }
}

#[derive(Default)]
pub struct PlayerInput {
    name: Option<String>,
    id: Option<u64>,
}

impl PlayerInput {
    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    pub fn check_fulfilled(&self) -> Result<(), String> {
        if self.name.is_none() {
            return Err("Player's name is not entered.".to_owned());
        } else if self.id.is_none() {
            return Err("Player's id is not entered.".to_owned());
        }
        Ok(())
    }
    pub fn create_connect4_player(&self) -> Result<Connect4Player, String> {
        let Some(name) = self.name.as_ref() else {
            return Err("name not set.".to_owned());
        };
        let Some(id) = self.id.as_ref() else {
            return Err("id not set.".to_owned());
        };
        Ok(Connect4Player::new(name.to_owned(), *id))
    }
}
