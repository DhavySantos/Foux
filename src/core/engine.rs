use crate::core::Window;

pub struct Engine {}

impl Engine {
    pub fn run(&mut self) {
        let mut window = Window::new("Foux Engine", 800, 640);
        while !window.should_close() {
            window.clear(0.3, 0.7, 0.3, 1.0);
            window.update();
        }
    }
}
