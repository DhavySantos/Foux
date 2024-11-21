use crate::core::ecs::{Scheduler, SystemType};
use crate::core::Window;

use super::components::MeshRenderer;

#[derive(Default)]
pub struct Engine {
    scheduler: Scheduler,
    renders: Vec<MeshRenderer>
}

impl Engine {
    pub fn add_system(&mut self, system_type: SystemType, system: impl FnMut(&mut Window, &mut Vec<MeshRenderer>) + 'static) {
        self.scheduler.insert(system_type, system);
    }

    pub fn run(&mut self) {
        let mut window = Window::new("Foux Engine", 800, 480);
        self.scheduler.invoke(SystemType::Startup, &mut window, &mut self.renders);

        while !window.should_close() {
            window.clear(0.07, 0.17, 0.07, 1.0);
            self.scheduler.invoke(SystemType::Update, &mut window, &mut self.renders);
            for render in self.renders.iter() {
                render.render();
            }
            window.update();
        }
    }
}
