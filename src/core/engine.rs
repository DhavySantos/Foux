use crate::core::Window;

pub struct Engine {
    window: Window,
}

impl Engine {
    pub fn new(title: &str) -> Engine {
        let window = Window::new(title);
        Engine { window }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.window.set_should_close(true);
        }
    }
}
