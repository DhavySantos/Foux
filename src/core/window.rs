pub struct Window {
    should_close: bool,
    title: String,
}

impl Window {
    pub fn new(title: &str) -> Window {
        let title = String::from(title);
        Window {
            should_close: false,
            title,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn set_should_close(&mut self, state: bool) {
        self.should_close = state;
    }
}
