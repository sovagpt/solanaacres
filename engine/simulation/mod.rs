pub mod time;
pub mod events;
pub mod scheduler;

use std::collections::HashMap;
use uuid::Uuid;

pub struct Simulation {
    time_system: time::TimeSystem,
    event_manager: events::EventManager,
    scheduler: scheduler::Scheduler,
    entities: HashMap<Uuid, Entity>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            time_system: time::TimeSystem::new(),
            event_manager: events::EventManager::new(),
            scheduler: scheduler::Scheduler::new(),
            entities: HashMap::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time_system.update(delta_time);
        self.event_manager.process_events();
        self.scheduler.update(delta_time);
        self.update_entities(delta_time);
    }
}