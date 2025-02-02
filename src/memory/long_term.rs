use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::Memory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermMemory {
    memories: HashMap<Uuid, Memory>,
    connections: HashMap<Uuid, Vec<Uuid>>,
    emotional_index: HashMap<String, Vec<Uuid>>,
}

impl Default for LongTermMemory {
    fn default() -> Self {
        Self {
            memories: HashMap::new(),
            connections: HashMap::new(),
            emotional_index: HashMap::new(),
        }
    }
}

impl LongTermMemory {
    pub fn add_memory(&mut self, memory: Memory) {
        // Index by emotion
        let emotion = categorize_emotion(memory.emotional_value);
        self.emotional_index
            .entry(emotion)
            .or_default()
            .push(memory.id);

        // Create connections with related memories
        for related_id in &memory.related_entities {
            if let Some(related_memory) = self.memories.get(related_id) {
                // Create bidirectional connection
                self.connections
                    .entry(memory.id)
                    .or_default()
                    .push(*related_id);
                self.connections
                    .entry(*related_id)
                    .or_default()
                    .push(memory.id);
            }
        }

        // Store the memory
        self.memories.insert(memory.id, memory);
    }

    pub fn find_memory(&self, query: &str) -> Option<Memory> {
        self.memories
            .values()
            .find(|m| m.content.contains(query))
            .cloned()
    }

    pub fn get_connected_memories(&self, memory_id: Uuid) -> Vec<Memory> {
        self.connections
            .get(&memory_id)
            .map(|connections| {
                connections
                    .iter()
                    .filter_map(|id| self.memories.get(id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_emotional_memories(&self, emotion: &str) -> Vec<Memory> {
        self.emotional_index
            .get(emotion)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.memories.get(id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn strengthen_connection(&mut self, memory_id1: Uuid, memory_id2: Uuid) {
        if self.memories.contains_key(&memory_id1) && self.memories.contains_key(&memory_id2) {
            self.connections
                .entry(memory_id1)
                .or_default()
                .push(memory_id2);
            self.connections
                .entry(memory_id2)
                .or_default()
                .push(memory_id1);
        }
    }
}

fn categorize_emotion(emotional_value: f32) -> String {
    match emotional_value {
        x if x > 0.7 => "very_positive",
        x if x > 0.3 => "positive",
        x if x > -0.3 => "neutral",
        x if x > -0.7 => "negative",
        _ => "very_negative",
    }.to_string()
}