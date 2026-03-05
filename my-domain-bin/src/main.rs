use board_games::{init_connect4, init_shogi55};
use my_board_game::GameSystem;
use my_jinro::Role;
use my_jinro::village::Village;
use std::io;

fn main() {
    let result = board_game();
    println!("{:?}", result);
}

fn board_game() -> Result<(), String> {
    let seed = getrandom::u64().unwrap();
    // GameSystem<T> はゲームごとに型がバラバラになるのはもうしょうがないので
    // 後続処理を共通化する時はGameSystem<T>とのインターフェースを用意してデータをやりとりする
    // (無理やり Box<dyn GameData> を扱っても結局書くコードはGameDataに移譲されるので変わらない)
    if false {
        start_new_game(&mut init_connect4(seed))?;
    } else {
        start_new_game(&mut init_shogi55(seed))?;
    }

    Ok(())
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

fn start_new_game(game_system: &mut GameSystem) -> Result<(), String> {
    loop {
        let game_data = &game_system.game_data.clone();
        let Some(phase) = game_system.get_phase() else {
            return Err(format!("phase not found: {}", game_system.phase_id));
        };
        phase.read_data(game_data)?;
        while let Some((question, args)) = phase.dialog_question() {
            loop {
                println!("dialog: {}", question);
                let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
                if let Err(message) = phase.dialog_answer(guess, args.clone()) {
                    println!("{}", message);
                } else {
                    break;
                };
            }
        }

        phase.write_data(game_data)?;
        if let Some(phase_id) = phase.next_phase_id() {
            game_system.phase_id = phase_id;
        } else {
            break Ok(());
        }
    }
}
