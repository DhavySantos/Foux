pub struct Window {
    title: String,
}

impl Window {
    pub fn new(title: &str) -> Window {
        let title = String::from(title);
        Window { title }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }
}
