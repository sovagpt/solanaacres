pub mod state_machine;
pub mod routines;
pub mod triggers;

use std::collections::HashMap;
use uuid::Uuid;

pub struct BehaviorSystem {
    state_machines: HashMap<Uuid, state_machine::StateMachine>,
    routines: HashMap<Uuid, routines::Routine>,
    trigger_system: triggers::TriggerSystem,
}

impl BehaviorSystem {
    pub fn new() -> Self {
        Self {
            state_machines: HashMap::new(),
            routines: HashMap::new(),
            trigger_system: triggers::TriggerSystem::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.trigger_system.update();
        
        for state_machine in self.state_machines.values_mut() {
            state_machine.update(delta_time);
        }
        
        for routine in self.routines.values_mut() {
            routine.update(delta_time);
        }
    }
}