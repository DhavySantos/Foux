use crate::core::ecs::{Scheduler, SystemType};
use crate::core::Window;

#[derive(Default)]
pub struct Engine {
    scheduler: Scheduler,
}

impl Engine {
    pub fn add_system(&mut self, system_type: SystemType, system: impl FnMut() + 'static) {
        self.scheduler.insert(system_type, system);
    }

    pub fn run(&mut self) {
        self.scheduler.invoke(SystemType::Startup);

        let mut window = Window::new("Foux Engine", 800, 480);

        while !window.should_close() {
            window.clear(0.07, 0.17, 0.07, 1.0);
            self.scheduler.invoke(SystemType::Update);
            window.update();
        }
    }
}
