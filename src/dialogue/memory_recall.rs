use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecall {
    memories: HashMap<String, Memory>,
    recent_recalls: VecDeque<RecallEvent>,
    associations: HashMap<String, Vec<String>>,
    emotional_triggers: HashMap<String, Vec<String>>,
    recall_strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    content: String,
    importance: f32,
    emotional_value: f32,
    related_entities: Vec<String>,
    recall_count: u32,
    last_recall: f32,
    creation_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallEvent {
    memory_key: String,
    trigger: String,
    success: bool,
    recall_time: f32,
}

impl Default for MemoryRecall {
    fn default() -> Self {
        Self {
            memories: HashMap::new(),
            recent_recalls: VecDeque::with_capacity(100),
            associations: HashMap::new(),
            emotional_triggers: HashMap::new(),
            recall_strength: 0.7,
        }
    }
}

impl MemoryRecall {
    pub fn update(&mut self, delta_time: f32) {
        // Update memory strength based on recall frequency
        for memory in self.memories.values_mut() {
            self.update_memory_strength(memory, delta_time);
        }
        
        // Clean up old recall events
        self.cleanup_recalls();
    }

    pub fn add_memory(&mut self, content: String, importance: f32, emotional_value: f32) {
        let memory = Memory {
            content: content.clone(),
            importance,
            emotional_value,
            related_entities: Vec::new(),
            recall_count: 0,
            last_recall: 0.0,
            creation_time: 0.0, // Current time should be passed
        };

        self.memories.insert(content.clone(), memory);
        self.create_associations(&content);
    }

    pub fn recall_relevant(&mut self, trigger: &str) -> Vec<String> {
        let mut relevant_memories = Vec::new();
        let mut scores = HashMap::new();

        // Score memories based on relevance to trigger
        for (content, memory) in &mut self.memories {
            let score = self.calculate_recall_score(memory, trigger);
            if score > 0.0 {
                scores.insert(content.clone(), score);
            }
        }

        // Sort by score and take top memories
        let mut scored_memories: Vec<_> = scores.into_iter().collect();
        scored_memories.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (content, score) in scored_memories.iter().take(3) {
            if let Some(memory) = self.memories.get_mut(content) {
                memory.recall_count += 1;
                memory.last_recall = 0.0; // Current time should be passed
                
                // Record recall event
                self.record_recall(content.clone(), trigger.to_string(), true);
                relevant_memories.push(content.clone());
            }
        }

        relevant_memories
    }

    pub fn add_association(&mut self, trigger: String, memory_key: String) {
        self.associations
            .entry(trigger)
            .or_default()
            .push(memory_key);
    }

    pub fn add_emotional_trigger(&mut self, emotion: String, memory_key: String) {
        self.emotional_triggers
            .entry(emotion)
            .or_default()
            .push(memory_key);
    }

    fn calculate_recall_score(&self, memory: &Memory, trigger: &str) -> f32 {
        let mut score = 0.0;

        // Direct content match
        if memory.content.contains(trigger) {
            score += 1.0;
        }

        // Association match
        if let Some(associations) = self.associations.get(trigger) {
            if associations.contains(&memory.content) {
                score += 0.5;
            }
        }

        // Recent recall bonus
        let recency_factor = (-memory.last_recall / 1000.0).exp();
        
        // Importance factor
        let importance_factor = memory.importance;
        
        // Recall frequency factor
        let frequency_factor = (memory.recall_count as f32 * 0.1).min(1.0);

        score * recency_factor * importance_factor * frequency_factor * self.recall_strength
    }

    fn update_memory_strength(&mut self, memory: &mut Memory, delta_time: f32) {
        // Decay importance over time
        let age = delta_time - memory.creation_time;
        let decay_factor = (-age / 10000.0).exp();
        
        memory.importance *= decay_factor;
    }

    fn create_associations(&mut self, content: &str) {
        // Create word-based associations
        let words: Vec<&str> = content.split_whitespace().collect();
        for word in words {
            self.associations
                .entry(word.to_string())
                .or_default()
                .push(content.to_string());
        }
    }

    fn record_recall(&mut self, memory_key: String, trigger: String, success: bool) {
        let event = RecallEvent {
            memory_key,
            trigger,
            success,
            recall_time: 0.0, // Current time should be passed
        };

        self.recent_recalls.push_back(event);
    }

    fn cleanup_recalls(&mut self) {
        while self.recent_recalls.len() > 100 {
            self.recent_recalls.pop_front();
        }
    }

    pub fn get_memory_stats(&self) -> MemoryStats {
        let total_memories = self.memories.len();
        let total_recalls: u32 = self.memories.values().map(|m| m.recall_count).sum();
        let avg_importance: f32 = self.memories.values().map(|m| m.importance).sum::<f32>() / total_memories as f32;

        MemoryStats {
            total_memories,
            total_recalls,
            avg_importance,
            recall_strength: self.recall_strength,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_memories: usize,
    pub total_recalls: u32,
    pub avg_importance: f32,
    pub recall_strength: f32,
}