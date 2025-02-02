use serde::{Serialize, Deserialize};
use super::{ShortTermMemory, LongTermMemory};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDecay {
    base_decay_rate: f32,
    emotional_factor: f32,
    importance_factor: f32,
    time_factor: f32,
}

impl Default for MemoryDecay {
    fn default() -> Self {
        Self {
            base_decay_rate: 0.1,
            emotional_factor: 0.2,
            importance_factor: 0.3,
            time_factor: 0.1,
        }
    }
}

impl MemoryDecay {
    pub fn process(
        &self,
        short_term: &mut ShortTermMemory,
        long_term: &mut LongTermMemory,
        delta_time: f32
    ) {
        // Process short-term memory decay
        self.decay_short_term(short_term, delta_time);
        
        // Process long-term memory decay (slower rate)
        self.decay_long_term(long_term, delta_time);
    }

    pub fn calculate_initial_decay_rate(&self, importance: f32) -> f32 {
        // Higher importance = slower decay
        self.base_decay_rate * (1.0 - importance * self.importance_factor)
    }

    fn decay_short_term(&self, short_term: &mut ShortTermMemory, delta_time: f32) {
        let decay_threshold = 5.0; // 5 seconds for short-term memory
        short_term.clear_old_memories(decay_threshold);
    }

    fn decay_long_term(&self, long_term: &mut LongTermMemory, delta_time: f32) {
        // Decay emotional connections over time
        for memories in long_term.emotional_index.values_mut() {
            memories.retain(|id| {
                if let Some(memory) = long_term.memories.get(id) {
                    let age = delta_time;  // Should be current_time - memory.timestamp
                    let decay_rate = self.calculate_decay_rate(memory.importance, memory.emotional_value);
                    let survival_chance = (-decay_rate * age).exp();
                    
                    rand::random::<f32>() < survival_chance
                } else {
                    false
                }
            });
        }
    }

    fn calculate_decay_rate(&self, importance: f32, emotional_value: f32) -> f32 {
        let base = self.base_decay_rate;
        let emotional = (1.0 - emotional_value.abs()) * self.emotional_factor;
        let importance_mod = (1.0 - importance) * self.importance_factor;
        
        base + emotional + importance_mod
    }

    pub fn adjust_decay_rate(&mut self, 
        new_base: Option<f32>,
        new_emotional: Option<f32>,
        new_importance: Option<f32>,
        new_time: Option<f32>
    ) {
        if let Some(base) = new_base {
            self.base_decay_rate = base;
        }
        if let Some(emotional) = new_emotional {
            self.emotional_factor = emotional;
        }
        if let Some(importance) = new_importance {
            self.importance_factor = importance;
        }
        if let Some(time) = new_time {
            self.time_factor = time;
        }
    }
}