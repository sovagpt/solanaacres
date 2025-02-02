use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub mod short_term;
pub mod long_term;
pub mod decay;
pub mod importance;

use short_term::ShortTermMemory;
use long_term::LongTermMemory;
use decay::MemoryDecay;
use importance::ImportanceScoring;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySystem {
    short_term: ShortTermMemory,
    long_term: LongTermMemory,
    decay_system: MemoryDecay,
    importance_scorer: ImportanceScoring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    id: Uuid,
    content: String,
    importance: f32,
    emotional_value: f32,
    timestamp: f32,
    related_entities: Vec<Uuid>,
    decay_rate: f32,
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self {
            short_term: ShortTermMemory::default(),
            long_term: LongTermMemory::default(),
            decay_system: MemoryDecay::default(),
            importance_scorer: ImportanceScoring::default(),
        }
    }
}

impl MemorySystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update short-term memory
        self.short_term.update(delta_time);
        
        // Process memory decay
        self.decay_system.process(&mut self.short_term, &mut self.long_term, delta_time);
        
        // Transfer important memories to long-term
        self.transfer_to_long_term();
    }

    pub fn add_memory(&mut self, content: String, emotional_value: f32, related_entities: Vec<Uuid>) {
        let importance = self.importance_scorer.calculate_importance(&content, emotional_value);
        
        let memory = Memory {
            id: Uuid::new_v4(),
            content,
            importance,
            emotional_value,
            timestamp: 0.0, // Current simulation time should be passed in
            related_entities,
            decay_rate: self.decay_system.calculate_initial_decay_rate(importance),
        };

        self.short_term.add_memory(memory);
    }

    pub fn recall_memory(&self, query: &str) -> Option<Memory> {
        // First check short-term memory
        if let Some(memory) = self.short_term.find_memory(query) {
            return Some(memory);
        }
        
        // Then check long-term memory
        self.long_term.find_memory(query)
    }

    fn transfer_to_long_term(&mut self) {
        let memories_to_transfer = self.short_term.get_important_memories();
        for memory in memories_to_transfer {
            if memory.importance > 0.7 {
                self.long_term.add_memory(memory);
            }
        }
    }
}