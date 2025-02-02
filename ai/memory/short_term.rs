use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use super::Memory;

const MAX_SHORT_TERM_MEMORIES: usize = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortTermMemory {
    memories: VecDeque<Memory>,
    capacity: usize,
    total_time: f32,
}

impl Default for ShortTermMemory {
    fn default() -> Self {
        Self {
            memories: VecDeque::with_capacity(MAX_SHORT_TERM_MEMORIES),
            capacity: MAX_SHORT_TERM_MEMORIES,
            total_time: 0.0,
        }
    }
}

impl ShortTermMemory {
    pub fn update(&mut self, delta_time: f32) {
        self.total_time += delta_time;
        
        // Remove old memories if we're over capacity
        while self.memories.len() > self.capacity {
            self.memories.pop_back();
        }
    }

    pub fn add_memory(&mut self, mut memory: Memory) {
        memory.timestamp = self.total_time;
        
        // Add to front of queue
        self.memories.push_front(memory);
        
        // Ensure we don't exceed capacity
        while self.memories.len() > self.capacity {
            self.memories.pop_back();
        }
    }

    pub fn find_memory(&self, query: &str) -> Option<Memory> {
        self.memories
            .iter()
            .find(|m| m.content.contains(query))
            .cloned()
    }

    pub fn get_recent_memories(&self, count: usize) -> Vec<Memory> {
        self.memories
            .iter()
            .take(count)
            .cloned()
            .collect()
    }

    pub fn get_important_memories(&self) -> Vec<Memory> {
        self.memories
            .iter()
            .filter(|m| m.importance > 0.7)
            .cloned()
            .collect()
    }

    pub fn clear_old_memories(&mut self, threshold_time: f32) {
        self.memories.retain(|m| self.total_time - m.timestamp < threshold_time);
    }

    pub fn get_memories_by_emotion(&self, emotion_threshold: f32) -> Vec<Memory> {
        self.memories
            .iter()
            .filter(|m| m.emotional_value >= emotion_threshold)
            .cloned()
            .collect()
    }
}