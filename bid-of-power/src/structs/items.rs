use crate::structs::board::{BoPBoard, Status};
use board_games::framework::TwoPlayer;

pub struct Item {
    id: u32,
    name: String,
    description: String,
}

impl Item {
    pub fn from(id: u32) -> Item {
        let (name, description) = ItemDefinition::get_name_description(id);
        Item {
            id,
            name,
            description,
        }
    }
}

pub struct ItemDefinition {}

impl ItemDefinition {
    pub fn apply_item(id: u32, board: &mut BoPBoard, player: &TwoPlayer) {
        let [first_info, second_info] = board.get_player_infos();
        let (own_info, opponent_info) = if player == &TwoPlayer::First {
            (first_info, second_info)
        } else {
            (second_info, first_info)
        };
        let mut winner = TwoPlayer::None;
        match id {
            0 => own_info.add_amount(Status::Attack, 5),
            1 => own_info.add_amount(Status::Attack, 10),
            2 => own_info.add_amount(Status::Defence, 5),
            3 => own_info.add_amount(Status::Defence, 10),
            4 => own_info.add_amount(Status::CurrentHp, 20),
            5 => {
                own_info.add_amount(Status::MaxHp, 10);
                own_info.add_amount(Status::CurrentHp, 10);
            }
            6 => own_info.add_amount(Status::CurrentGold, 5),
            7 => own_info.add_amount(Status::EstimatedGold, 1),
            8 => {
                own_info.add_amount(Status::CurrentHp, 10);
                own_info.add_amount(Status::Attack, 10);
                own_info.add_amount(Status::Defence, 10);
            }
            9 => {
                own_info.subtract_amount(Status::CurrentHp, 5);
                own_info.add_amount(Status::Attack, 5);
                own_info.subtract_amount(Status::Defence, 5);
                opponent_info.subtract_amount(Status::CurrentHp, 5);
                opponent_info.add_amount(Status::Attack, 5);
                opponent_info.subtract_amount(Status::Defence, 5);
                if own_info.get_status_amount(Status::CurrentHp) == 0 {
                    winner = player.next();
                } else if opponent_info.get_status_amount(Status::CurrentHp) == 0 {
                    winner = *player;
                }
            }
            10 => {
                opponent_info.subtract_amount(Status::CurrentHp, 15);
                if opponent_info.get_status_amount(Status::CurrentHp) == 0 {
                    winner = *player;
                }
            }
            11 => own_info.swap_max_hp_current_hp(opponent_info),
            12 => own_info.swap_status(opponent_info, Status::Attack),
            13 => own_info.swap_status(opponent_info, Status::Defence),
            14 => opponent_info.cut_status(Status::Attack),
            15 => opponent_info.cut_status(Status::Defence),
            16 => own_info.golden_add_amount(Status::MaxHp, 2),
            17 => own_info.golden_add_amount(Status::Attack, 1),
            18 => own_info.golden_add_amount(Status::Defence, 1),
            19 => own_info.balance(),
            20 => opponent_info.shrink(),
            _ => panic!(),
        }
        if winner != TwoPlayer::None {
            board.set_winner(winner);
        }
    }
    pub fn get_name_description(id: u32) -> (String, String) {
        let (name, description) = match id {
            0 => ("ダガー", "自己ATK+5"),
            1 => ("ロングソード", "自己ATK+10"),
            2 => ("レザーアーマー", "自己DEF+5"),
            3 => ("チェインメイル", "自己DEF+10"),
            4 => ("キュア", "自己HP+20"),
            5 => ("ビルドアップ", "自己MHP+10,HP+10"),
            6 => ("トレジャー", "自己Gold+5"),
            7 => ("ゲインアップ", "自己獲得Gold+1"),
            8 => ("エクスカリバー", "自己HP+10,ATK+10,DEF+10"),
            9 => ("カオス", "全員HP-5,ATK+5,DEF-5"),
            10 => ("マジックボルト", "相手HP-15"),
            11 => ("HPスワップ", "お互いのMHP,HPを入れ替える"),
            12 => ("ATKスワップ", "お互いのATKを入れ替える"),
            13 => ("DEFスワップ", "お互いのDEFを入れ替える"),
            14 => ("ウィークネス", "相手ATK半減"),
            15 => ("アーマーブレイク", "相手DEF半減"),
            16 => ("ゴールデンヒール", "自己HP+自己現在Gold×2"),
            17 => ("ゴールデンダガー", "自己ATK+自己現在Gold"),
            18 => ("ゴールデンスキン", "自己DEF+自己現在Gold"),
            19 => ("バランス", "自己ATK,DEFを高い方に合わせ+1"),
            20 => ("シュリンク", "相手ATK,DEFを低い方に合わせ-1"),
            _ => panic!(),
        };
        (name.to_string(), description.to_string())
    }
}
