use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    action_type: ActionType,
    target: Option<ActionTarget>,
    duration: f32,
    progress: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Move { destination: Vector2 },
    Talk { dialogue: String },
    Trade { item: String, price: f32 },
    Work { task: String },
    Rest { duration: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionTarget {
    Entity(Uuid),
    Location(Vector2),
    Item(String),
}

impl Action {
    pub fn new(action_type: ActionType, target: Option<ActionTarget>, duration: f32) -> Self {
        Self {
            action_type,
            target,
            duration,
            progress: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.progress += delta_time;
        self.progress >= self.duration
    }
}