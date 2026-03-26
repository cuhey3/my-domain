#[derive(Default)]
pub struct CommonPlayer {
    name: String,
    id: u64,
}

impl CommonPlayer {
    pub fn new(name: String, id: u64) -> Self {
        CommonPlayer { name, id }
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}
