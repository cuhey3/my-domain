use crate::BoardGames;
use crate::connect4::Connect4Drawer;
use crate::shogi55::Shogi55Drawer;
use board_games::{init_connect4, init_shogi55};
use my_board_game::{AnswerType, Drawer, GameSystem};
use std::io;

pub struct BoardGameRunner {
    game_system: GameSystem,
    drawer: Box<dyn Drawer>,
}

impl BoardGameRunner {
    pub fn new_with_name(board_game_name: BoardGames) -> Self {
        let seed = getrandom::u64().unwrap();
        match board_game_name {
            BoardGames::Connect4 => BoardGameRunner {
                game_system: init_connect4(seed),
                drawer: Box::new(Connect4Drawer::default()),
            },
            BoardGames::Shogi55 => BoardGameRunner {
                game_system: init_shogi55(seed),
                drawer: Box::new(Shogi55Drawer::default()),
            },
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let game_data = &self.game_system.game_data.clone();
            let Some(phase) = self.game_system.get_phase() else {
                return Err(format!("phase not found: {}", &self.game_system.phase_id));
            };

            phase.read_data(game_data)?;
            while let Some((answer_type, args)) = phase.dialog_question() {
                loop {
                    self.drawer.draw(phase.get_draw_data());
                    let guess = match &answer_type {
                        AnswerType::Skip => "".to_string(),
                        AnswerType::Input => {
                            let mut guess = String::new();
                            io::stdin()
                                .read_line(&mut guess)
                                .expect("Failed to read line");
                            guess
                        }
                        _ => panic!(),
                    };
                    if let Err(error) = phase.dialog_answer(guess, args.clone()) {
                        self.drawer.draw_error(error);
                    } else {
                        break;
                    };
                }
            }

            phase.write_data(game_data)?;
            if let Some(phase_id) = phase.next_phase_id() {
                self.game_system.phase_id = phase_id;
            } else {
                break Ok(());
            }
        }
    }
}
