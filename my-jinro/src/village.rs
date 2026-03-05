use crate::NgReason::{
    CoDifferentRole, CoHasPosition, CoNoPosition, CoNonHuman, FortuneTellerView, GuardView,
    HasOpposition, HunterDeadAllBeforePeace, KilledIsNotWolf, MediumView, PartnerView,
    WolfSaturation,
};
use crate::Role::{FortuneTeller, Hunter, Maniac, Mason, Medium, Villager, Wolf};
use crate::tell::TellType;
use crate::{Breakdown, Color, IteratorConstraint, NgReason, Player, Role, RoleIterator, Tell};
use std::cmp;
use std::collections::{HashMap, HashSet};

struct Constants {
    non_human_count_of_rule: usize,
    should_co_position_count_of_rule: usize,
    player_count: usize,
    max_role_index: usize,
    flag: Flag,
}

struct Flag {
    has_hunter: bool,
}

pub struct Village<'a> {
    pub role_list: Vec<Role>,
    pub player_list: Vec<Player>,
    pub tell_list: Vec<Tell>,
    pub player_names: Vec<String>,
    pub ok_roles: Vec<Vec<Role>>,
    ng_roles: Vec<Vec<Role>>,
    constants: Constants,
    pub available_role_list: Vec<Vec<Role>>,
    pub breakdown: Breakdown<'a>,
}

impl<'a> Village<'a> {
    pub fn new() -> Village<'a> {
        Village {
            role_list: vec![],
            player_list: vec![],
            tell_list: vec![],
            player_names: vec![],
            ok_roles: vec![],
            ng_roles: vec![],
            constants: Constants {
                non_human_count_of_rule: 0,
                should_co_position_count_of_rule: 0,
                player_count: 0,
                max_role_index: 0,
                flag: Flag { has_hunter: false },
            },
            available_role_list: vec![],
            breakdown: Breakdown::new(),
        }
    }
    //
    // pub fn dead_events(&self) -> &Vec<Tell> {
    //     &self.breakdown.dead_events
    // }

    // pub fn co_events(&self) -> &Vec<&Tell> {
    //     &self.breakdown.co_events
    // }
    // COしていてかつ自分が村陣営の内訳が存在しない場合、破綻(failure)
    // pub fn is_failure(&self, player_index: usize) -> bool {
    //     let found = self
    //         .breakdown
    //         .co_events
    //         .iter()
    //         .find(|tell| match &tell.tell_type {
    //             TellType::CO => tell.is_valid && (player_index == tell.told_player_index()),
    //             _ => false,
    //         });
    //     if found.is_none() {
    //         false
    //     } else {
    //         let found = self
    //             .ok_roles
    //             .iter()
    //             .find(|roles| !roles[player_index].is_non_human());
    //         found.is_none()
    //     }
    // }

    pub fn wolf_indexes(&self) -> &Vec<usize> {
        &self.breakdown.wolf_indexes
    }

    pub fn set_role_list_by_index(&mut self, role_indexes: Vec<usize>) {
        let mut role_list = vec![];
        let mut non_human_count_of_rule = 0;
        let mut should_co_position_count_of_rule = 0;
        let mut has_hunter = false;
        let mut max_role_index = 0;
        for index in role_indexes {
            let role = Role::from_index(index);
            if role.is_non_human() {
                non_human_count_of_rule += 1;
            }
            if role.is_should_co_position() {
                should_co_position_count_of_rule += 1;
            }
            if matches!(role, Hunter) {
                has_hunter = true;
            }
            max_role_index = max_role_index.max(index);
            role_list.push(role);
        }
        self.role_list = role_list;
        self.constants = Constants {
            non_human_count_of_rule,
            should_co_position_count_of_rule,
            player_count: self.role_list.len(),
            max_role_index,
            flag: Flag { has_hunter },
        }
    }
    pub fn add_player(&mut self, name: String) {
        self.player_names.push(name);
        self.player_list.push(Player {
            index: self.player_list.len(),
        });
    }
    pub fn edit_player(&mut self, index: usize, name: String) {
        self.player_names[index] = name;
    }

    pub fn disable_tell(&mut self, index: usize) {
        self.tell_list[index].is_valid = false
    }
    pub fn get_available_roles(&self, player_index: usize) -> Option<&Vec<Role>> {
        self.available_role_list.get(player_index)
    }
    // pub fn get_available_roles_on_player_view(
    //     &self,
    //     view_player_index: usize,
    //     target_player_index: usize,
    // ) -> Vec<Role> {
    //     let target_player_role_set: HashSet<Role> = self
    //         .ok_roles
    //         .iter()
    //         .filter(|roles| !roles[view_player_index].is_non_human())
    //         .map(|roles| roles[target_player_index])
    //         .collect();
    //     let mut target_player_roles = Vec::from_iter(target_player_role_set);
    //     target_player_roles.sort();
    //     target_player_roles
    // }
    pub fn get_wolf_accuracy(&self, player_index: usize, scale: f64, maniac_ratio: f64) -> f64 {
        let ok_roles_count = self.ok_roles.len();
        if ok_roles_count == 0 {
            return -1.0;
        }
        let ok_roles_count = ok_roles_count as f64;
        let ratio_sum: f64 = self
            .ok_roles
            .iter()
            .map(|roles| match roles.get(player_index) {
                Some(Wolf) => 1.0,
                Some(Maniac) => maniac_ratio,
                _ => 0.0,
            })
            .sum();
        ratio_sum / ok_roles_count * scale
    }

    fn tell_co_withdrawal(&mut self, player_index: usize) {
        for tell in self.tell_list.iter_mut() {
            if tell.told_player.as_ref().is_some() && tell.told_player_index() != player_index {
                continue;
            }
            match tell.tell_type {
                TellType::CO | TellType::HasPosition => {
                    tell.is_valid = false;
                }
                _ => {}
            }
        }
    }

    pub fn tell_co(&mut self, player_index: usize, role: Role) {
        let tell = Tell::new_co(player_index, role);
        self.tell_co_withdrawal(player_index);
        self.tell_list.push(tell);
    }

    pub fn tell_has_position(&mut self, player_index: usize) {
        let tell = Tell::new_has_position(player_index);
        self.tell_list.push(tell);
    }

    pub fn tell_fortune(&mut self, player_index: usize, target_player_index: usize, color: Color) {
        let tell = Tell::new_fortune(player_index, target_player_index, color);
        self.tell_list.push(tell);
    }

    pub fn tell_see(&mut self, player_index: usize, target_player_index: usize, color: Color) {
        let tell = Tell::new_see(player_index, target_player_index, color);
        self.tell_list.push(tell);
    }

    pub fn tell_guard(&mut self, player_index: usize, target_player_index: usize) {
        let tell = Tell::new_guard(player_index, target_player_index);
        self.tell_list.push(tell);
    }

    pub fn tell_partner(&mut self, player_index: usize, target_player_index: usize) {
        let tell = Tell::new_partner(player_index, target_player_index);
        self.tell_list.push(tell);
    }

    pub fn tell_no_opposition(&mut self, role: Role) {
        let tell = Tell::new_no_opposition(role);
        self.tell_list.push(tell);
    }

    pub fn tell_executed(&mut self, target_player_index: usize) {
        let tell = Tell::new_executed(target_player_index);
        self.tell_list.push(tell);
    }

    pub fn tell_killed(&mut self, target_player_index: usize) {
        let tell = Tell::new_killed(target_player_index);
        self.tell_list.push(tell);
    }

    pub fn tell_peace(&mut self) {
        let tell = Tell::new_peace();
        self.tell_list.push(tell);
    }

    fn check_role_different_co(&self, roles: &[Role], co_role_indexes: &[Vec<usize>]) -> bool {
        !co_role_indexes
            .iter()
            .enumerate()
            .any(|(role_index, co_player_indexes)| {
                co_player_indexes.iter().any(|co_player_index| {
                    !roles[*co_player_index].is_non_human()
                        && roles[*co_player_index].get_index() != role_index
                })
            })
    }

    fn check_has_position(&self, roles: &[Role], has_position_indexes: &Vec<usize>) -> bool {
        for index in has_position_indexes {
            match roles[*index] {
                Villager => return false,
                _ => continue,
            }
        }
        true
    }

    fn check_no_position(&self, roles: &[Role], no_position_player_indexes: &Vec<usize>) -> bool {
        for index in no_position_player_indexes {
            if roles[*index].is_should_co_position() {
                return false;
            }
        }
        true
    }

    fn check_co_non_human(&self, roles: &[Role], non_human_co_player_indexes: &Vec<usize>) -> bool {
        for index in non_human_co_player_indexes {
            if !roles[*index].is_non_human() {
                return false;
            }
        }
        true
    }

    fn check_killed_is_not_wolf(&self, roles: &[Role], killed_player_indexes: &[usize]) -> bool {
        !roles.iter().enumerate().any(|(player_index, role)| {
            killed_player_indexes.contains(&player_index) && matches!(role, Wolf)
        })
    }

    // 平和の前に死んだプレイヤーは狩人ではない
    fn check_dead_player_before_peace(
        &self,
        roles: &[Role],
        before_peace_dead_player_indexes: &[usize],
    ) -> bool {
        // 与えられた role_list 内のいずれのプレイヤーが下記条件に該当するとその role_list は NG
        // 狩人、かつ平和の前に死亡（処刑 or 噛まれ）
        !roles.iter().enumerate().any(|(player_index, role)| {
            matches!(role, Hunter) && before_peace_dead_player_indexes.contains(&player_index)
        })
    }
    //　対抗なしチェック
    fn check_no_opposition(
        &self,
        roles: &[Role],
        target_role: &Role,
        position_co_indexes: &[Vec<usize>],
    ) -> bool {
        let target_co_indexes: &Vec<usize> = &position_co_indexes[target_role.get_index()];
        !roles.iter().enumerate().any(|(player_index, role)| {
            // 役職が同じでない
            role.get_index() != target_role.get_index() &&
                // CO者である
                target_co_indexes.contains(&player_index)
        })
    }
    fn check_wolf_saturation(
        &self,
        roles: &[Role],
        dead_player_timeline: &Vec<Vec<usize>>,
    ) -> bool {
        for dead_player_indexes in dead_player_timeline {
            let still_alive_roles: Vec<&Role> = roles
                .iter()
                .enumerate()
                .filter(|(player_index, role)| !dead_player_indexes.contains(player_index))
                .map(|(_, role)| role)
                .collect();
            let still_alive_player_number = still_alive_roles.len();
            let wolf_count = still_alive_roles
                .into_iter()
                .filter(|role| matches!(role, Wolf))
                .count();
            if wolf_count == 0 || wolf_count * 2 >= still_alive_player_number {
                // console_log!(
                //     "wolf_satuation_ng{:?}, {:?}",
                //     wolf_count,
                //     still_alive_player_number
                // );
                return false;
            }
            if still_alive_player_number == 4 {
                // console_log!(
                //     "debug　{:?} {:?} {:?} {:?}",
                //     still_alive_player_number,
                //     wolf_count,
                //     roles,
                //     dead_player_timeline
                // );
            }
        }
        true
    }

    fn check_fortune_teller_view(
        &self,
        roles: &[Role],
        fortune_color_indexes: &Vec<(usize, usize, &Color)>,
    ) -> bool {
        !fortune_color_indexes
            .iter()
            .any(|(fortune_teller_index, fortune_telling_index, color)| {
                matches!(roles[*fortune_teller_index], FortuneTeller)
                    && roles[*fortune_telling_index].get_color().get_index() != color.get_index()
            })
    }
    fn check_medium_view(
        &self,
        roles: &[Role],
        see_color_indexes: &Vec<(usize, usize, &Color)>,
    ) -> bool {
        !see_color_indexes
            .iter()
            .any(|(medium_index, see_index, color)| {
                matches!(roles[*medium_index], Medium)
                    && roles[*see_index].get_color().get_index() != color.get_index()
            })
    }
    fn check_hunter_guard_view(&self, roles: &[Role], guard_indexes: &[(usize, usize)]) -> bool {
        !guard_indexes.iter().any(|(hunter_index, guard_index)| {
            matches!(roles[*hunter_index], Hunter)
                && matches!(roles[*guard_index].get_color(), Color::Black)
        })
    }
    fn check_partner_view(&self, roles: &[Role], partner_indexes: &[(usize, usize)]) -> bool {
        !partner_indexes.iter().any(|(mason_index, partner_index)| {
            matches!(roles[*mason_index], Mason) && !matches!(roles[*partner_index], Mason)
        })
    }
    fn check_should_co_when_all_exposure(
        &self,
        suggested_roles: &[Role],
        should_co_position_player_count: usize,
        position_co_indexes: &[Vec<usize>],
    ) -> bool {
        // console_log!("check target 1 {:?} {:?} {:?}", position_co_indexes, suggested_roles, self.non_human_count_of_rule);
        // 個別の役職について、単体で人外が全部出ているかの判定
        // 2-0CO、2人外の時は、真占いは出なくてはいけない
        for (role_index, position_co_index_list) in position_co_indexes.iter().enumerate() {
            // console_log!("check target 2 {:?} {:?}", role_index, position_co_index_list);
            if position_co_index_list.len() < self.constants.non_human_count_of_rule {
                continue;
            }
            if !Role::from_index(role_index).is_should_co_position() {
                continue;
            }
            // この役職について他にCO者がいてはいけない
            for (n, role) in suggested_roles.iter().enumerate() {
                // 違う役職である
                if role_index != role.get_index() {
                    continue;
                }
                // すでにCO者である
                if position_co_index_list.contains(&n) {
                    continue;
                }
                return false;
            }
        }
        // ルール上のCO必要役職数(例えば1+1)と人外の数(例えば3)を足したものが、
        // 現在のCO必要役職のCO数(例えば3)よりも1以上大きい場合、
        // 潜伏真がCOしても全人外露出にはならないため、COの必要は発生しない
        if self.constants.should_co_position_count_of_rule + self.constants.non_human_count_of_rule
            > should_co_position_player_count + 1
        {
            return true;
        }

        // 全人外露出にリーチしているので、
        // 潜伏している真はCOしなければならない（はず、奇策は知らない）
        for (player_index, suggested_role) in suggested_roles
            .iter()
            .enumerate()
            .take(self.constants.player_count)
        {
            // COの必要がない役職は無視
            if !suggested_role.is_should_co_position() {
                continue;
            }
            let suggested_role_index = suggested_role.get_index();
            let position_co_list = &position_co_indexes[suggested_role_index];
            // すでにCO済み
            // この判定上と合わせて2回やってる
            if position_co_list.contains(&player_index) {
                continue;
            }
            return false;
        }
        true
    }
    pub fn expect_wolf(&mut self) -> Result<(), String> {
        let mut constraint =
            IteratorConstraint::new(self.constants.player_count, self.constants.max_role_index);
        self.update_iterator_constraint(&mut constraint);
        println!("{:?}", constraint);
        let role_list = &mut self.role_list;
        role_list.sort_by_key(|role| role.get_index());
        self.ok_roles.clear();
        self.ng_roles.clear();
        // self.breakdown.dead_events.clear();
        // self.breakdown.dead_events = self
        //     .tell_list
        //     .iter()
        //     .filter(|tell| match tell.tell_type {
        //         TellType::Executed | TellType::Killed => tell.is_valid,
        //         _ => false,
        //     })
        //     .cloned()
        //     .collect::<Vec<Tell>>();
        // self.breakdown.co_events.clear();
        // self.breakdown.co_events = self
        //     .tell_list
        //     .iter()
        //     .filter(|tell| match tell.tell_type {
        //         TellType::CO | TellType::HasPosition => tell.is_valid,
        //         _ => false,
        //     })
        //     .collect::<Vec<&Tell>>();
        self.breakdown.wolf_indexes.clear();
        let mut ng_reason: HashMap<Vec<Role>, (usize, NgReason)> = HashMap::new();
        // TODO
        // ランダム生成のループ回数を何回にするかは内訳による
        // for _ in 0..100000 {
        //     role_list.shuffle(&mut rng);
        //     let role_key = format!("{:?}", role_list);
        //     if role_key_set.contains(&role_key) {
        //         continue;
        //     }
        //     role_key_set.insert(role_key);
        //     self.ok_roles.push(role_list.to_owned());
        // }
        let mut iter = RoleIterator::new_with_constraint(role_list, constraint);
        self.ok_roles.push(iter.role_list.clone());
        while let Some(_) = iter.next_with_constraint() {
            self.ok_roles.push(iter.role_list.clone());
        }
        for tell_index in 0..self.tell_list.len() {
            if !self.tell_list[tell_index].is_valid {
                continue;
            }
            self.available_role_list.clear();
            let tell_list_sub = &self.tell_list[0..=tell_index];
            // COのうち役職のみを集約した二次元配列
            let mut position_co_indexes: Vec<Vec<usize>> = vec![vec![]; Role::get_max_index() + 1];
            for tell in tell_list_sub.iter() {
                if matches!(tell.tell_type, TellType::CO) {
                    let role = tell.target_role.as_ref().unwrap();
                    if tell.is_valid && role.is_position() {
                        position_co_indexes[role.get_index()].push(tell.told_player_index());
                    }
                }
            }
            // console_log!("position co: {:?}", position_co_indexes);
            let has_position_player_events: Vec<&Tell> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::HasPosition => tell.is_valid,
                    _ => false,
                })
                .collect();
            // let has_position_player_indexes = has_position_player_events
            //     .iter()
            //     .map(|tell| tell.told_player_index())
            //     .collect();
            // 役職COのうち真が潜伏していたら出た方がいい役職について集約
            // 狩人については偽は放っておかれるし噛みでわかるので潜伏でよい
            let should_co_position_player_indexes: Vec<usize> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::CO => {
                        tell.is_valid && tell.target_role.as_ref().unwrap().is_should_co_position()
                    }
                    _ => false,
                })
                .map(|tell| tell.told_player_index())
                .collect();
            let should_co_position_player_count = should_co_position_player_indexes.len();
            // let no_position_player_indexes: Vec<usize> = tell_list_sub
            //     .iter()
            //     .filter(|tell| match tell.tell_type {
            //         TellType::CO => {
            //             tell.is_valid && matches!(tell.target_role.as_ref().unwrap(), Villager)
            //         }
            //         _ => false,
            //     })
            //     .map(|tell| tell.told_player_index())
            //     .collect();
            let non_human_co_player_indexes: Vec<usize> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::CO => {
                        tell.is_valid && tell.target_role.as_ref().unwrap().is_non_human()
                    }
                    _ => false,
                })
                .map(|tell| tell.told_player_index())
                .collect();
            let fortune_color_indexes: Vec<(usize, usize, &Color)> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::Fortune => tell.is_valid,
                    _ => false,
                })
                .map(|tell| match tell.tell_type {
                    TellType::Fortune => (
                        tell.told_player_index(),
                        tell.target_player_index(),
                        tell.target_color.as_ref().unwrap(),
                    ),
                    _ => panic!(),
                })
                .collect();
            let see_color_indexes: Vec<(usize, usize, &Color)> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::See => tell.is_valid,
                    _ => false,
                })
                .map(|tell| match tell.tell_type {
                    TellType::See => (
                        tell.told_player_index(),
                        tell.target_player_index(),
                        tell.target_color.as_ref().unwrap(),
                    ),
                    _ => panic!(),
                })
                .collect();
            let guard_indexes: Vec<(usize, usize)> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::Guard => tell.is_valid,
                    _ => false,
                })
                .map(|tell| match tell.tell_type {
                    TellType::Guard => (tell.told_player_index(), tell.target_player_index()),
                    _ => panic!(),
                })
                .collect();
            let partner_indexes: Vec<(usize, usize)> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::Partner => tell.is_valid,
                    _ => false,
                })
                .map(|tell| match tell.tell_type {
                    TellType::Partner => (tell.told_player_index(), tell.target_player_index()),
                    _ => panic!(),
                })
                .collect();
            let player_killed_events: Vec<&Tell> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::Killed => tell.is_valid,
                    _ => false,
                })
                .collect();
            let killed_player_indexes: Vec<usize> = player_killed_events
                .iter()
                .map(|tell| tell.target_player_index())
                .collect();
            let player_executed_events: Vec<&Tell> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::Executed => tell.is_valid,
                    _ => false,
                })
                .collect();
            let executed_player_indexes: Vec<usize> = player_executed_events
                .iter()
                .map(|tell| tell.target_player_index())
                .collect();
            let player_dead_events: Vec<&Tell> = tell_list_sub
                .iter()
                .filter(|tell| match tell.tell_type {
                    TellType::Killed | TellType::Executed => tell.is_valid,
                    _ => false,
                })
                .collect();
            let dead_player_indexes: Vec<usize> = player_dead_events
                .iter()
                .map(|tell| tell.target_player_index())
                .collect();
            // [dead1, dead2, dead3] => [[dead1], [dead1, dead2], [dead1, dead2, dead3]]
            let mut dead_player_timeline: Vec<Vec<usize>> = vec![];
            for (parent_index, tell) in player_dead_events.iter().enumerate() {
                let mut parent_indexes: Vec<usize> = vec![];
                for (child_index, tell) in player_dead_events.iter().enumerate() {
                    if child_index <= parent_index {
                        parent_indexes.push(tell.target_player_index());
                    }
                }
                dead_player_timeline.push(parent_indexes);
            }
            // console_log!("dead_player_timeline {:?}", dead_player_timeline);
            let max_peace_tell_index = tell_list_sub
                .iter()
                .enumerate()
                .filter(|(_, tell)| match tell.tell_type {
                    TellType::Peace => tell.is_valid,
                    _ => false,
                })
                .map(|(tell_index, _)| tell_index)
                .max();
            let mut before_peace_dead_player_indexes = vec![];
            if self.constants.flag.has_hunter {
                if let Some(target_tell_index) = max_peace_tell_index {
                    for role in self.role_list.iter() {
                        if matches!(role, Hunter) {
                            before_peace_dead_player_indexes = tell_list_sub
                                .iter()
                                .enumerate()
                                .filter(|(tell_index, tell)| {
                                    *tell_index < target_tell_index
                                        && match tell.tell_type {
                                            TellType::Killed | TellType::Executed => tell.is_valid,
                                            _ => false,
                                        }
                                })
                                .map(|(_, tell)| tell.target_player_index())
                                .collect();
                            break;
                        }
                    }
                }
                println!(
                    "before peace dead player indexes len: {}",
                    before_peace_dead_player_indexes.len()
                );
            }
            // console_log!("{:?}", dead_player_indexes);
            // let mut output_count = 0;
            let mut new_ok_roles: Vec<Vec<Role>> = vec![];
            println!("{}", self.ok_roles.len());
            for role_list in self.ok_roles.iter() {
                if !self.check_fortune_teller_view(role_list, &fortune_color_indexes) {
                    self.ng_roles.push(role_list.clone());
                    ng_reason.insert(role_list.clone(), (tell_index, FortuneTellerView));
                    continue;
                }
                if !self.check_medium_view(role_list, &see_color_indexes) {
                    println!("ng medium view {:?}", role_list);
                    self.ng_roles.push(role_list.clone());
                    ng_reason.insert(role_list.clone(), (tell_index, MediumView));
                    continue;
                }
                if !self.check_hunter_guard_view(role_list, &guard_indexes) {
                    println!("ng hunter guard view {:?}", role_list);
                    self.ng_roles.push(role_list.clone());
                    ng_reason.insert(role_list.clone(), (tell_index, GuardView));
                    continue;
                }
                if !self.check_partner_view(role_list, &partner_indexes) {
                    println!("ng partner view {:?}", role_list);
                    self.ng_roles.push(role_list.clone());
                    ng_reason.insert(role_list.clone(), (tell_index, PartnerView));
                    continue;
                }
                if self.constants.flag.has_hunter
                    && !self.check_dead_player_before_peace(
                        role_list,
                        &before_peace_dead_player_indexes,
                    )
                {
                    println!("ng dead player before peace {:?}", role_list);
                    self.ng_roles.push(role_list.clone());
                    ng_reason.insert(role_list.clone(), (tell_index, HunterDeadAllBeforePeace));
                    continue;
                }
                // 最後のイベントが対抗なしイベントだった時に追加でチェック
                if matches!(self.tell_list[tell_index].tell_type, TellType::NoOpposition)
                    && !self.check_no_opposition(
                        role_list,
                        self.tell_list[tell_index].target_role.as_ref().unwrap(),
                        &position_co_indexes,
                    )
                {
                    println!("ng no opposition {:?}", role_list);
                    self.ng_roles.push(role_list.clone());
                    ng_reason.insert(role_list.clone(), (tell_index, HasOpposition));
                    continue;
                }

                //　最後にチェックしないとダメな気がする…
                if !self.check_wolf_saturation(role_list, &dead_player_timeline) {
                    println!("ng check_wolf_saturation {:?}", role_list);
                    self.ng_roles.push(role_list.clone());
                    ng_reason.insert(role_list.clone(), (tell_index, WolfSaturation));
                    continue;
                }
                // if !self.check_should_co_when_all_exposure(
                //     &role_list,
                //     should_co_position_player_count,
                //     &position_co_indexes,
                // ) {
                //     console_log!("ng should co when all exposure  {:?}", role_list);
                //     self.ng_roles.push(role_list.to_owned());
                //     ng_reason.insert(role_list.to_owned(), (tell_index, ShouldCoWhenAllExposure));
                //     continue;
                // }
                // output_count += 1;
                // console_log!("ok {:?} {:?}", output_count, role_list);
                new_ok_roles.push(role_list.clone());
            }
            self.ok_roles = new_ok_roles;
            // console_log!("ok_roles: {:?} {:?}", self.ok_roles.len(), self.ok_roles);
            // console_log!("ng_reason: {:?}", ng_reason,);
            for n in 0..self.player_list.len() {
                let available_roles: HashSet<Role> = self
                    .ok_roles
                    .iter()
                    .map(|suggested_role| suggested_role[n].clone())
                    .collect();
                let unavailable_roles: HashSet<Role> = self
                    .role_list
                    .iter()
                    .filter(|role| !available_roles.contains(*role))
                    .map(|role| Role::from_index(role.get_index()))
                    .collect();
                // console_log!(
                //     "{} player able role {:?} {:?}",
                //     n,
                //     available_roles,
                //     unavailable_roles
                // );
                let mut copied_available_roles: Vec<Role> = available_roles
                    .iter()
                    .map(|role| Role::from_index(role.get_index()))
                    .collect();
                copied_available_roles.sort_by_key(|role| role.get_index());
                self.available_role_list.push(copied_available_roles);
                let mut max_of_max_tell_index = 0_usize;
                if !unavailable_roles.is_empty() {
                    for ng_role in unavailable_roles.iter() {
                        let max_tell_index = ng_reason
                            .iter()
                            .filter(|entry| entry.0[n] == *ng_role)
                            .map(|entry| entry.1.0)
                            .max()
                            .unwrap();
                        max_of_max_tell_index = cmp::max(max_tell_index, max_of_max_tell_index);
                        // let ng_reason_set: HashSet<NgReason> = ng_reason
                        //     .iter()
                        //     .filter(|entry| entry.0[n] == *ng_role && entry.1.0 == max_tell_index)
                        //     .map(|entry| entry.1.1)
                        //     .collect();

                        // let ng_reason_text = ng_reason_set
                        //     .iter()
                        //     .map(|reason| reason.get_reason_text())
                        //     .collect::<Vec<String>>()
                        //     .join(",");
                        // console_log!(
                        //     "ng reason: {:?} {:?} {:?}",
                        //     ng_role,
                        //     max_tell_index,
                        //     ng_reason_text
                        // );
                    }

                    // 中身確定
                    if available_roles.len() == 1 {
                        let role = available_roles.iter().find(|_| true).unwrap();
                        // console_log!("ok reason: {:?} {:?}", role, max_of_max_tell_index);
                        if matches!(role, Wolf) {
                            self.breakdown.wolf_indexes.push(n);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update_iterator_constraint(&self, constraint: &mut IteratorConstraint) {
        for tell in self.tell_list.iter() {
            if !tell.is_valid {
                continue;
            }
            match tell.tell_type {
                TellType::CO => {
                    let player_index = tell.told_player.as_ref().unwrap().index;
                    let co_role = tell.target_role.as_ref().unwrap();
                    if matches!(co_role, Villager) {
                        for role in self.role_list.iter() {
                            if role.is_should_co_position() {
                                constraint.add(player_index, role.get_index());
                            };
                        }
                    } else if co_role.is_non_human() {
                        for role in self.role_list.iter() {
                            if !role.is_non_human() {
                                constraint.add(player_index, role.get_index());
                            };
                        }
                    } else {
                        for role in self.role_list.iter() {
                            if !role.is_allowed_co(co_role) {
                                constraint.add(player_index, role.get_index());
                            };
                        }
                    }
                }
                TellType::HasPosition => {
                    let player_index = tell.told_player.as_ref().unwrap().index;
                    constraint.add(player_index, Villager.get_index());
                }
                TellType::Killed => {
                    let player_index = tell.target_player.as_ref().unwrap().index;
                    constraint.add(player_index, Wolf.get_index());
                }

                _ => {}
            }
        }
    }
}
