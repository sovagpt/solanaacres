pub mod collision;
pub mod proximity;

use std::collections::HashMap;
use uuid::Uuid;

pub struct InteractionSystem {
    collision_system: collision::CollisionSystem,
    proximity_system: proximity::ProximitySystem,
    interaction_history: HashMap<Uuid, Vec<Interaction>>,
}

impl InteractionSystem {
    pub fn new() -> Self {
        Self {
            collision_system: collision::CollisionSystem::new(),
            proximity_system: proximity::ProximitySystem::new(),
            interaction_history: HashMap::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.collision_system.update(delta_time);
        self.proximity_system.update(delta_time);
        self.process_interactions();
    }
}