pub mod npc;
pub mod environment;

use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub position: Vector2,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    NPC(npc::NPCType),
    Environment(environment::EnvironmentType),
}

impl Entity {
    pub fn new(entity_type: EntityType, position: Vector2) -> Self {
        Self {
            id: Uuid::new_v4(),
            entity_type,
            position,
            active: true,
        }
    }
}