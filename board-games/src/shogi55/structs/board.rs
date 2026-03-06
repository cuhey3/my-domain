#[cfg(test)]
mod tests;

use crate::framework::TwoPlayer;
use crate::shogi55::structs::board_inner::BoardInner;
use crate::shogi55::structs::piece::Piece;
use crate::shogi55::structs::piece_info::PieceInfo;
use crate::shogi55::structs::possibility::{Possibility, Possible};
use rand::Rng;
use rand::rngs::SmallRng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct Shogi55Board {
    board_inner: BoardInner,
    pieces_in_hand: [Vec<Piece>; 2],
    last_player: TwoPlayer,
    possible_moves_cache: Arc<Mutex<Option<Vec<Shogi55Move>>>>,
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Shogi55Place {
    col: usize,
    row: usize,
}

impl Shogi55Place {
    pub fn new(col: usize, row: usize) -> Shogi55Place {
        Shogi55Place { col, row }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
        self.col
    }
}

#[derive(Debug, Clone)]
pub struct Shogi55Move {
    player: TwoPlayer,
    in_hand: Option<Piece>,
    from: Option<Shogi55Place>,
    to: Shogi55Place,
    promotion_flag: bool,
}

impl Shogi55Move {
    pub fn new_with_from_to(
        player: &TwoPlayer,
        from: &Shogi55Place,
        to: &Shogi55Place,
        promotion_flag: bool,
    ) -> Self {
        Self {
            player: *player,
            in_hand: None,
            from: Some(*from),
            to: *to,
            promotion_flag,
        }
    }

    pub fn new_drop_to(player: &TwoPlayer, in_hand: Piece, to: &Shogi55Place) -> Self {
        Self {
            player: *player,
            in_hand: Some(in_hand),
            from: None,
            to: *to,
            promotion_flag: false,
        }
    }

    pub fn from_input(
        player: TwoPlayer,
        in_hand: Option<Piece>,
        from: Option<Shogi55Place>,
        to: Shogi55Place,
        promotion_flag: bool,
    ) -> Self {
        Shogi55Move {
            player,
            in_hand,
            from,
            to,
            promotion_flag,
        }
    }
}

impl Shogi55Board {
    pub fn init(&mut self) {
        let first = TwoPlayer::First;
        let second = TwoPlayer::Second;
        [
            (1, 1, second, Piece::King),
            (2, 1, second, Piece::Gold),
            (3, 1, second, Piece::Silver),
            (4, 1, second, Piece::Bishop),
            (5, 1, second, Piece::Rook),
            (1, 2, second, Piece::Pawn),
            (5, 5, first, Piece::King),
            (4, 5, first, Piece::Gold),
            (3, 5, first, Piece::Silver),
            (2, 5, first, Piece::Bishop),
            (1, 5, first, Piece::Rook),
            (5, 4, first, Piece::Pawn),
        ]
        .iter()
        .for_each(|(col, row, player, piece)| {
            let place = Shogi55Place::new(*col, *row);
            let piece_info = PieceInfo::new(player, piece);
            self.board_inner.placed_map_insert(place, piece_info);
        });
        self.init_all_possibilities();
    }

    pub fn get_piece_in_hand(&self) -> &[Vec<Piece>; 2] {
        &self.pieces_in_hand
    }

    pub fn get_placed_map(&self) -> &HashMap<Shogi55Place, PieceInfo> {
        self.board_inner.placed_map()
    }

    pub fn set_last_player_for_test(&mut self, player: TwoPlayer) {
        self.last_player = player;
    }

    pub fn check_input_from(&self, place: &Shogi55Place) -> Result<(), String> {
        let player = self.get_next_player();
        let placed = self.board_inner.placed_map().get(place);
        if let Some(piece_info) = placed {
            if piece_info.get_player() != &player {
                return Err("その駒は動かせません".into());
            }
        } else {
            return Err("そこからは動かせません".into());
        }
        Ok(())
    }

    pub fn get_place_map_len(&self) -> usize {
        self.board_inner.placed_map().len()
    }

    pub fn check_input_in_hand(
        &self,
        to_place: &Shogi55Place,
        piece: &Piece,
    ) -> Result<(), String> {
        let is_pawn = matches!(piece, Piece::Pawn);
        let player = self.get_next_player();
        let in_hand_to_places = self.get_all_place_from_in_hand(&player, is_pawn);
        if !in_hand_to_places.contains(to_place) {
            return Err("そこには打てません".into());
        }

        // 打ち歩詰め判定開始
        if is_pawn && self.board_inner.is_drop_pawn_mate(&player, to_place) {
            return Err("打ち歩詰めです".into());
        }

        // 王手されている時は合駒（Possible::Moveの precondition マスに打ち込む）でなければいけない
        let (is_checked, possibles) = self
            .board_inner
            .get_checking_with_precondition(&player, false);
        if !possibles.is_empty()
            && is_checked
            && let Possible::Move(ref precondition) = possibles[0].1
            && !precondition.contains(to_place)
        {
            return Err("玉が取られます".into());
        }
        Ok(())
    }

    pub fn check_input_from_to(
        &self,
        from_place: &Shogi55Place,
        to_place: &Shogi55Place,
    ) -> Result<(), String> {
        // 駒が置いていない時は check_input_from のせいにしてパニックさせる
        let piece_info = self
            .board_inner
            .placed_map()
            .get(from_place)
            .expect("駒がありません");
        if !piece_info
            .get_possibility()
            .get_possible(to_place)
            .can_move()
        {
            return Err("そこへは動かせません".into());
        }
        let (is_checked, possibles) = self
            .board_inner
            .get_checking_with_precondition(&self.get_next_player(), true);
        if is_checked {
            self.check_input_from_to_when_checked(from_place, to_place, piece_info, possibles)?;
        } else {
            // 王手されていない場合（ = Possible::Moveになっている相手の駒がない）
            // Possible::Blocked している駒を範囲外に動かす手が禁じられる
            if possibles.iter().any(|(target_place, possible)| {
                target_place != to_place && possible.is_blocking_over(from_place, to_place)
            }) {
                return Err("玉が取られます".into());
            }
        }
        Ok(())
    }

    // Deprecated
    fn check_input_from_to_when_checked(
        &self,
        from: &Shogi55Place,
        to: &Shogi55Place,
        piece_info: &PieceInfo,
        checked_possibles: Vec<(Shogi55Place, Possible)>,
    ) -> Result<(), String> {
        let piece = piece_info.get_piece();
        // 玉について、玉に動かせる手が残っている場合は合法手
        if matches!(piece, Piece::King) {
            let player = self.get_next_player();
            let possible_moves = self.board_inner.get_possible_moves_of_king(&player);
            return if !possible_moves.contains(to) {
                Err("玉が取られます".into())
            } else {
                Ok(())
            };
        }
        // 玉以外の駒について、
        // 動かしたことによりブロックが外れたら合法手ではない
        // 動かすことで王手している駒の進路を妨害できる、または王手している駒を取ることができれば合法手
        let moving_pieces = checked_possibles
            .iter()
            .filter(|(_, possible)| matches!(possible, Possible::Move(_)))
            .collect::<Vec<_>>();
        if moving_pieces.len() == 1 {
            let (place, _) = moving_pieces[0];
            let Possible::Move(ref precondition) = moving_pieces[0].1 else {
                // Move は必ず 長さ 0 以上の precondition を持つ
                panic!()
            };
            // 王手している駒を取っていない、かつ、王手している駒を遮っていない
            if place != to && !precondition.contains(to) {
                return Err("玉が取られます".into());
            }

            // 動かしたことによりブロックが外れたら合法手ではない
            // 動かすことで王手している駒の進路を妨害できる、または王手している駒を取ることができれば合法手
            if checked_possibles
                .iter()
                .any(|(place, possible)| place != to && possible.is_blocking_over(from, to))
            {
                return Err("玉が取られます".into());
            }
        } else {
            return Err("玉が取られます".into());
        }
        Ok(())
    }

    pub fn check_is_required_promotion_input(
        &mut self,
        from: &Shogi55Place,
        to: &Shogi55Place,
    ) -> Result<(bool, bool), String> {
        let test_from = Some(*from);
        let possible_moves = self.get_all_possible_moves();
        let target_moves = possible_moves
            .iter()
            .filter(|_move| _move.from == test_from && _move.to == *to)
            .collect::<Vec<_>>();
        if target_moves.is_empty() {
            Err("入力が不正です".into())
        } else if target_moves.len() == 1 {
            Ok((false, target_moves[0].promotion_flag))
        } else {
            Ok((true, false))
        }
    }
    pub fn safe_move(&mut self, shogi55_move: Shogi55Move) -> Result<(), String> {
        let player = shogi55_move.player;
        let to = shogi55_move.to;
        let mut arg = vec![&to];
        if shogi55_move.in_hand.is_some() {
            self.safe_move_in_hand(shogi55_move)?;
            self.update_possibility_by_places(arg);
        } else {
            let from = shogi55_move.from.unwrap();
            self.safe_move_placed_piece(shogi55_move)?;
            arg.push(&from);
            self.update_possibility_by_places(arg);
        }
        self.clear_possible_moves_cache();
        // self.last_player.next() を使ってもいいけど
        // テストが順序を無視して Shogi55Move を入れてくる場合を考慮して Shogi55Move からセットする
        self.last_player = player;
        Ok(())
    }

    fn safe_move_in_hand(&mut self, shogi55_move: Shogi55Move) -> Result<(), String> {
        let Shogi55Move {
            player,
            to,
            in_hand,
            ..
        } = shogi55_move;
        let piece = in_hand.unwrap();
        let piece_index = self.get_piece_index_in_hand(player, &piece)?;
        self.board_inner
            .placed_map_insert(to, PieceInfo::new(&player, &piece));
        self.remove_piece_in_hand_by_index(player, piece_index);
        Ok(())
    }

    fn safe_move_placed_piece(&mut self, shogi55_move: Shogi55Move) -> Result<(), String> {
        let Shogi55Move {
            player,
            from: Some(from),
            to,
            ..
        } = shogi55_move
        else {
            return Err("invalid input".into());
        };

        let mut piece_info = self
            .board_inner
            .placed_map_remove(&from)
            .expect("駒がありません。動かせませんでした");

        // 今ある駒を placed_pieces から削除し、持ち駒に加える
        if let Some(to_placed) = self.board_inner.placed_map_remove(&to) {
            if to_placed.get_player() == &player {
                panic!("自分の駒は取れません")
            } else {
                self.add_piece_in_hand(player, *to_placed.get_piece());
            }
        };
        if shogi55_move.promotion_flag {
            piece_info.promote();
        }
        self.board_inner.placed_map_insert(to, piece_info);
        Ok(())
    }

    fn add_piece_in_hand(&mut self, player: TwoPlayer, piece: Piece) {
        self.pieces_in_hand[player.get_index()].push(piece.captured())
    }

    fn get_piece_index_in_hand(
        &mut self,
        player: TwoPlayer,
        piece: &Piece,
    ) -> Result<usize, String> {
        let player_in_hand = &mut self.pieces_in_hand[player.get_index()];
        let Some(index) = player_in_hand.iter().position(|p| piece == p) else {
            return Err("piece not in hand".into());
        };
        Ok(index)
    }

    fn remove_piece_in_hand_by_index(&mut self, player: TwoPlayer, index: usize) {
        self.pieces_in_hand[player.get_index()].remove(index);
    }

    pub fn get_next_player(&self) -> TwoPlayer {
        self.last_player.next()
    }

    // 駒が打てる全てのマスを取得する
    // pawn_flag = true の場合は歩打ちとして二歩になるマスは除外する
    // 桂香には対応していない
    pub fn get_all_place_from_in_hand(
        &self,
        player: &TwoPlayer,
        pawn_flag: bool,
    ) -> Vec<Shogi55Place> {
        let mut possible_pawn_places = vec![];
        let mut blank_places = vec![];
        for col in 1..6 {
            let mut row_has_pawn_flag = false;
            let mut tmp_places = vec![];
            for row in 1..6 {
                if let Some(piece_info) = self
                    .board_inner
                    .placed_map()
                    .get(&Shogi55Place::new(col, row))
                {
                    if piece_info.get_piece() == &Piece::Pawn && piece_info.get_player() == player {
                        row_has_pawn_flag = true;
                    }
                } else {
                    let place = Shogi55Place::new(col, row);
                    blank_places.push(place);
                    if pawn_flag {
                        if row == 1 {
                            if *player != TwoPlayer::First {
                                tmp_places.push(place);
                            }
                        } else if row == 5 {
                            if *player != TwoPlayer::Second {
                                tmp_places.push(place);
                            }
                        } else {
                            tmp_places.push(place);
                        }
                    }
                }
            }
            if pawn_flag && !row_has_pawn_flag {
                possible_pawn_places.extend(tmp_places);
            }
        }
        if pawn_flag {
            possible_pawn_places
        } else {
            blank_places
        }
    }

    pub fn init_all_possibilities(&mut self) {
        let keys = self
            .board_inner
            .placed_map()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        keys.iter().for_each(|place| self.update_possibility(place));
    }

    // possibility を更新するまで Possible::Placed の形で元の場所が残されている
    // それを利用して更新対象駒を特定し、現在の場所を渡して update_possibility で更新をかける
    pub fn update_possibility_by_places(&mut self, places: Vec<&Shogi55Place>) {
        let target_places = self
            .board_inner
            .placed_map()
            .iter()
            .filter_map(|(place, piece_info)| {
                // ある駒が対象の場所に存在する、
                // あるいは対象の場所に対して Possible::Nothing以外の Possibility を持っている、
                // あるいはある駒の全ての Possible の precondition に対象を含めばその駒は Possibility 更新対象
                // なので駒の場所を保存
                if places.contains(&place)
                    || places.iter().any(|target_place| {
                        !matches!(
                            piece_info.get_possibility().get_possible(target_place),
                            Possible::Nothing
                        ) || piece_info
                            .get_possibility()
                            .precondition_contains(target_place)
                    })
                {
                    Some(*place)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        target_places
            .iter()
            .for_each(|place| self.update_possibility(place));
    }

    pub fn clear_possible_moves_cache(&mut self) {
        *self.possible_moves_cache.lock().unwrap() = None;
    }

    pub fn get_all_possible_moves(&self) -> Vec<Shogi55Move> {
        if let Some(cache) = self.possible_moves_cache.lock().unwrap().as_ref() {
            return cache.clone();
        }
        let player = self.last_player.next();
        let (king_place, _) = self.board_inner.get_king_info(&player);
        let mut moves = vec![];

        // 玉について、玉に動かせる手が残っている場合は合法手
        let possible_moves = self.board_inner.get_possible_moves_of_king(&player);
        possible_moves.iter().for_each(|place| {
            moves.push(Shogi55Move::new_with_from_to(
                &player,
                &king_place,
                place,
                false,
            ))
        });
        let (is_checked, _) = self
            .board_inner
            .get_checking_with_precondition(&player, true);
        moves.extend(if is_checked {
            self.get_all_possible_moves_on_checked(&player)
        } else {
            self.get_all_possible_moves_when_no_check(&player)
        });
        *self.possible_moves_cache.lock().unwrap() = Some(moves.clone());
        moves
    }

    fn get_eval_value(&self) -> i32 {
        let player = TwoPlayer::First;
        let mut point = 0;
        self.board_inner
            .placed_map()
            .values()
            .for_each(|piece_info| {
                point += piece_info.get_piece().point()
                    * if piece_info.get_player() == &player {
                        1
                    } else {
                        -1
                    };
            });
        self.pieces_in_hand[player.get_index()]
            .iter()
            .for_each(|piece| {
                point += piece.captured_point();
            });
        self.pieces_in_hand[player.next().get_index()]
            .iter()
            .for_each(|piece| {
                point -= piece.captured_point();
            });
        point
    }
    pub fn nest_search(&mut self, remain_nest_count: usize, rng: &mut SmallRng) -> (usize, i32) {
        let possible_moves = self.get_all_possible_moves();
        let is_first = self.last_player.next() == TwoPlayer::First;
        if possible_moves.is_empty() {
            return if is_first { (0, -10000) } else { (0, 10000) };
        }
        let mut best_point = if is_first { -10000 } else { 10000 };
        let mut best_move_index = 0;
        for (i, _move) in possible_moves.iter().enumerate() {
            let mut board_clone = self.clone();
            board_clone.safe_move(_move.clone()).unwrap();
            let point = if remain_nest_count == 0 {
                board_clone.get_eval_value()
            } else {
                let (_, nested_point) = board_clone.nest_search(remain_nest_count - 1, rng);
                nested_point
            };
            if if best_point == point {
                rng.random_bool(0.5)
            } else if is_first {
                best_point < point
            } else {
                best_point > point
            } {
                best_point = point;
                best_move_index = i;
            };
        }
        (best_move_index, best_point)
    }

    fn get_all_possible_moves_on_checked(&self, player: &TwoPlayer) -> Vec<Shogi55Move> {
        let mut moves = vec![];
        let (_, place_possibles) = self
            .board_inner
            .get_checking_with_precondition(player, true);
        let moving_pieces = place_possibles
            .iter()
            .filter(|(_, possible)| matches!(possible, Possible::Move(_)))
            .collect::<Vec<_>>();
        // 両王手の時は玉を動かす以外に合法手はない
        if moving_pieces.len() > 1 {
            return moves;
        }
        let (checking_piece_place, _) = moving_pieces[0];
        let Possible::Move(ref precondition) = moving_pieces[0].1 else {
            panic!()
        };

        // 盤上の駒の合法手（玉以外）
        self.board_inner
            .placed_pieces_of_player(player)
            .into_iter()
            .for_each(|(from_place, piece_info)| {
                let piece = piece_info.get_piece();
                // 玉以外の駒について
                if matches!(piece_info.get_piece(), Piece::King) {
                    return;
                }
                piece_info
                    .get_possibility()
                    .get_place_to_possible()
                    .iter()
                    .for_each(|(to_place, possible)| {
                        if !possible.can_move() // これ必要?
                            // 王手している駒を取っていない、かつ、王手している駒を遮っていない時は合法手でない
                            || (checking_piece_place != to_place && !precondition.contains(to_place))
                            // 動かしたことによりブロックが外れたら合法手ではない
                            || place_possibles.iter().any(|(_, possible)| {
                                possible.is_blocking_over(from_place, to_place)
                            })
                        {
                            return;
                        }
                        // 強制成でなければ不成を合法手に追加
                        if !piece.is_force_promotion(player, to_place) {
                            moves.push(Shogi55Move::new_with_from_to(
                                player, from_place, to_place, false,
                            ));
                        }
                        // 成駒の合法手
                        if piece.can_promote(player, from_place, to_place)
                        {
                            moves.push(Shogi55Move::new_with_from_to(
                                player, from_place, to_place, true,
                            ));
                        }
                    });
            });
        let pawn_drop_all_places = &self.get_all_place_from_in_hand(player, true);
        let not_pawn_drop_all_places = &self.get_all_place_from_in_hand(player, false);
        // 持ち駒の合法手（王手されている状態）
        self.pieces_in_hand[player.get_index()]
            .iter()
            .for_each(|piece| {
                let all_places = if matches!(piece, Piece::Pawn) {
                    pawn_drop_all_places
                } else {
                    not_pawn_drop_all_places
                };
                all_places.iter().for_each(|to_place| {
                    // 王手されている時は合駒（Possible::Moveの precondition マスに打ち込む）でなければいけない
                    if precondition.contains(to_place) {
                        moves.push(Shogi55Move::new_drop_to(player, *piece, to_place))
                    }
                });
            });
        moves
    }

    fn get_all_possible_moves_when_no_check(&self, player: &TwoPlayer) -> Vec<Shogi55Move> {
        let mut moves = vec![];

        let (_, placed_possibles) = self
            .board_inner
            .get_checking_with_precondition(player, true);
        let blocking_preconditions = placed_possibles
            .iter()
            .filter(|(_, possible)| matches!(possible, Possible::Blocked(_)))
            .collect::<Vec<_>>();
        // 盤上の駒の合法手（玉以外）
        self.board_inner
            .placed_pieces_of_player(player)
            .into_iter()
            .for_each(|(from_place, piece_info)| {
                let piece = piece_info.get_piece();
                if matches!(piece_info.get_piece(), Piece::King) {
                    return;
                }
                piece_info
                    .get_possibility()
                    .get_place_to_possible()
                    .iter()
                    .for_each(|(to_place, possible)| {
                        if !possible.can_move() {
                            return;
                        }
                        // 発生している全てのブロックについて、駒を動かすことでブロックが外れてはいけない
                        // (一つの駒は複数のブロックに関わることはない)
                        if blocking_preconditions
                            .iter()
                            .all(|(target_piece_place, possible)| {
                                target_piece_place == to_place
                                    || !possible.is_blocking_over(from_place, to_place)
                            })
                        {
                            // 強制成でなければ不成を合法手に追加
                            if !piece.is_force_promotion(player, to_place) {
                                moves.push(Shogi55Move::new_with_from_to(
                                    player, from_place, to_place, false,
                                ));
                            }
                            // 成駒の合法手
                            if piece.can_promote(player, from_place, to_place) {
                                moves.push(Shogi55Move::new_with_from_to(
                                    player, from_place, to_place, true,
                                ));
                            }
                        };
                    });
            });
        let pawn_drop_all_places = &self.get_all_place_from_in_hand(player, true);
        let not_pawn_drop_all_places = &self.get_all_place_from_in_hand(player, false);
        // 持ち駒の合法手（王手されていない状態）
        self.pieces_in_hand[player.get_index()]
            .iter()
            .for_each(|piece| {
                let all_places = if matches!(piece, Piece::Pawn) {
                    pawn_drop_all_places
                } else {
                    not_pawn_drop_all_places
                };
                all_places.iter().for_each(|to_place| {
                    moves.push(Shogi55Move::new_drop_to(player, *piece, to_place))
                });
            });
        moves
    }

    pub fn update_possibility(&mut self, place: &Shogi55Place) {
        let Some(piece_info) = &self.board_inner.placed_map().get(place) else {
            return;
        };
        let player = *piece_info.get_player();
        let mut possibility = Possibility::new();
        for direction in piece_info.get_piece().get_directions().into_iter() {
            let mut d = direction;
            let mut preconditions = vec![];
            let mut block_flag = false;
            if player == TwoPlayer::Second {
                d = d.inverse();
            }
            // 自分の場所を記憶しておく
            possibility.set_possible(place, Possible::Placed);

            while let Some(dist) = d.add_mod(place) {
                let possible = if let Some(piece_info) = self.board_inner.placed_map().get(&dist) {
                    if piece_info.get_player() == &player {
                        possibility.set_possible(&dist, Possible::Affect(preconditions.clone()));
                        preconditions.clear();
                        break;
                    } else if block_flag {
                        // Blocked の precondition はブロックの直接の原因以外も含んで複数になる
                        // precondition の中で動くとブロックし続ける
                        // precondition の外に動くとブロックが解除される
                        possibility.set_possible(&dist, Possible::Blocked(preconditions.clone()));
                        preconditions.clear();
                        break;
                    } else if piece_info.get_piece() == &Piece::King {
                        // 玉ではブロックできない（取られたら負けるからブロックとかではない）
                        let possible = Possible::Move(preconditions.clone());
                        preconditions.push(dist);
                        possible
                    } else {
                        let possible = Possible::Capture(preconditions.clone());
                        preconditions.push(dist);
                        block_flag = true;
                        possible
                    }
                } else {
                    let possible = if block_flag {
                        Possible::Blocked(preconditions.clone())
                    } else {
                        Possible::Move(preconditions.clone())
                    };
                    preconditions.push(dist);
                    possible
                };
                possibility.set_possible(&dist, possible);
                let Some(next) = d.next() else {
                    preconditions.clear();
                    break;
                };
                d = next;
            }
        }
        self.board_inner
            .placed_map_get_mut(place)
            .unwrap()
            .set_possibility(&possibility);
    }
}
