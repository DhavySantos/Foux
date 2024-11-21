use crate::core::{components::MeshRenderer, Window};
use std::collections::HashMap;

/// Enum representing different system types for scheduling.
#[derive(Hash, PartialEq, Eq)]
pub enum SystemType {
    /// Represents the startup system type.
    Startup,
    /// Represents the update system type.
    Update,
    /// Represents the key system type.
    Key,
}

pub type System = dyn FnMut(&mut Window, &mut Vec<MeshRenderer>);

/// A struct that holds a collection of systems (functions) categorized by their type.
#[derive(Default)]
pub struct Scheduler {
    /// A HashMap that maps system types to a vector of systems (functions).
    systems: HashMap<SystemType, Vec<Box<System>>>,
}

impl Scheduler {
    /// Creates a new, empty `Scheduler`.
    ///
    /// # Returns
    /// A new `Scheduler` instance with no systems.
    pub fn new() -> Scheduler {
        Scheduler::default()
    }

    /// Inserts a system (function) into the scheduler under a specific system type.
    ///
    /// # Arguments
    ///
    /// * `system_type` - The type of system (e.g., `Startup`, `Update`).
    /// * `system` - The function to be scheduled, which must implement `FnMut`.
    ///
    /// # Example
    /// ```
    /// let mut scheduler = Scheduler::new();
    /// scheduler.insert(SystemType::Startup, || println!("Starting up..."));
    /// ```
    pub fn insert(&mut self, system_type: SystemType, system: impl FnMut(&mut Window, &mut Vec<MeshRenderer>) + 'static) {
        self.systems
            .entry(system_type)
            .or_default()
            .push(Box::new(system));
    }

    /// Invokes all systems (functions) of a specified system type.
    ///
    /// This method will run all the functions that are associated with the given `system_type`.
    ///
    /// # Arguments
    ///
    /// * `system_type` - The type of system whose functions are to be invoked.
    /// * `window` - The window instance passed to the systems.
    ///
    /// # Example
    /// ```
    /// let mut scheduler = Scheduler::new();
    /// scheduler.insert(SystemType::Startup, || println!("Starting up..."));
    /// scheduler.invoke(SystemType::Startup);
    /// ```
    pub fn invoke(&mut self, system_type: SystemType, window: &mut Window, renders: &mut Vec<MeshRenderer>) {
        if let Some(systems) = self.systems.get_mut(&system_type) {
            for system in systems.iter_mut() {
                // Dereference the Box to invoke the function
                system(window, renders);
            }
        }
    }
}
