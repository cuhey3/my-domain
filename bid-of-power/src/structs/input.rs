use board_games::convert_input;
use board_games::framework::TwoPlayer;

#[derive(Default, Debug, Clone)]
pub enum BoPInput {
    #[default]
    Nothing,
    DisplayItem(DisplayItemInput),
    Bid(BidInput),
    ItemUse(ItemUseInput),
    BattleChoose(BattleChooseInput),
}

#[derive(Default, Debug, Clone)]
pub enum Confirm {
    #[default]
    BeforeInput,
    Confirming,
    Confirmed,
}

impl BoPInput {
    pub fn get_player(&self) -> &TwoPlayer {
        match &self {
            BoPInput::Nothing => &TwoPlayer::None,
            BoPInput::DisplayItem(_) => &TwoPlayer::None,
            BoPInput::Bid(input) => &input.player,
            BoPInput::ItemUse(input) => &input.player,
            BoPInput::BattleChoose(input) => &input.player,
        }
    }

    pub fn has_confirm(&self) -> bool {
        match self {
            BoPInput::Nothing => false,
            BoPInput::DisplayItem(_) => false,
            BoPInput::Bid(input) => matches!(input.confirm, Confirm::Confirming),
            BoPInput::ItemUse(input) => matches!(input.confirm, Confirm::Confirming),
            BoPInput::BattleChoose(_) => false,
        }
    }

    // 入力がやり直しになったら Input を default に戻す
    pub fn confirm(&mut self, answer: &str) {
        if matches!(
            self,
            BoPInput::Nothing | BoPInput::DisplayItem(_) | BoPInput::BattleChoose(_)
        ) {
            return;
        }
        match answer {
            "c" | "n" => match self {
                BoPInput::Bid(input) => *input = BidInput::default(),
                BoPInput::ItemUse(input) => *input = ItemUseInput::default(),
                _ => panic!(),
            },
            _ => match self {
                BoPInput::Bid(input) => input.confirm = Confirm::Confirmed,
                BoPInput::ItemUse(input) => input.confirm = Confirm::Confirmed,
                _ => panic!(),
            },
        }
    }

    pub fn answer_to_input(&mut self, answer: &String) -> Result<(), String> {
        match self {
            Self::Bid(input) => {
                let mut split = answer.split(",");
                if let Some(list_no_string) = split.next()
                    && let Some(amount_string) = split.next()
                {
                    let list_no = convert_input::simple_parse(list_no_string)?;
                    let amount = convert_input::simple_parse(amount_string)?;
                    input.list_no = list_no;
                    input.amount = amount;
                    input.confirm = Confirm::Confirming;
                } else {
                    return Err(format!("入力が不正です: {}", answer));
                };
                Ok(())
            }
            Self::ItemUse(input) => {
                let stock_no = convert_input::accept_empty_parse(answer)?;
                input.stock_no = stock_no;
                input.confirm = Confirm::Confirming;
                Ok(())
            }
            Self::BattleChoose(input) => {
                let battle_flag = convert_input::yes_or_no(answer)?;
                input.battle_flag = battle_flag;
                Ok(())
            }
            _ => panic!(),
        }
    }

    pub fn set_player(&mut self, player: &TwoPlayer) {
        match self {
            BoPInput::Bid(input) => input.player = *player,
            BoPInput::ItemUse(input) => input.player = *player,
            BoPInput::BattleChoose(input) => input.player = *player,
            _ => panic!(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct DisplayItemInput {
    item_ids: Vec<u64>,
    seed: u64,
}

#[derive(Default, Debug, Clone)]
pub struct BidInput {
    player: TwoPlayer,
    list_no: usize,
    amount: u32,
    confirm: Confirm,
}

#[derive(Default, Debug, Clone)]
pub struct ItemUseInput {
    player: TwoPlayer,
    stock_no: Option<usize>,
    confirm: Confirm,
}

#[derive(Default, Debug, Clone)]
pub struct BattleChooseInput {
    player: TwoPlayer,
    battle_flag: bool,
}

impl BidInput {
    pub fn get_list_no(&self) -> usize {
        self.list_no
    }
    pub fn get_player(&self) -> &TwoPlayer {
        &self.player
    }
    pub fn get_amount(&self) -> u32 {
        self.amount
    }
}

impl ItemUseInput {
    pub fn get_player(&self) -> &TwoPlayer {
        &self.player
    }

    pub fn get_stock_no(&self) -> Option<usize> {
        self.stock_no
    }
}

impl BattleChooseInput {
    pub fn get_player(&self) -> &TwoPlayer {
        &self.player
    }
}
