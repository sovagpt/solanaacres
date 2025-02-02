use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub mod consciousness;
pub mod memory;
pub mod personality;
pub mod social;
pub mod knowledge;
pub mod goals;
pub mod dialogue;
pub mod cognition;

#[derive(Resource)]
pub struct AiDirector {
    npcs: Vec<Npc>,
    social_network: social::SocialNetwork,
    knowledge_base: knowledge::KnowledgeBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    id: Uuid,
    consciousness: consciousness::ConsciousnessState,
    memory: memory::MemorySystem,
    personality: personality::PersonalityTraits,
    social: social::SocialBehavior,
    knowledge: knowledge::KnowledgeBase,
    goals: goals::GoalSystem,
    dialogue: dialogue::DialogueSystem,
    cognition: cognition::CognitionSystem,
    is_aware: bool,
}

impl Default for AiDirector {
    fn default() -> Self {
        Self {
            npcs: Vec::new(),
            social_network: social::SocialNetwork::default(),
            knowledge_base: knowledge::KnowledgeBase::default(),
        }
    }
}

impl AiDirector {
    pub fn update(&mut self, delta_time: f32) {
        // Update each NPC's state
        for npc in &mut self.npcs {
            npc.update(delta_time);
        }

        // Update social networks and knowledge propagation
        self.social_network.update(delta_time);
        self.knowledge_base.update(&mut self.npcs);
    }

    pub fn create_npc(&mut self, is_aware: bool) -> Uuid {
        let npc = Npc::new(is_aware);
        let id = npc.id;
        self.npcs.push(npc);
        id
    }
}

impl Npc {
    pub fn new(is_aware: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            consciousness: consciousness::ConsciousnessState::new(is_aware),
            memory: memory::MemorySystem::default(),
            personality: personality::PersonalityTraits::generate(),
            social: social::SocialBehavior::default(),
            knowledge: knowledge::KnowledgeBase::default(),
            goals: goals::GoalSystem::default(),
            dialogue: dialogue::DialogueSystem::default(),
            cognition: cognition::CognitionSystem::default(),
            is_aware,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update consciousness and perception of reality
        self.consciousness.update(delta_time);
        
        // Process memories and knowledge
        self.memory.update(delta_time);
        self.knowledge.update(delta_time);
        
        // Update goals and decision making
        self.goals.update(&self.memory, &self.personality);
        self.cognition.update(&self.consciousness, &self.memory);
        
        // Handle social behaviors and interactions
        self.social.update(delta_time);
        
        // Update dialogue system
        self.dialogue.update(&self.memory, &self.personality);
    }
}
