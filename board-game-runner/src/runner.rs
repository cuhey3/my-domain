use crate::BoardGames;
use crate::connect4::Connect4Drawer;
use crate::shogi55::Shogi55Drawer;
use bid_of_power::init_bop;
use bid_of_power::structs::BoPDrawer;
use board_games::framework::structs::common_draw_data::{CommonDrawData, CommonDrawTask};
use board_games::framework::{AnswerType, Drawer, GameSystem};
use board_games::{init_connect4, init_shogi55};
use http_client_adapter::http_client_adapter_impl::HttpClientAdapterImpl;
use matching_if::webrtc::matching_sequence::MatchingSequence;
use std::{env, io};
use webrtc_adapter::peer_connection_adapter_impl::PeerConnectionAdapterImpl;
use webrtc_if::peer_connection_adapter::PeerConnectionAdapter;

pub struct BoardGameRunner {
    game_system: GameSystem,
    peer_connection_wrapper: Option<PeerConnectionAdapterImpl>,
    drawer: Box<dyn Drawer>,
    common_drawer: CommonDrawer,
}

impl BoardGameRunner {
    pub fn new_with_name(board_game_name: BoardGames) -> Self {
        let seed = getrandom::u64().unwrap();
        match board_game_name {
            BoardGames::Connect4 => BoardGameRunner {
                game_system: init_connect4(seed),
                peer_connection_wrapper: None,
                drawer: Box::new(Connect4Drawer::default()),
                common_drawer: CommonDrawer,
            },
            BoardGames::Shogi55 => BoardGameRunner {
                game_system: init_shogi55(seed),
                peer_connection_wrapper: None,
                drawer: Box::new(Shogi55Drawer::default()),
                common_drawer: CommonDrawer,
            },
        }
    }

    pub fn new_for_dev() -> Self {
        let seed = getrandom::u64().unwrap();
        Self {
            game_system: init_bop(seed),
            peer_connection_wrapper: None,
            drawer: Box::new(BoPDrawer::default()),
            common_drawer: CommonDrawer {},
        }
    }

    pub async fn run(&mut self) -> Result<(), String> {
        let domain =
            env::var("MATCHING_SERVER_DOMAIN").map_err(|_| "MATCHING_SERVER_DOMAIN not set")?;

        let port = env::var("MATCHING_SERVER_PORT").unwrap_or("443".to_owned());

        let matching_server_url = format!("{}:{}", domain, port);

        loop {
            let game_data = &self.game_system.game_data.clone();
            let common_game_data = &self.game_system.common_game_data.clone();

            let phase_id = self.game_system.phase_id;

            let phase = self
                .game_system
                .get_phase()
                .ok_or(format!("phase not found: {phase_id}"))?;

            if phase.is_required_matching() {
                rustls::crypto::ring::default_provider()
                    .install_default()
                    .expect("Failed to install rustls crypto provider");

                let peer_connection_wrapper = MatchingSequence::<
                    PeerConnectionAdapterImpl,
                    HttpClientAdapterImpl,
                >::new(matching_server_url.to_owned())
                .get_peer_connection_wrapper()
                .await
                .map_err(|err| format!("matching failed: {err}"))?;

                phase.set_is_player_a(peer_connection_wrapper.is_offerer());

                self.peer_connection_wrapper = Some(peer_connection_wrapper);
            }
            phase.read_common_data(common_game_data)?;
            phase.read_data(game_data)?;

            while let Some((answer_type, args)) = phase.dialog_question() {
                loop {
                    self.common_drawer.draw(phase.get_common_draw_data());
                    if phase.has_draw_task() {
                        self.drawer.draw(phase.get_draw_data());
                    }

                    if let AnswerType::WaitWithMessage(ref message) = answer_type {
                        self.peer_connection_wrapper
                            .as_ref()
                            .unwrap()
                            .send_json(message)
                            .await
                            .map_err(|err| format!("data channel send error: {err}"))?;
                    }

                    if let AnswerType::NoWaitWithMessage(ref message) = answer_type {
                        self.peer_connection_wrapper
                            .as_ref()
                            .unwrap()
                            .send_json(message)
                            .await
                            .map_err(|err| format!("data channel send error: {err}"))?;
                    }

                    if matches!(
                        answer_type,
                        AnswerType::Wait | AnswerType::WaitWithMessage(_)
                    ) {
                        let json = self
                            .peer_connection_wrapper
                            .as_mut()
                            .unwrap()
                            .wait_message_json()
                            .await
                            .map_err(|err| format!("cannot receive message: {err}"))?;

                        phase
                            .dialog_answer_json(&json)
                            .map_err(|err| format!("dialog answer json failed: {err}"))?;

                        break;
                    }

                    let guess = match &answer_type {
                        AnswerType::Skip => "".to_string(),
                        AnswerType::NoWaitWithMessage(_) => "".to_string(),
                        AnswerType::Input => {
                            let mut guess = String::new();
                            io::stdin()
                                .read_line(&mut guess)
                                .expect("Failed to read line");
                            guess
                        }
                        _ => panic!(),
                    };

                    if let Err(error) = phase.dialog_answer(guess.clone(), args.clone()) {
                        self.drawer.draw_error(error);
                    } else {
                        if !guess.is_empty()
                            && let Some(wrapper) = &self.peer_connection_wrapper
                        {
                            wrapper.send_json(&guess).await?;
                        }

                        break;
                    };
                }
            }

            phase.write_common_data(common_game_data)?;
            phase.write_data(game_data)?;

            if phase.has_draw_task() {
                self.drawer.draw(phase.get_draw_data());
            }

            if let Some(phase_id) = phase.next_phase_id() {
                self.game_system.phase_id = phase_id;
            } else {
                break Ok(());
            }
        }
    }
}

struct CommonDrawer;

impl CommonDrawer {
    fn draw(&mut self, draw_data: &mut CommonDrawData) {
        while let Some(task) = draw_data.take_task() {
            match task {
                CommonDrawTask::Question(message) => println!("{}", message),
                CommonDrawTask::Message(message) => println!("{}", message),
                CommonDrawTask::DebugMessage(message) => println!("{}", message),
                CommonDrawTask::EvaluateValue(message) => println!("評価値: {}", message),
                CommonDrawTask::GameResult(result) => println!("{}", result),
                _ => {}
            }
        }
    }
}
