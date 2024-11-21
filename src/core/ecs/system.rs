use std::collections::HashMap;


#[derive(Hash, PartialEq, Eq)]
pub enum SystemType {
    Startup,
    Update,
}

#[derive(Default)]
pub struct Scheduler {
    systems: HashMap<SystemType, Vec<Box<dyn FnMut()>>>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler::default()
    }

    pub fn insert(&mut self, system_type: SystemType, system: impl FnMut() + 'static) {
        self.systems
            .entry(system_type)
            .or_default()
            .push(Box::new(system));
    }

    pub fn invoke(&mut self, system_type: SystemType) {
        for system in self.systems.entry(system_type).or_default() {
            system();
        }
    }
}
