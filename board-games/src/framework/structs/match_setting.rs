#[derive(Default, Clone, Copy)]
pub enum MatchMode {
    Computer,
    #[default]
    Offline,
    Casual,
    Ranked,
}

#[derive(Default, Clone, Copy)]
pub struct MatchSetting {
    match_mode: MatchMode,
    // 待った可否
    enable_do_over: bool,
    // 評価値表示有無
    with_eval_value: bool,
}

impl MatchSetting {
    pub fn get_match_mode(&self) -> MatchMode {
        self.match_mode
    }

    pub fn set_enable_do_over(&mut self, enable_do_over: bool) {
        self.enable_do_over = enable_do_over;
    }

    pub fn set_with_eval_value(&mut self, with_eval_value: bool) {
        self.with_eval_value = with_eval_value;
    }

    pub fn set_cpu_mode(&mut self, has_cpu: bool) {
        if has_cpu {
            self.match_mode = MatchMode::Computer;
        } else {
            self.match_mode = MatchMode::Offline;
        }
    }

    pub fn set_online_mode(&mut self, online_mode: bool) {
        if online_mode {
            self.match_mode = MatchMode::Casual;
        } else {
            self.match_mode = MatchMode::Offline;
        }
    }

    pub fn get_enable_do_over(&self) -> bool {
        self.enable_do_over
    }

    pub fn get_with_eval_value(&self) -> bool {
        self.with_eval_value
    }
}
