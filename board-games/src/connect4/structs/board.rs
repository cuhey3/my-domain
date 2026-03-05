#[cfg(test)]
mod tests;

use my_board_game::TwoPlayer;

#[derive(Default, Clone)]
pub struct Connect4Board {
    last_x: usize,
    last_y: usize,
    last_stone: TwoPlayer,
    board: [[TwoPlayer; 6]; 7],
    column_height: [usize; 7],
    winner: TwoPlayer,
    pub stone_count: usize,
    moves: Vec<Connect4Move>,
}

impl Connect4Board {
    pub fn draw(&self) {
        println!("1 2 3 4 5 6 7");
        for y in (0..6).rev() {
            let row_string = (0..7)
                .map(|x| match self.board[x][y] {
                    TwoPlayer::None => "  ",
                    TwoPlayer::First => "■ ",
                    TwoPlayer::Second => "□ ",
                })
                .collect::<String>();
            println!("{}", row_string);
        }
        println!("1 2 3 4 5 6 7");
    }

    fn check_input(&self, input_index: usize) -> Result<(), String> {
        if !(1..=7).contains(&input_index) {
            return Err("数字は1-7の範囲で入力してください".into());
        }
        Ok(())
    }
    pub fn safe_move(&mut self, input_index: usize) -> Result<(), String> {
        if self.winner.exist() {
            return Err("勝敗が決しています".into());
        }
        if self.is_fill() {
            return Err("この盤にはもう置けません".into());
        }
        self.check_input(input_index)?;
        let last_x = input_index - 1;
        if self.column_height[last_x] > 5 {
            return Err("その列にはもう置けません".into());
        }
        let stone = self.last_stone.next();
        let connect4_move = Connect4Move::new(input_index, stone);
        self.accept_move(&connect4_move)?;
        self.moves.push(connect4_move);
        self.judge();
        Ok(())
    }

    fn accept_move(&mut self, connect4_move: &Connect4Move) -> Result<(), String> {
        let Connect4Move { index, stone } = connect4_move;
        self.last_x = index - 1;
        self.last_y = self.column_height[self.last_x];
        self.last_stone = *stone;
        self.board[self.last_x][self.last_y] = *stone;
        self.column_height[self.last_x] += 1;
        self.stone_count += 1;
        Ok(())
    }

    pub fn reject_last_2_move(&mut self) -> Result<(), String> {
        if self.stone_count < 2 {
            return Err("待ったできません".into());
        }
        if self.stone_count == 2 {
            *self = Connect4Board::default();
        } else {
            let last_move = self.moves.pop().unwrap();
            self.reject_move(&last_move)?;
            let last_move = self.moves.pop().unwrap();
            self.reject_move(&last_move)?;
            let new_last_move = *self.moves.last().unwrap();
            self.reject_move(&new_last_move)?;
            self.accept_move(&new_last_move)?;
        }
        Ok(())
    }

    fn reject_move(&mut self, connect4_move: &Connect4Move) -> Result<(), String> {
        let Connect4Move { index, .. } = connect4_move;
        let last_x = index - 1;
        self.column_height[last_x] -= 1;
        let last_y = self.column_height[last_x];
        self.board[last_x][last_y] = TwoPlayer::None;
        self.stone_count -= 1;
        self.winner = TwoPlayer::None;
        Ok(())
    }

    pub fn reject_last_one_move(&mut self) -> Result<(), String> {
        if self.stone_count < 1 {
            return Err("待ったできません".into());
        }
        if self.stone_count == 1 {
            *self = Connect4Board::default();
        } else {
            let last_move = self.moves.pop().unwrap();
            self.reject_move(&last_move)?;
            let new_last_move = *self.moves.last().unwrap();
            self.reject_move(&new_last_move)?;
            self.accept_move(&new_last_move)?;
        }
        Ok(())
    }
    pub fn judge(&mut self) -> bool {
        if self.last_stone == TwoPlayer::None {
            false
        } else if self.check_pattern(self.last_x, self.last_y, self.last_stone) {
            self.winner = self.last_stone;
            true
        } else {
            false
        }
    }

    fn get_stone_include_outside(&self, x: i32, y: i32) -> TwoPlayer {
        if x < 0 || y < 0 || 6 < x || 5 < y {
            return TwoPlayer::None;
        }
        let x = x as usize;
        let y = y as usize;
        self.board[x][y]
    }

    fn check_pattern(&self, x: usize, y: usize, value: TwoPlayer) -> bool {
        let x = x as i32;
        let y = y as i32;
        let patterns = [
            // ＼
            (-1, -1),
            // ─
            (-1, 0),
            // ／
            (-1, 1),
            // │
            (0, 1),
        ];
        patterns.iter().any(|(x_add, y_add)| {
            let mut matched_len = 0;
            for i in 1..4 {
                let test_x = x + x_add * i;
                let test_y = y + y_add * i;
                let test_result = self.get_stone_include_outside(test_x, test_y) == value;
                if test_result {
                    matched_len += 1;
                } else {
                    break;
                }
            }
            if matched_len == 3 {
                return true;
            }
            for i in 1..(4 - matched_len) {
                let test_x = x + -x_add * i;
                let test_y = y + -y_add * i;
                let test_result = self.get_stone_include_outside(test_x, test_y) == value;
                if !test_result {
                    return false;
                }
            }
            true
        })
    }
    pub fn winner(&self) -> TwoPlayer {
        self.winner
    }

    pub fn is_fill(&self) -> bool {
        self.stone_count > 41
    }
    pub fn is_first_player_turn(&self) -> bool {
        self.last_stone != TwoPlayer::First
    }
    pub fn get_next_player(&self) -> TwoPlayer {
        self.last_stone.next()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Connect4Move {
    index: usize,
    stone: TwoPlayer,
}

impl Connect4Move {
    fn new(index: usize, stone: TwoPlayer) -> Self {
        Connect4Move { index, stone }
    }
}
