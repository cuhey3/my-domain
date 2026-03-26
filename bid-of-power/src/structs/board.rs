use crate::structs::input::{BattleChooseInput, BidInput, BoPInput, ItemUseInput};
use crate::structs::items::{Item, ItemDefinition};
use board_games::framework::TwoPlayer;

#[derive(Default, Clone)]
pub struct BoPBoard {
    current_bids: Vec<BidInput>,
    last_player: TwoPlayer,
    initiative: TwoPlayer,
    input_state: usize,
    is_ready_to_use_item: bool,
    player_info: [PlayerInfo; 2],
    // 現在出品中のアイテム一覧
    list_items: Vec<usize>,
    // これから出品されるアイテムの一覧
    display_items: Vec<usize>,
    winner: TwoPlayer,
    turn: usize,
}

impl BoPBoard {
    pub fn add_input(&mut self, input: &BoPInput) -> Result<(), String> {
        if let BoPInput::Bid(bid) = &input {
            self.current_bids.push(bid.clone());

            if self.is_bid_complete() {
                self.stock_item_from_list_items_using_complete_bids()?;

                self.current_bids.clear();

                if !self.is_ready_to_use_item && self.turn > 2 {
                    self.is_ready_to_use_item = true;
                }

                self.next_input_state();
            } else {
                self.last_player = *(input.get_player());
            }
        } else if let BoPInput::ItemUse(item_use) = &input {
            if let Some(stock_no) = item_use.get_stock_no() {
                let player = input.get_player();

                let player_index = player.get_index();

                let item_id = *self.player_info[player_index]
                    .stock_items
                    .get(stock_no - 1)
                    .unwrap() as u32;

                ItemDefinition::apply_item(item_id, self, player);

                self.player_info[player_index]
                    .stock_items
                    .remove(stock_no - 1);
            };
            if self.last_player == self.initiative {
                self.next_input_state();
            } else {
                self.last_player = *(input.get_player());
            }
        } else if let BoPInput::BattleChoose(battle_choose) = &input {
            let player = battle_choose.get_player();

            let player_index = player.get_index();

            let opponent_player_index = player.next().get_index();

            if battle_choose.get_battle_flag() {
                let own_attack_point = self.player_info[player_index].attack_point;

                let opponent_defence_point = self.player_info[opponent_player_index]
                    .defence_point
                    .min(own_attack_point - 1);

                let opponent_current_hp = self.player_info[opponent_player_index].current_hp;

                self.player_info[opponent_player_index].current_hp = opponent_current_hp
                    - (own_attack_point - opponent_defence_point).min(opponent_current_hp);
            } else {
                self.player_info[player_index].current_gold_amount += 1;
            }
            if self.last_player == self.initiative {
                self.next_input_state();
            } else {
                self.last_player = *(input.get_player());
            }
        } else if self.last_player == self.initiative {
            self.next_input_state();
        } else {
            self.last_player = *(input.get_player());
        }

        Ok(())
    }

    pub fn stock_item_from_list_items_using_complete_bids(&mut self) -> Result<(), String> {
        let [.., second_last_bid, last_bid] = self.current_bids.as_slice() else {
            return Err("current bids state invalid".into());
        };

        let mut valid_bids = [second_last_bid, last_bid];

        valid_bids.sort_by_key(|bid| -(bid.get_list_no() as isize));

        valid_bids.iter().for_each(|bid| {
            let player_index = bid.get_player().get_index();

            let list_item_index = bid.get_list_no() - 1;

            let item_id = self.list_items[list_item_index];

            self.player_info[player_index].stock_items.push(item_id);

            self.player_info[player_index].current_gold_amount -= bid.get_amount();

            self.list_items.remove(list_item_index);
        });

        if second_last_bid.get_amount() > last_bid.get_amount() {
            self.initiative = *second_last_bid.get_player();
        } else if last_bid.get_amount() > second_last_bid.get_amount() {
            self.initiative = *last_bid.get_player();
        } else {
            self.initiative = self.initiative.next();
        }

        Ok(())
    }

    pub fn supply_list_items_from_display_items(&mut self) {
        let mut list_items_len = self.list_items.len();

        while list_items_len < 3 {
            let Some(item_id) = self.display_items.first() else {
                return;
            };

            self.list_items.push(*item_id);

            self.display_items.remove(0);

            list_items_len += 1;
        }
    }

    // プレイヤー入力用のバリデーション（CPUには使わない）
    // テストなどで BoPInput を投入する時にも使用する
    pub fn validate_player_input(&mut self, input: &BoPInput) -> Result<(), String> {
        match input {
            // 最初の二つはシステム用
            BoPInput::Nothing | BoPInput::DisplayItem(_) => return Err("今は入力できません".into()),
            BoPInput::Bid(input) => self.validate_bid_input(input)?,
            BoPInput::ItemUse(input) => self.validate_item_use_input(input)?,
            BoPInput::BattleChoose(input) => self.validate_battle_choose_input(input)?,
        };
        Ok(())
    }

    pub fn has_winner(&self) -> bool {
        false
    }

    pub fn get_next_input(&self) -> BoPInput {
        match self.input_state {
            0 => {
                let mut input = BoPInput::Bid(BidInput::default());

                input.set_player(&self.get_next_player_with_initiative());

                input
            }
            1 => {
                let mut input = BoPInput::ItemUse(ItemUseInput::default());

                input.set_player(&self.get_next_player_with_initiative());

                input
            }
            2 => {
                let mut input = BoPInput::BattleChoose(BattleChooseInput::default());

                input.set_player(&self.get_next_player_with_initiative());

                input
            }
            _ => panic!(),
        }
    }

    pub fn is_first_player_turn(&self) -> bool {
        self.get_next_player_with_initiative() == TwoPlayer::First
    }

    pub fn get_next_player_with_initiative(&self) -> TwoPlayer {
        if self.last_player == TwoPlayer::None {
            self.initiative
        } else {
            self.last_player.next()
        }
    }

    pub fn init(&mut self) {
        self.initiative = TwoPlayer::First;

        self.turn = 1;

        for i in 0..2 {
            self.player_info[i].current_gold_amount = 5;
            self.player_info[i].estimated_gold_amount = 3;
            self.player_info[i].max_hp = 50;
            self.player_info[i].current_hp = 50;
            self.player_info[i].attack_point = 10;
            self.player_info[i].defence_point = 5;
        }

        self.list_items.push(1);
        self.list_items.push(2);
        self.list_items.push(3);

        self.display_items.push(4);
        self.display_items.push(5);
        self.display_items.push(6);
        self.display_items.push(7);
        self.display_items.push(8);
        self.display_items.push(9);
        self.display_items.push(10);
        self.display_items.push(11);
        self.display_items.push(12);
    }

    fn is_bid_complete(&self) -> bool {
        let current_bids_len = self.current_bids.len();

        if self.input_state != 0 || current_bids_len < 2 {
            return false;
        }

        let second_last_bid = self.current_bids.get(current_bids_len - 2).unwrap();

        let last_bid = self.current_bids.last().unwrap();

        second_last_bid.get_list_no() != last_bid.get_list_no()
    }

    fn next_input_state(&mut self) {
        self.input_state += 1;

        self.input_state %= if self.is_ready_to_use_item { 3 } else { 1 };

        if self.input_state == 0 {
            self.next_turn_logic();
        }

        self.last_player = TwoPlayer::None;
    }

    fn next_turn_logic(&mut self) {
        self.turn += 1;

        self.player_info[0].current_gold_amount += self.player_info[0].estimated_gold_amount;

        self.player_info[1].current_gold_amount += self.player_info[1].estimated_gold_amount;

        self.supply_list_items_from_display_items();
    }

    // プレイヤー入力用のバリデーション（CPUには使わない）
    fn validate_bid_input(&self, input: &BidInput) -> Result<(), String> {
        if self.input_state != 0 || &self.get_next_player_with_initiative() != input.get_player() {
            return Err("今は入力できません".into());
        }

        let list_items_len = self.list_items.len();

        if input.get_list_no() >= list_items_len {
            return Err(format!("Noは 1-{list_items_len} の範囲で入力してください"));
        }

        let input_amount = input.get_amount();

        let current_gold_amount =
            self.player_info[input.get_player().get_index()].current_gold_amount;

        if input_amount == 0 {
            return Err("価格は1以上を入力してください".into());
        }

        if input_amount > current_gold_amount {
            return Err(format!(
                "Gold が足りません(所持: {current_gold_amount} 入札: {input_amount})"
            ));
        }

        if let Some(last) = self.current_bids.last() {
            let last_amount = last.get_amount();

            if last.get_list_no() == input.get_list_no() && input_amount <= last_amount + 1 {
                return Err(format!(
                    "現在価格より2以上多く入札してください(現在価格: {last_amount} 入札: {input_amount})"
                ));
            }
        }

        Ok(())
    }

    // プレイヤー入力用のバリデーション（CPUには使わない）
    fn validate_item_use_input(&self, input: &ItemUseInput) -> Result<(), String> {
        if self.input_state != 1
            || !self.is_ready_to_use_item
            || &self.get_next_player_with_initiative() != input.get_player()
        {
            return Err("今は入力できません".into());
        }
        if let Some(stock_no) = input.get_stock_no() {
            let len = self.player_info[input.get_player().get_index()]
                .stock_items
                .len();

            if len <= stock_no {
                return Err(format!(
                    "使用するアイテムは 1-{len} の範囲で入力してください"
                ));
            }
        }

        Ok(())
    }

    // プレイヤー入力用のバリデーション（CPUには使わない）
    fn validate_battle_choose_input(&self, input: &BattleChooseInput) -> Result<(), String> {
        if self.input_state != 2
            || !self.is_ready_to_use_item
            || &self.get_next_player_with_initiative() != input.get_player()
        {
            return Err("今は入力できません".into());
        }
        Ok(())
    }

    pub fn get_player_infos(&mut self) -> &mut [PlayerInfo; 2] {
        &mut self.player_info
    }
    pub fn set_winner(&mut self, winner: TwoPlayer) {
        self.winner = winner;
    }

    pub fn get_list_items(&self) -> Vec<Item> {
        self.list_items
            .iter()
            .map(|item_id| Item::from(*item_id as u32))
            .collect::<Vec<Item>>()
    }

    pub fn get_current_bids(&self) -> Vec<BidInput> {
        self.current_bids.clone()
    }

    pub fn get_player_info(&self) -> (PlayerInfo, PlayerInfo) {
        (self.player_info[0].clone(), self.player_info[1].clone())
    }
}

#[derive(Default, Clone, Debug)]
pub struct PlayerInfo {
    max_hp: u32,
    current_hp: u32,
    attack_point: u32,
    defence_point: u32,
    current_gold_amount: u32,
    estimated_gold_amount: u32,
    stock_items: Vec<usize>,
}

pub enum Status {
    MaxHp,
    CurrentHp,
    Attack,
    Defence,
    CurrentGold,
    EstimatedGold,
}

impl PlayerInfo {
    pub fn add_amount(&mut self, status: Status, amount: u32) {
        match status {
            Status::MaxHp => self.max_hp += amount,
            Status::CurrentHp => self.current_hp = (self.current_hp + amount).min(self.max_hp),
            Status::Attack => self.attack_point += amount,
            Status::Defence => self.defence_point += amount,
            Status::CurrentGold => self.current_gold_amount += amount,
            Status::EstimatedGold => self.estimated_gold_amount += amount,
        }
    }

    pub fn golden_add_amount(&mut self, status: Status, scale: u32) {
        let amount = self.current_gold_amount * scale;

        self.add_amount(status, amount);
    }

    pub fn balance(&mut self) {
        let amount = self.attack_point.max(self.defence_point) + 1;

        self.attack_point = amount;

        self.defence_point = amount;
    }

    pub fn shrink(&mut self) {
        let amount = self.attack_point.min(self.defence_point).max(1) - 1;

        self.attack_point = amount;

        self.defence_point = amount;
    }

    pub fn subtract_amount(&mut self, status: Status, amount: u32) {
        match status {
            Status::MaxHp => {
                self.max_hp -= self.max_hp.min(amount);

                self.current_hp = self.current_hp.min(self.max_hp);
            }
            Status::CurrentHp => self.current_hp -= amount.min(self.current_hp),
            Status::Attack => self.attack_point -= amount.min(self.attack_point),
            Status::Defence => self.defence_point -= amount.min(self.defence_point),
            Status::CurrentGold => self.current_gold_amount -= amount.min(self.current_gold_amount),
            Status::EstimatedGold => {
                self.estimated_gold_amount -= amount.min(self.estimated_gold_amount)
            }
        }
    }

    pub fn cut_status(&mut self, status: Status) {
        match status {
            Status::MaxHp => {
                self.max_hp /= 2;

                self.current_hp = self.current_hp.min(self.max_hp);
            }
            Status::CurrentHp => self.current_hp /= 2,
            Status::Attack => self.attack_point /= 2,
            Status::Defence => self.defence_point /= 2,
            Status::CurrentGold => self.current_gold_amount /= 2,
            Status::EstimatedGold => self.estimated_gold_amount /= 2,
        }
    }

    pub fn swap_max_hp_current_hp(&mut self, opponent: &mut PlayerInfo) {
        std::mem::swap(&mut self.max_hp, &mut opponent.max_hp);
        std::mem::swap(&mut self.current_hp, &mut opponent.current_hp);
    }

    pub fn swap_status(&mut self, opponent: &mut PlayerInfo, status: Status) {
        match status {
            Status::MaxHp => self.swap_max_hp_current_hp(opponent),
            Status::CurrentHp => self.swap_max_hp_current_hp(opponent),
            Status::Attack => {
                std::mem::swap(&mut self.attack_point, &mut opponent.attack_point);
            }
            Status::Defence => {
                std::mem::swap(&mut self.defence_point, &mut opponent.defence_point);
            }
            Status::CurrentGold => {
                std::mem::swap(
                    &mut self.current_gold_amount,
                    &mut opponent.current_gold_amount,
                );
            }
            Status::EstimatedGold => {
                std::mem::swap(
                    &mut self.estimated_gold_amount,
                    &mut opponent.estimated_gold_amount,
                );
            }
        }
    }

    pub fn get_status_amount(&self, status: Status) -> u32 {
        match status {
            Status::MaxHp => self.max_hp,
            Status::CurrentHp => self.current_hp,
            Status::Attack => self.attack_point,
            Status::Defence => self.defence_point,
            Status::CurrentGold => self.current_gold_amount,
            Status::EstimatedGold => self.estimated_gold_amount,
        }
    }
}
