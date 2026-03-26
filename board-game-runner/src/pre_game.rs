use board_games::framework::Drawer;
use std::any::Any;

#[derive(Default)]
pub struct PreGameDrawer {}

impl PreGameDrawer {}

impl Drawer for PreGameDrawer {
    fn draw(&mut self, draw_data: Box<&mut dyn Any>) {}

    fn draw_error(&mut self, error: String) {
        println!("error: {}", error);
    }
}
