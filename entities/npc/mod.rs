pub mod states;
pub mod actions;
pub mod memory;
pub mod awareness;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPC {
    pub id: Uuid,
    pub state: states::NPCState,
    pub memory: memory::MemorySystem,
    pub awareness: awareness::AwarenessSystem,
    pub available_actions: Vec<actions::Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NPCType {
    Villager,
    Merchant,
    Guard,
    Wanderer,
}

impl NPC {
    pub fn new(npc_type: NPCType) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: states::NPCState::default(),
            memory: memory::MemorySystem::new(),
            awareness: awareness::AwarenessSystem::new(),
            available_actions: Vec::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.state.update(delta_time);
        self.memory.update(delta_time);
        self.awareness.update(delta_time);
    }
}