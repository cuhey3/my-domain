#[cfg(test)]
mod tests;

use crate::connect4::structs::board::Connect4Board;
use my_board_game::TwoPlayer;
use rand::rngs::SmallRng;
use rand::{Rng, RngCore, SeedableRng};

pub struct Connect4Simulate {
    rng: SmallRng,
    board: Connect4Board,
    result: [(usize, Option<isize>); 7],
    checkmate_result: (TwoPlayer, Vec<usize>),
}

struct SimulationInner {
    rng: SmallRng,
    board: Connect4Board,
}

impl Connect4Simulate {
    pub fn new(
        board: &Connect4Board,
        seed: u64,
        checkmate_result: (TwoPlayer, Vec<usize>),
    ) -> Self {
        Connect4Simulate {
            rng: SmallRng::seed_from_u64(seed),
            board: board.clone(),
            result: [(0, None); 7],
            checkmate_result,
        }
    }

    // TODO
    // 呼び出し時点で勝者が決まっている場合は早期return
    pub fn simulate(&mut self) {
        let stone = self.board.get_next_player();
        let mut result = [(1, None); 7];
        for (i, item) in result.iter_mut().enumerate() {
            let index = i + 1;
            *item = (index, None);
            let mut board_clone = self.board.clone();
            let Ok(_) = board_clone.safe_move(index) else {
                continue;
            };
            board_clone.judge();
            let winner = board_clone.winner();
            if winner.exist() {
                if winner == stone {
                    *item = (index, Some(100));
                } else {
                    *item = (index, Some(-100));
                }
                continue;
            }
            let mut simulation_count = 0;
            let mut win_count = 0;
            let mut has_checkmate = false;
            while simulation_count < 100 {
                let mut inner = SimulationInner::new(
                    &board_clone,
                    SmallRng::seed_from_u64(self.rng.next_u64()),
                );
                let test_has_checkmate = inner.random_move_until_finished();
                if test_has_checkmate {
                    has_checkmate = true;
                    break;
                }
                let winner = inner.board.winner();
                if winner.exist() {
                    if winner == stone {
                        win_count += 1;
                    } else {
                        win_count -= 1;
                    }
                };
                simulation_count += 1;
            }
            if has_checkmate {
                *item = (index, Some(-100))
            } else {
                *item = (index, Some(win_count))
            }
        }
        self.result = result;
        self.sort_result()
    }

    pub fn show_result(&mut self) {
        let own_stone = self.board.get_next_player();
        let checkmate_stone = self.checkmate_result.0;
        let print_str: String = self
            .result
            .iter()
            .map(|(index, count)| {
                format!(
                    "{}({}) ",
                    index,
                    if count.is_none() {
                        "-".into()
                    } else if checkmate_stone == TwoPlayer::None {
                        if self.checkmate_result.1.contains(index) {
                            format!("{}", count.unwrap())
                        } else {
                            "-100".into()
                        }
                    } else if checkmate_stone == own_stone {
                        if self.checkmate_result.1.contains(index) {
                            "詰み(勝ち)".into()
                        } else {
                            format!("{}", count.unwrap())
                        }
                    } else {
                        "詰み(負け)".into()
                    }
                )
            })
            .collect();
        println!("{}", print_str);
    }

    fn sort_result(&mut self) {
        let own_stone = self.board.get_next_player();
        self.result.sort_by_key(|(move_index, count)| match count {
            Some(count) => match self.checkmate_result.0 {
                TwoPlayer::None => {
                    if self.checkmate_result.1.contains(move_index) {
                        *count * -1
                    } else {
                        isize::MAX
                    }
                }
                stone if stone == own_stone => {
                    if self.checkmate_result.1.contains(move_index) {
                        isize::MIN
                    } else {
                        *count * -1
                    }
                }
                _ => *count * -1,
            },
            None => isize::MAX,
        });
    }

    pub fn get_best_move_with_checkmate_search(&self) -> Option<usize> {
        self.result
            .iter()
            .find(|(move_index, _)| self.checkmate_result.1.contains(move_index))
            .map(|tup| tup.0)
    }
}

impl SimulationInner {
    fn new(board: &Connect4Board, rng: SmallRng) -> Self {
        SimulationInner {
            rng,
            board: board.clone(),
        }
    }

    fn random_move(&mut self) {
        loop {
            let index: usize = self.rng.random_range(0..7);
            if let Ok(()) = self.board.safe_move(index + 1) {
                break;
            };
        }
    }

    fn random_move_until_finished(&mut self) -> bool {
        let mut random_move_count = 0;
        while !self.board.is_fill() && !self.board.winner().exist() {
            self.random_move();
            random_move_count += 1;
            self.board.judge();
        }
        random_move_count == 1
    }
}
