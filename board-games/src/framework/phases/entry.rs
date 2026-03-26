use crate::framework::phases::CommonPhase;
use crate::framework::structs::common_draw_data::{CommonDrawData, CommonDrawTask};
use crate::framework::structs::common_game_data::CommonGameData;
use crate::framework::structs::common_player::CommonPlayer;
use crate::framework::{AnswerType, Constants, Phase};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct CommonEntryPhase {
    state_position: usize,
    shogi55_player_a: Option<PlayerInput>,
    shogi55_player_b: Option<PlayerInput>,
    has_cpu: bool,
    is_online: bool,
    common_draw_data: CommonDrawData,
}

impl Phase for CommonEntryPhase {
    fn get_phase_id(&self) -> usize {
        CommonPhase::Entry as usize
    }

    fn dialog_question(&mut self) -> Option<(AnswerType, Vec<isize>)> {
        match self.state_position {
            0 => {
                let own_name_expression = if self.is_online || self.has_cpu {
                    "あなた"
                } else {
                    "一人目"
                };

                self.add_common_draw_task(CommonDrawTask::Question(format!(
                    "{own_name_expression}の名前を入力してください"
                )));

                Some((AnswerType::Input, vec![Constants::PlayerA as isize]))
            }
            1 => {
                let own_name_expression = if self.is_online || self.has_cpu {
                    "あなた"
                } else {
                    "一人目"
                };

                self.add_common_draw_task(CommonDrawTask::Question(format!(
                    "{own_name_expression}のidを入力してください"
                )));

                Some((AnswerType::Input, vec![Constants::PlayerA as isize]))
            }
            2 => {
                self.add_common_draw_task(CommonDrawTask::Question(
                    "二人目の名前を入力してください".into(),
                ));

                Some((AnswerType::Input, vec![Constants::PlayerB as isize]))
            }
            3 => {
                self.add_common_draw_task(CommonDrawTask::Question(
                    "二人目のidを入力してください".into(),
                ));

                Some((AnswerType::Input, vec![Constants::PlayerB as isize]))
            }
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

                if self.has_cpu || self.is_online {
                    self.shogi55_player_b = Some(PlayerInput {
                        // TODO
                        // is_online の場合は CPU ではない
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
        if self.is_online {
            Some(CommonPhase::OnlineDecideFirstPlayer as usize)
        } else {
            Some(CommonPhase::DecideFirstPlayer as usize)
        }
    }

    fn read_common_data(&mut self, game_data: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        let game_data = game_data.borrow();

        self.has_cpu = game_data.has_cpu();

        self.is_online = game_data.is_online();

        Ok(())
    }

    fn write_common_data(&mut self, game_data: &Rc<RefCell<CommonGameData>>) -> Result<(), String> {
        let mut game_data = game_data.borrow_mut();

        let shogi55_player = &self
            .shogi55_player_a
            .as_ref()
            .ok_or("shogi55_player_a is none.")?;

        let player = shogi55_player.create_common_player()?;

        game_data.set_first_player(player);

        let shogi55_player = &self
            .shogi55_player_b
            .as_ref()
            .ok_or("shogi55_player_b is none.")?;

        let player = shogi55_player.create_common_player()?;

        game_data.set_second_player(player);

        Ok(())
    }

    fn get_common_draw_data(&mut self) -> &mut CommonDrawData {
        &mut self.common_draw_data
    }
}

impl CommonEntryPhase {
    fn entry(&mut self, args: &[isize]) -> Result<(), String> {
        match args.first() {
            None => Err("Arguments require input of player A/B.".to_owned()),
            Some(i) if *i == Constants::PlayerA as isize => {
                if self.shogi55_player_a.is_none() {
                    self.shogi55_player_a = Some(PlayerInput::default());
                    Ok(())
                } else {
                    Err("Player A already entered.".to_owned())
                }
            }
            Some(i) if *i == Constants::PlayerB as isize => {
                if self.shogi55_player_b.is_none() {
                    self.shogi55_player_b = Some(PlayerInput::default());
                    Ok(())
                } else {
                    Err("Player B already entered.".to_owned())
                }
            }
            _ => Err("The argument does not indicate player A/B.".to_owned()),
        }
    }

    fn check_player(&self, _: &Vec<isize>) -> Result<(), String> {
        self.shogi55_player_a
            .as_ref()
            .ok_or("Player A is not entered.")?
            .check_fulfilled()?;

        self.shogi55_player_b
            .as_ref()
            .ok_or("Player B is not entered.")?
            .check_fulfilled()?;

        Ok(())
    }
    fn set_name(&mut self, name: &str, args: &[isize]) -> Result<(), String> {
        match args.first() {
            None => Err("Arguments require input of player A/B.".to_owned()),
            Some(i) if *i == Constants::PlayerA as isize => match self.shogi55_player_a.as_mut() {
                None => Err("Player A is not entered.".to_owned()),
                Some(player_a) => {
                    player_a.set_name(name.to_owned());
                    Ok(())
                }
            },
            Some(i) if *i == Constants::PlayerB as isize => match self.shogi55_player_b.as_mut() {
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
            Some(i) if *i == Constants::PlayerA as isize => match self.shogi55_player_a.as_mut() {
                None => Err("Player A is not entered.".to_owned()),
                Some(player_a) => {
                    player_a.set_id(id);
                    Ok(())
                }
            },
            Some(i) if *i == Constants::PlayerB as isize => match self.shogi55_player_b.as_mut() {
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
        self.name.as_ref().ok_or("Player's name is not entered.")?;

        self.id.as_ref().ok_or("Player's id is not entered.")?;

        Ok(())
    }
    pub fn create_common_player(&self) -> Result<CommonPlayer, String> {
        let name = self.name.as_ref().ok_or("name not set.")?;

        let id = self.id.as_ref().ok_or("id not set.")?;

        Ok(CommonPlayer::new(name.to_owned(), *id))
    }
}
