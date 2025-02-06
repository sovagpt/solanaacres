pub mod interaction;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: Uuid,
    pub env_type: EnvironmentType,
    pub position: Vector2,
    pub interaction_points: Vec<interaction::InteractionPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentType {
    Building {
        building_type: BuildingType,
        size: Vector2,
    },
    Decoration {
        decoration_type: DecorationType,
    },
    Zone {
        zone_type: ZoneType,
        radius: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingType {
    House,
    Shop,
    Tavern,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecorationType {
    Tree,
    Bench,
    Fountain,
    Lamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoneType {
    Social,
    Market,
    Residential,
    Work,
}

impl Environment {
    pub fn new(env_type: EnvironmentType, position: Vector2) -> Self {
        Self {
            id: Uuid::new_v4(),
            env_type,
            position,
            interaction_points: Vec::new(),
        }
    }
}