use crate::framework::TwoPlayer;
use crate::shogi55::structs::board::Shogi55Place;
use std::str::FromStr;

pub enum Direction {
    Up(bool, isize),
    Down(bool, isize),
    Left(bool, isize),
    Right(bool, isize),
    UpLeft(bool, isize),
    UpRight(bool, isize),
    DownLeft(bool, isize),
    DownRight(bool, isize),
    TwoUpLeft,
    TwoUpRight,
    TwoDownLeft,
    TwoDownRight,
}

impl Direction {
    pub fn inverse(&self) -> Direction {
        match self {
            Direction::Up(b, c) => Direction::Down(*b, *c),
            Direction::Down(b, c) => Direction::Up(*b, *c),
            Direction::Left(b, c) => Direction::Right(*b, *c),
            Direction::Right(b, c) => Direction::Left(*b, *c),
            Direction::UpLeft(b, c) => Direction::DownRight(*b, *c),
            Direction::UpRight(b, c) => Direction::DownLeft(*b, *c),
            Direction::DownLeft(b, c) => Direction::UpRight(*b, *c),
            Direction::DownRight(b, c) => Direction::UpLeft(*b, *c),
            Direction::TwoUpLeft => Direction::TwoDownRight,
            Direction::TwoUpRight => Direction::TwoDownLeft,
            Direction::TwoDownLeft => Direction::TwoUpRight,
            Direction::TwoDownRight => Direction::TwoUpLeft,
        }
    }

    pub fn next(&self) -> Option<Direction> {
        match self {
            Direction::Up(true, c) => Some(Direction::Up(true, c + 1)),
            Direction::Down(true, c) => Some(Direction::Down(true, c + 1)),
            Direction::Left(true, c) => Some(Direction::Left(true, c + 1)),
            Direction::Right(true, c) => Some(Direction::Right(true, c + 1)),
            Direction::UpLeft(true, c) => Some(Direction::UpLeft(true, c + 1)),
            Direction::UpRight(true, c) => Some(Direction::UpRight(true, c + 1)),
            Direction::DownLeft(true, c) => Some(Direction::DownLeft(true, c + 1)),
            Direction::DownRight(true, c) => Some(Direction::DownRight(true, c + 1)),
            _ => None,
        }
    }

    pub fn add_mod(&self, place2: &Shogi55Place) -> Option<Shogi55Place> {
        let (col_mod, row_mod) = match self {
            Direction::Up(_, c) => (0, -c),
            Direction::Down(_, c) => (0, *c),
            Direction::Left(_, c) => (-c, 0),
            Direction::Right(_, c) => (*c, 0),
            Direction::UpLeft(_, c) => (*c, -c),
            Direction::UpRight(_, c) => (-c, -c),
            Direction::DownLeft(_, c) => (*c, *c),
            Direction::DownRight(_, c) => (-c, *c),
            Direction::TwoUpLeft => (1, -2),
            Direction::TwoUpRight => (-1, -2),
            Direction::TwoDownLeft => (1, 2),
            Direction::TwoDownRight => (-1, 2),
        };

        let new_col = place2.get_col() as isize + col_mod;
        match new_col {
            1..6 => {
                let new_row = place2.get_row() as isize + row_mod;
                match new_row {
                    1..6 => Some(Shogi55Place::new(new_col as usize, new_row as usize)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    King = 0,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
    PromotedRook,
    PromotedBishop,
    PromotedSilver,
    PromotedKnight,
    PromotedLance,
    PromotedPawn,
}

impl Piece {
    pub fn point(&self) -> i32 {
        match self {
            Piece::King => 100,
            Piece::Rook => 7,
            Piece::Bishop => 6,
            Piece::Gold => 5,
            Piece::Silver => 4,
            Piece::Knight => 3,
            Piece::Lance => 2,
            Piece::Pawn => 1,
            Piece::PromotedRook => 9,
            Piece::PromotedBishop => 8,
            Piece::PromotedSilver => 5,
            Piece::PromotedKnight => 4,
            Piece::PromotedLance => 3,
            Piece::PromotedPawn => 2,
        }
    }

    pub fn captured_point(&self) -> i32 {
        match self {
            Piece::King => 200, // 何となく取った方がポイントが多いようにしておく
            Piece::Rook => 6,
            Piece::Bishop => 6,
            Piece::Gold => 4,
            Piece::Silver => 3,
            Piece::Knight => 2,
            Piece::Lance => 2,
            Piece::Pawn => 1,
            _ => 0, // 成駒は取られた時点で普通の駒に戻るから設定不要
        }
    }
    pub fn kanji(&self) -> &str {
        match self {
            Piece::King => "王",
            Piece::Rook => "飛",
            Piece::Bishop => "角",
            Piece::Gold => "金",
            Piece::Silver => "銀",
            Piece::Knight => "桂",
            Piece::Lance => "香",
            Piece::Pawn => "歩",
            Piece::PromotedRook => "龍",
            Piece::PromotedBishop => "馬",
            Piece::PromotedSilver => "全",
            Piece::PromotedKnight => "圭",
            Piece::PromotedLance => "杏",
            Piece::PromotedPawn => "と",
        }
    }

    pub fn get_directions(&self) -> Vec<Direction> {
        match self {
            Piece::King => vec![
                Direction::Up(false, 1),
                Direction::Down(false, 1),
                Direction::Left(false, 1),
                Direction::Right(false, 1),
                Direction::UpLeft(false, 1),
                Direction::UpRight(false, 1),
                Direction::DownLeft(false, 1),
                Direction::DownRight(false, 1),
            ],
            Piece::Rook => vec![
                Direction::Up(true, 1),
                Direction::Down(true, 1),
                Direction::Left(true, 1),
                Direction::Right(true, 1),
            ],
            Piece::Bishop => vec![
                Direction::UpLeft(true, 1),
                Direction::UpRight(true, 1),
                Direction::DownLeft(true, 1),
                Direction::DownRight(true, 1),
            ],
            Piece::Silver => vec![
                Direction::Up(false, 1),
                Direction::UpLeft(false, 1),
                Direction::UpRight(false, 1),
                Direction::DownLeft(false, 1),
                Direction::DownRight(false, 1),
            ],
            Piece::Knight => vec![Direction::TwoUpLeft, Direction::TwoUpRight],
            Piece::Lance => vec![Direction::Up(true, 1)],
            Piece::Pawn => vec![Direction::Up(false, 1)],
            Piece::PromotedRook => vec![
                Direction::Up(true, 1),
                Direction::Down(true, 1),
                Direction::Left(true, 1),
                Direction::Right(true, 1),
                Direction::UpLeft(false, 1),
                Direction::UpRight(false, 1),
                Direction::DownLeft(false, 1),
                Direction::DownRight(false, 1),
            ],
            Piece::PromotedBishop => vec![
                Direction::Up(false, 1),
                Direction::Down(false, 1),
                Direction::Left(false, 1),
                Direction::Right(false, 1),
                Direction::UpLeft(true, 1),
                Direction::UpRight(true, 1),
                Direction::DownLeft(true, 1),
                Direction::DownRight(true, 1),
            ],
            _ => vec![
                Direction::Up(false, 1),
                Direction::Down(false, 1),
                Direction::Left(false, 1),
                Direction::Right(false, 1),
                Direction::UpLeft(false, 1),
                Direction::UpRight(false, 1),
            ],
        }
    }

    pub fn promote(&self) -> Piece {
        match self {
            Piece::Rook => Piece::PromotedRook,
            Piece::Bishop => Piece::PromotedBishop,
            Piece::Silver => Piece::PromotedSilver,
            Piece::Knight => Piece::PromotedKnight,
            Piece::Lance => Piece::PromotedLance,
            Piece::Pawn => Piece::PromotedPawn,
            _ => *self,
        }
    }

    pub fn can_promote(&self, player: &TwoPlayer, from: &Shogi55Place, to: &Shogi55Place) -> bool {
        match self {
            Piece::Rook
            | Piece::Bishop
            | Piece::Silver
            | Piece::Knight
            | Piece::Lance
            | Piece::Pawn => match player {
                TwoPlayer::Second => from.get_row() == 5 || to.get_row() == 5,
                _ => from.get_row() == 1 || to.get_row() == 1,
            },
            _ => false,
        }
    }

    // 桂香に対応していないロジック
    pub fn is_force_promotion(&self, player: &TwoPlayer, to: &Shogi55Place) -> bool {
        matches!(self, Piece::Pawn)
            && match player {
                TwoPlayer::Second => to.get_row() == 5,
                _ => to.get_row() == 1,
            }
    }
    pub fn captured(&self) -> Piece {
        match self {
            Piece::PromotedRook => Piece::Rook,
            Piece::PromotedBishop => Piece::Bishop,
            Piece::PromotedSilver => Piece::Silver,
            Piece::PromotedKnight => Piece::Knight,
            Piece::PromotedLance => Piece::Lance,
            Piece::PromotedPawn => Piece::Pawn,
            _ => *self,
        }
    }
    pub fn shogi55_in_hand_kanji_set() -> [&'static str; 7] {
        [
            Piece::Rook.kanji(),
            Piece::Bishop.kanji(),
            Piece::Gold.kanji(),
            Piece::Silver.kanji(),
            Piece::Knight.kanji(),
            Piece::Lance.kanji(),
            Piece::Pawn.kanji(),
        ]
    }
}

impl FromStr for Piece {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "王" => Piece::King,
            "飛" => Piece::Rook,
            "角" => Piece::Bishop,
            "金" => Piece::Gold,
            "銀" => Piece::Silver,
            "桂" => Piece::Knight,
            "香" => Piece::Lance,
            "歩" => Piece::Pawn,
            "龍" => Piece::PromotedRook,
            "馬" => Piece::PromotedBishop,
            "全" => Piece::PromotedSilver,
            "圭" => Piece::PromotedKnight,
            "杏" => Piece::PromotedLance,
            "と" => Piece::PromotedPawn,
            _ => return Err(format!("piece not found {}", input)),
        })
    }
}
