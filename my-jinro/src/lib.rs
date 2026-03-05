mod tell;
pub mod village;

use crate::Color::{Black, White};
use crate::Role::*;
use crate::tell::Tell;

pub struct Player {
    pub index: usize,
}

impl Player {
    pub fn new(player_index: usize) -> Self {
        Self {
            index: player_index,
        }
    }
}
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn get_index(&self) -> usize {
        match self {
            White => 0,
            Black => 1,
        }
    }
}

pub enum NgReason {
    CoDifferentRole,
    CoHasPosition,
    CoNoPosition,
    CoNonHuman,
    FortuneTellerView,
    MediumView,
    GuardView,
    PartnerView,
    HasOpposition,
    HunterDeadAllBeforePeace,
    KilledIsNotWolf,
    WolfSaturation,
    ShouldCoWhenAllExposure,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Role {
    Villager,
    FortuneTeller,
    Medium,
    Hunter,
    Wolf,
    Maniac,
    Mason,
    GM,
}

impl Role {
    pub fn get_max_index() -> usize {
        7
    }
    pub fn from_index(index: usize) -> Role {
        match index {
            0 => Villager,
            1 => FortuneTeller,
            2 => Medium,
            3 => Hunter,
            4 => Wolf,
            5 => Maniac,
            6 => Mason,
            7 => GM,
            _ => panic!(),
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            Villager => 0,
            FortuneTeller => 1,
            Medium => 2,
            Hunter => 3,
            Wolf => 4,
            Maniac => 5,
            Mason => 6,
            GM => 7,
        }
    }

    pub fn is_position(&self) -> bool {
        matches!(self, FortuneTeller | Medium | Mason | Hunter)
    }
    pub fn is_should_co_position(&self) -> bool {
        matches!(self, FortuneTeller | Medium | Mason)
    }
    pub fn is_non_human(&self) -> bool {
        matches!(self, Wolf | Maniac)
    }
    pub fn get_color(&self) -> Color {
        match self {
            Wolf => Black,
            _ => White,
        }
    }

    // 実際の役職とCOの役職が異なっていてよいか
    // 村人の役職騙りはNG
    // 役職の役職騙りは撤回が前提であるので撤回前はその可能性を消さなくてはいけない（ので、NGとする）
    pub fn is_allowed_co(&self, other: &Self) -> bool {
        // 実際の役職と同じものはCOできる
        if self == other {
            return true;
        }

        // 人外ならば何でもCOできる
        if self.is_non_human() {
            return true;
        }
        false
    }
}

pub struct Breakdown<'a> {
    pub dead_events: Vec<&'a Tell>,
    pub co_events: Vec<&'a Tell>,
    pub wolf_indexes: Vec<usize>,
}

impl<'a> Breakdown<'a> {
    pub fn new() -> Breakdown<'a> {
        Breakdown {
            dead_events: vec![],
            co_events: vec![],
            wolf_indexes: vec![],
        }
    }
}

#[derive(Debug)]
pub struct IteratorConstraint {
    ng_matrix: Vec<Vec<bool>>,
}

impl IteratorConstraint {
    pub fn new(player_len: usize, role_max: usize) -> Self {
        Self {
            ng_matrix: vec![vec![false; role_max + 1]; player_len],
        }
    }
    pub fn add(&mut self, player_index: usize, role_index: usize) {
        self.ng_matrix[player_index][role_index] = true;
    }
    pub fn remove(&mut self, player_index: usize, role_index: usize) {
        self.ng_matrix[player_index][role_index] = false;
    }
    pub fn is_ng(&self, player_index: usize, role_index: usize) -> bool {
        self.ng_matrix[player_index][role_index]
    }
}

pub struct RoleIterator<'a> {
    pub role_list: &'a mut Vec<Role>,
    pub constraint: IteratorConstraint,
    pub last_increment_index: usize,
    pub last_ng_index: Option<usize>,
    pub last_ng_value: usize,
    length: usize,
}

impl<'a> RoleIterator<'a> {
    pub fn new_with_constraint(
        role_list: &mut Vec<Role>,
        constraint: IteratorConstraint,
    ) -> RoleIterator<'_> {
        role_list.sort_by_key(Role::get_index);
        let mut iter = RoleIterator {
            length: role_list.len(),
            last_increment_index: role_list.len() - 1,
            last_ng_index: None,
            role_list,
            constraint,
            last_ng_value: 0,
        };
        iter.ready();
        iter
    }

    pub fn ready(&mut self) -> Option<()> {
        'outer: loop {
            for i in 0..self.length {
                let last_increment_value = self.role_list[i].get_index();
                if self.constraint.is_ng(i, last_increment_value) {
                    self.last_ng_index = Some(i);
                    self.last_ng_value = last_increment_value;
                    if self.next().is_none() {
                        return None;
                    } else {
                        continue 'outer;
                    };
                }
            }
            return Some(());
        }
    }
}

impl RoleIterator<'_> {
    pub fn next_with_constraint(&mut self) -> Option<()> {
        self.next()?;
        self.ready()
        // loop {
        //     return match self.next() {
        //         Some(_) => {
        //             if self.last_ng_index.is_some()
        //                 && self.last_ng_value
        //                     == self.role_list[self.last_ng_index.unwrap()].get_index()
        //             {
        //                 continue;
        //             } else {
        //                 self.last_ng_index = None;
        //             }
        //             let last_increment_value =
        //                 self.role_list[self.last_increment_index].get_index();
        //             if self
        //                 .constraint
        //                 .is_ng(self.last_increment_index, last_increment_value)
        //             {
        //                 self.last_ng_index = Some(self.last_increment_index);
        //                 self.last_ng_value = last_increment_value;
        //                 continue;
        //             }
        //             Some(())
        //         }
        //         None => None,
        //     };
        // }
    }
}

impl<'a> Iterator for RoleIterator<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let mut to_index;
        for from_index in (1..self.length).rev() {
            to_index = from_index - 1;
            let from_value = self.role_list[from_index].get_index();
            let to_value = self.role_list[to_index].get_index();
            let mut replace_index = from_index;
            if from_value > to_value {
                for i in (from_index..self.length).rev() {
                    let value = self.role_list[i].get_index();
                    if value <= from_value && to_value < value {
                        replace_index = i;
                        break;
                    }
                }
                self.role_list.swap(to_index, replace_index);
                self.last_increment_index = to_index;
                let swap_count = (self.length - 1 - to_index) / 2;
                for i in 0..swap_count {
                    self.role_list.swap(self.length - 1 - i, to_index + 1 + i);
                }
                return Some(());
            }
        }
        None
    }
}
