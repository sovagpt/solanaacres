pub struct LifecycleSystem {
    entities: HashMap<Uuid, EntityLifecycle>,
    pending_creation: Vec<Entity>,
    pending_deletion: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct EntityLifecycle {
    state: LifecycleState,
    creation_time: f32,
    last_update: f32,
    ttl: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LifecycleState {
    Pending,
    Active,
    Inactive,
    MarkedForDeletion,
}

impl LifecycleSystem {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            pending_creation: Vec::new(),
            pending_deletion: Vec::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Process pending creations
        for entity in self.pending_creation.drain(..) {
            self.create_entity(entity);
        }

        // Update existing entities
        for (id, lifecycle) in &mut self.entities {
            if let Some(ttl) = lifecycle.ttl {
                if lifecycle.last_update + ttl < delta_time {
                    self.pending_deletion.push(*id);
                }
            }
            lifecycle.last_update = delta_time;
        }

        // Process deletions
        for id in self.pending_deletion.drain(..) {
            self.delete_entity(id);
        }
    }
}