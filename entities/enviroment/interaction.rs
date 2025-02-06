use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionPoint {
    pub position: Vector2,
    pub interaction_type: InteractionType,
    pub available_actions: Vec<EnvironmentAction>,
    pub current_users: Vec<Uuid>,
    pub max_users: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Door,
    Counter,
    Seat,
    Workstation,
    Storage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentAction {
    Enter,
    Exit,
    Sit,
    Work,
    Trade,
    Store,
    Retrieve,
}

impl InteractionPoint {
    pub fn new(position: Vector2, interaction_type: InteractionType) -> Self {
        Self {
            position,
            interaction_type,
            available_actions: Vec::new(),
            current_users: Vec::new(),
            max_users: 1,
        }
    }

    pub fn is_available(&self) -> bool {
        self.current_users.len() < self.max_users
    }

    pub fn add_user(&mut self, user_id: Uuid) -> bool {
        if self.is_available() {
            self.current_users.push(user_id);
            true
        } else {
            false
        }
    }

    pub fn remove_user(&mut self, user_id: &Uuid) {
        self.current_users.retain(|id| id != user_id);
    }

    pub fn get_available_actions(&self) -> &[EnvironmentAction] {
        &self.available_actions
    }
}

pub struct EnvironmentInteraction {
    pub interaction_points: HashMap<Uuid, InteractionPoint>,
    pub active_interactions: HashMap<Uuid, Vec<Uuid>>, // NPC ID -> Interaction Point IDs
}

impl EnvironmentInteraction {
    pub fn new() -> Self {
        Self {
            interaction_points: HashMap::new(),
            active_interactions: HashMap::new(),
        }
    }

    pub fn start_interaction(&mut self, npc_id: Uuid, point_id: Uuid) -> bool {
        if let Some(point) = self.interaction_points.get_mut(&point_id) {
            if point.add_user(npc_id) {
                self.active_interactions
                    .entry(npc_id)
                    .or_default()
                    .push(point_id);
                return true;
            }
        }
        false
    }

    pub fn end_interaction(&mut self, npc_id: Uuid, point_id: Uuid) {
        if let Some(point) = self.interaction_points.get_mut(&point_id) {
            point.remove_user(&npc_id);
        }
        if let Some(interactions) = self.active_interactions.get_mut(&npc_id) {
            interactions.retain(|id| *id != point_id);
        }
    }
}