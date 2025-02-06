pub mod lifecycle;
pub mod update;

use std::collections::HashMap;
use uuid::Uuid;

pub struct SystemManager {
    lifecycle_system: lifecycle::LifecycleSystem,
    update_system: update::UpdateSystem,
    active_systems: HashMap<String, Box<dyn System>>,
}

pub trait System {
    fn update(&mut self, delta_time: f32);
    fn initialize(&mut self) -> Result<(), String>;
    fn shutdown(&mut self);
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            lifecycle_system: lifecycle::LifecycleSystem::new(),
            update_system: update::UpdateSystem::new(),
            active_systems: HashMap::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.lifecycle_system.update(delta_time);
        self.update_system.update(delta_time);
        
        for system in self.active_systems.values_mut() {
            system.update(delta_time);
        }
    }
}