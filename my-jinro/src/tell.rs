use crate::Color::{Black, White};
use crate::{Color, Player, Role};

pub struct Tell {
    pub told_player: Option<Player>,
    pub tell_type: TellType,
    pub target_player: Option<Player>,
    pub target_role: Option<Role>,
    target_tell_index: Option<usize>,
    pub target_color: Option<Color>,
    pub is_valid: bool,
}

impl Tell {
    pub fn new_co(player_index: usize, role: Role) -> Tell {
        Tell {
            told_player: Some(Player::new(player_index)),
            tell_type: TellType::CO,
            target_player: None,
            target_role: Some(role),
            target_tell_index: None,
            target_color: None,
            is_valid: true,
        }
    }
    pub fn new_has_position(player_index: usize) -> Tell {
        Tell {
            told_player: Some(Player::new(player_index)),
            tell_type: TellType::HasPosition,
            target_player: None,
            target_role: None,
            target_tell_index: None,
            target_color: None,
            is_valid: true,
        }
    }
    pub fn new_fortune(player_index: usize, target_player_index: usize, color: Color) -> Tell {
        Tell {
            told_player: Some(Player::new(player_index)),
            tell_type: TellType::Fortune,
            target_player: Some(Player::new(target_player_index)),
            target_role: None,
            target_tell_index: None,
            target_color: Some(color),
            is_valid: true,
        }
    }
    pub fn new_see(player_index: usize, target_player_index: usize, color: Color) -> Tell {
        Tell {
            told_player: Some(Player::new(player_index)),
            tell_type: TellType::See,
            target_player: Some(Player::new(target_player_index)),
            target_role: None,
            target_tell_index: None,
            target_color: Some(color),
            is_valid: true,
        }
    }
    pub fn new_guard(player_index: usize, target_player_index: usize) -> Tell {
        Tell {
            told_player: Some(Player::new(player_index)),
            tell_type: TellType::Guard,
            target_player: Some(Player::new(target_player_index)),
            target_role: None,
            target_tell_index: None,
            target_color: None,
            is_valid: true,
        }
    }
    pub fn new_partner(player_index: usize, target_player_index: usize) -> Tell {
        Tell {
            told_player: Some(Player::new(player_index)),
            tell_type: TellType::Partner,
            target_player: Some(Player::new(target_player_index)),
            target_role: None,
            target_tell_index: None,
            target_color: None,
            is_valid: true,
        }
    }
    pub fn new_no_opposition(role: Role) -> Tell {
        Tell {
            told_player: None,
            tell_type: TellType::NoOpposition,
            target_player: None,
            target_role: Some(role),
            target_tell_index: None,
            target_color: None,
            is_valid: true,
        }
    }
    pub fn new_executed(target_player_index: usize) -> Tell {
        Tell {
            told_player: None,
            tell_type: TellType::Executed,
            target_player: Some(Player::new(target_player_index)),
            target_role: None,
            target_tell_index: None,
            target_color: None,
            is_valid: true,
        }
    }
    pub fn new_killed(target_player_index: usize) -> Tell {
        Tell {
            told_player: None,
            tell_type: TellType::Killed,
            target_player: Some(Player::new(target_player_index)),
            target_role: None,
            target_tell_index: None,
            target_color: None,
            is_valid: true,
        }
    }
    pub fn new_peace() -> Tell {
        Tell {
            told_player: None,
            tell_type: TellType::Peace,
            target_player: None,
            target_role: None,
            target_tell_index: None,
            target_color: None,
            is_valid: true,
        }
    }

    pub fn tell_type(&self) -> String {
        format!("{:?}", self.tell_type)
    }

    pub fn told_player(&self, player_names: Vec<String>) -> String {
        if let Some(told_player) = &self.told_player {
            player_names[told_player.index].to_owned()
        } else {
            "GM".to_owned()
        }
    }

    pub fn told_player_index(&self) -> usize {
        self.told_player.as_ref().unwrap().index
    }

    pub fn target_player_index(&self) -> usize {
        self.target_player.as_ref().unwrap().index
    }

    pub fn target_color(&self) -> isize {
        match self.target_color {
            Some(White) => 0,
            Some(Black) => 1,
            None => -1,
        }
    }

    pub fn disabled(&mut self) {
        self.is_valid = false;
    }
}

#[derive(Debug)]
pub enum TellType {
    CO,
    HasPosition,
    Fortune,
    See,
    Guard,
    Partner,
    NoOpposition,
    Withdrawal,
    Dawned,
    Executed,
    Killed,
    Peace,
}
