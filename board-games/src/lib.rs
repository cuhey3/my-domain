mod connect4;
mod shogi55;

pub use connect4::draw_data::Connect4DrawData;
pub use connect4::draw_data::Connect4DrawTask;
pub use connect4::init_connect4;
pub use connect4::structs::Connect4Data;
pub use connect4::structs::board::Connect4Board;
pub use my_board_game::GameData;
pub use my_board_game::GameSystem;
pub use shogi55::draw_data;
pub use shogi55::init_shogi55;
pub use shogi55::structs::board::Shogi55Board;
pub use shogi55::structs::board::Shogi55Place;
