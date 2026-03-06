use crate::framework::TwoPlayer;
use crate::shogi55::structs::board::Shogi55Place;
use crate::shogi55::structs::piece::{Direction, Piece};
use crate::shogi55::structs::piece_info::PieceInfo;
use crate::shogi55::structs::possibility::Possible;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type CheckingWithPreconditionCache = HashMap<(usize, bool), (bool, Vec<(Shogi55Place, Possible)>)>;
type KingInfoCache = [Option<(Shogi55Place, PieceInfo)>; 2];

#[derive(Default, Clone)]
pub struct BoardInner {
    placed_map: HashMap<Shogi55Place, PieceInfo>,
    king_info_cache: Arc<Mutex<KingInfoCache>>,
    checking_with_precondition_cache: Arc<Mutex<CheckingWithPreconditionCache>>,
}

impl BoardInner {
    pub fn placed_map(&self) -> &HashMap<Shogi55Place, PieceInfo> {
        &self.placed_map
    }

    pub fn placed_map_insert(&mut self, key: Shogi55Place, value: PieceInfo) -> Option<PieceInfo> {
        self.cache_clear();
        self.placed_map.insert(key, value)
    }

    pub fn placed_map_remove(&mut self, key: &Shogi55Place) -> Option<PieceInfo> {
        self.cache_clear();
        self.placed_map.remove(key)
    }

    pub fn placed_map_get_mut(&mut self, key: &Shogi55Place) -> Option<&mut PieceInfo> {
        self.placed_map.get_mut(key)
    }

    pub fn get_king_info(&self, player: &TwoPlayer) -> (Shogi55Place, PieceInfo) {
        let player_index = player.get_index();
        if let Some(cache) = &self.king_info_cache.lock().unwrap()[player_index] {
            cache.clone()
        } else {
            let (place, piece_info) = self
                .placed_map()
                .iter()
                .find(|(_, piece_info)| {
                    piece_info.get_player() == player && piece_info.get_piece() == &Piece::King
                })
                .unwrap();
            self.king_info_cache.lock().unwrap()[player_index] = Some((*place, piece_info.clone()));
            (*place, piece_info.clone())
        }
    }

    pub fn get_checking_with_precondition(
        &self,
        player: &TwoPlayer,
        include_blocked: bool,
    ) -> (bool, Vec<(Shogi55Place, Possible)>) {
        let player_index = player.get_index();
        if let Some(cache) = &self
            .checking_with_precondition_cache
            .lock()
            .unwrap()
            .get(&(player_index, include_blocked))
        {
            (*cache).clone()
        } else {
            let (king_place, _) = self.get_king_info(player);
            let mut is_checked = false;
            let possibles = self
                .placed_pieces_of_player(&player.next())
                .iter()
                .filter_map(|(place, piece_info)| {
                    let possible = piece_info.get_possibility().get_possible(&king_place);
                    match possible {
                        Possible::Blocked(_) if include_blocked => {
                            Some((**place, possible.clone()))
                        }
                        Possible::Move(_) => {
                            is_checked = true;
                            Some((**place, possible.clone()))
                        }
                        _ => None,
                    }
                })
                .collect::<Vec<_>>();
            self.checking_with_precondition_cache
                .lock()
                .unwrap()
                .insert(
                    (player_index, include_blocked),
                    (is_checked, possibles.clone()),
                );
            (is_checked, possibles)
        }
    }
    pub fn placed_pieces_of_player(&self, player: &TwoPlayer) -> Vec<(&Shogi55Place, &PieceInfo)> {
        self.placed_map()
            .iter()
            .filter(|(_, piece_info)| piece_info.get_player() == player)
            .collect()
    }
    fn cache_clear(&self) {
        *self.king_info_cache.lock().unwrap() = [None, None];
        *self.checking_with_precondition_cache.lock().unwrap() = HashMap::new();
    }

    pub fn is_drop_pawn_mate(&self, player: &TwoPlayer, to_place: &Shogi55Place) -> bool {
        let opponent_player = player.next();
        let (king_place, _) = self.get_king_info(&opponent_player);

        let mut direction = Direction::Up(false, 1);
        if player == &TwoPlayer::Second {
            direction = direction.inverse();
        };

        // 歩の効いている場所＝相手の玉の場所だったら打ち歩詰めの可能性がある
        direction.add_mod(to_place)
            .unwrap()
            == king_place
            // 打った歩を相手が玉以外で取ることができなければ打ち歩詰めの可能性がある
            && self.placed_pieces_of_player(&opponent_player).iter().all(
            |(_, piece_info)| {
                if matches!(piece_info.get_piece(), Piece::King) {
                    return true;
                }
                !piece_info
                    .get_possibility()
                    .get_possible(to_place)
                    .can_move()
            },
        )   // 相手玉の動かせる場所がなければ打ち歩詰めが完成
            && {
            let possible_moves = self.get_possible_moves_of_king(&opponent_player);
            possible_moves.is_empty()
        }
    }

    pub fn get_possible_moves_of_king(&self, player: &TwoPlayer) -> Vec<Shogi55Place> {
        let (_, king_info) = self.get_king_info(player);
        let opponent_pieces = self.placed_pieces_of_player(&player.next());
        king_info
            .get_possibility()
            .get_place_to_possible()
            .iter()
            .filter_map(|(place, possible)| {
                // 玉から見て Capture か Move でなければ動ける場所ではない
                if !matches!(possible, Possible::Capture(_) | Possible::Move(_)) {
                    None
                } else if opponent_pieces.iter().all(|(_, piece_info)| {
                    // 玉が動こうとしている場所に対して敵の駒全てが Nothing か Blocked か Placed ならその手は合法手
                    matches!(
                        piece_info.get_possibility().get_possible(place),
                        Possible::Nothing | Possible::Blocked(_) | Possible::Placed
                    )
                }) {
                    Some(*place)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
