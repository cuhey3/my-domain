use my_jinro::Role;
use my_jinro::village::Village;

fn main() {
    jinro();
}

fn jinro() {
    let mut village = Village::new();
    village.set_role_list_by_index(vec![0, 1, 2, 4, 4, 5, 6, 6]);
    // village.set_role_list_by_index(vec![0, 0, 0, 0, 0, 1, 2, 3, 4, 4, 5, 6, 6]);
    println!("{:?}", village.role_list);
    village.tell_co(0, Role::FortuneTeller);
    village.tell_killed(1);
    village.tell_has_position(2);
    village.tell_co(3, Role::Villager);
    village.tell_co(4, Role::Maniac);
    let result = village.expect_wolf();
    println!("{:?}", result);
    // println!("{:#?}", village.ok_roles);
    println!("{:#?}", village.ok_roles.len());
}
