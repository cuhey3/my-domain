use crate::shogi55::structs::board::Shogi55Place;
use std::collections::HashMap;

// 盤面にある駒から盤面全体を見て、それぞれのマスがどのような状態にあるかを指す enum
#[derive(Default, Clone, Debug)]
pub enum Possible {
    #[default]
    // 何も影響を与えられない
    Nothing,
    // 駒が存在している場所を表す
    Placed,
    // 敵の駒により自分の移動がブロックされている状態（ブロックが外れる (is_blocking_over() = true)と駒が取れる）
    Blocked(Vec<Shogi55Place>),
    // 味方の駒があり、もしその駒を敵に取られても次で敵の駒を取り返せる場所
    Affect(Vec<Shogi55Place>),
    // 相手の駒を取れる場所（その駒が玉の場合は実装の都合により Move 扱いとなる）
    Capture(Vec<Shogi55Place>),
    // 駒がなく、その場所に移動できる（玉の場合は Capture ではなく Move 扱いとなる）
    Move(Vec<Shogi55Place>),
}

impl Possible {
    pub fn can_move(&self) -> bool {
        matches!(self, Possible::Move(_) | Possible::Capture(_))
    }

    // from でブロックしていてかつ to がブロックしていなければ "ブロックが外れた"
    pub fn is_blocking_over(&self, from: &Shogi55Place, to: &Shogi55Place) -> bool {
        matches!(self, Possible::Blocked(precondition) if precondition.contains(from) && !precondition.contains(to))
    }
}

#[derive(Debug, Clone)]
pub struct Possibility {
    place_to_possible: HashMap<Shogi55Place, Possible>,
}

impl Possibility {
    pub fn new() -> Self {
        Self {
            place_to_possible: HashMap::new(),
        }
    }

    pub fn get_possible(&self, place: &Shogi55Place) -> &Possible {
        self.place_to_possible
            .get(place)
            .unwrap_or(&Possible::Nothing)
    }

    pub fn set_possible(&mut self, place: &Shogi55Place, possible: Possible) {
        self.place_to_possible.insert(*place, possible);
    }

    pub fn get_place_to_possible(&self) -> &HashMap<Shogi55Place, Possible> {
        &self.place_to_possible
    }

    // Possibility 再計算用に precondition の中身を見て place が含まれているかを確認する
    pub fn precondition_contains(&self, place: &Shogi55Place) -> bool {
        self.place_to_possible
            .values()
            .any(|possible| match possible {
                Possible::Blocked(precondition) if precondition.contains(place) => true,
                Possible::Affect(precondition) if precondition.contains(place) => true,
                Possible::Capture(precondition) if precondition.contains(place) => true,
                Possible::Move(precondition) if precondition.contains(place) => true,
                _ => false,
            })
    }
}
