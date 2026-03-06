use crate::shogi55::structs::board::{Shogi55Board, Shogi55Move};
use rand::SeedableRng;
use rand::prelude::SmallRng;

pub struct Shogi55Simulate {
    board: Shogi55Board,
    rng: SmallRng,
}

impl Shogi55Simulate {
    pub fn get_simulate(board: &Shogi55Board, seed: u64) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(seed),
            board: board.clone(),
        }
    }

    pub fn simulate(&mut self) {}

    pub fn get_best_move_with_eval_value(&mut self) -> (Shogi55Move, i32) {
        let pieces_with_moves = self.board.get_all_possible_moves();
        let (index, point) = self.board.nest_search(3, &mut self.rng);
        (pieces_with_moves[index].clone(), point)
    }
}
