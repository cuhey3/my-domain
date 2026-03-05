#[cfg(test)]
mod tests;

use crate::connect4::structs::board::Connect4Board;
use my_board_game::TwoPlayer;

pub struct SearchCheckmate {
    connect4board: Connect4Board,
    nest_count: usize,
}

impl SearchCheckmate {
    pub fn new(connect4board: Connect4Board, nest_count: usize) -> SearchCheckmate {
        SearchCheckmate {
            connect4board,
            nest_count,
        }
    }
    pub fn search(&mut self) -> (TwoPlayer, Vec<usize>) {
        self.nest_search(self.nest_count)
    }

    fn nest_search(&mut self, remain_nest_count: usize) -> (TwoPlayer, Vec<usize>) {
        let own = self.connect4board.get_next_player();
        let opponent = own.opponent();
        let mut moves = vec![];
        // 自分がこの手で勝てるならその結果を返す
        for i in 1..8 {
            if self.connect4board.safe_move(i).is_err() {
                continue;
            };
            let winner = self.connect4board.winner();
            self.connect4board
                .reject_last_one_move()
                .expect("TODO: panic message");
            if winner.exist() {
                // 詰みは一つしか見つけない（高速化のため）
                return (own, vec![i]);
            }
            moves.push(i);
        }
        if remain_nest_count == 0 {
            return (TwoPlayer::None, moves);
        }
        // すぐ勝てる手はないので nested_search
        let mut draw_indexes = vec![];
        for move_index in moves.iter() {
            self.connect4board
                .safe_move(*move_index)
                .expect("TODO: panic message");
            let nested_search = self.nest_search(remain_nest_count - 1);
            self.connect4board
                .reject_last_one_move()
                .expect("TODO: panic message");
            match nested_search {
                (TwoPlayer::None, _) => {
                    draw_indexes.push(*move_index);
                }
                (winner, _) if winner == own => return (own, vec![*move_index]),
                _ => {}
            }
        }
        if !draw_indexes.is_empty() {
            (TwoPlayer::None, draw_indexes)
        } else {
            (opponent, moves)
        }
    }
}
