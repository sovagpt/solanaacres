use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySystem {
    short_term: VecDeque<Memory>,
    long_term: Vec<Memory>,
    capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    content: String,
    importance: f32,
    timestamp: f32,
    emotional_value: f32,
}

impl MemorySystem {
    pub fn new() -> Self {
        Self {
            short_term: VecDeque::with_capacity(10),
            long_term: Vec::new(),
            capacity: 100,
        }
    }

    pub fn add_memory(&mut self, content: String, importance: f32, emotional_value: f32) {
        let memory = Memory {
            content,
            importance,
            timestamp: 0.0,
            emotional_value,
        };

        if importance > 0.7 {
            self.long_term.push(memory);
        } else {
            self.short_term.push_back(memory);
        }
    }
}