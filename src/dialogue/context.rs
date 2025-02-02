use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContext {
    variables: HashMap<String, String>,
    topic_stack: VecDeque<String>,
    context_history: VecDeque<ContextFrame>,
    current_focus: Option<String>,
    pending_questions: VecDeque<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFrame {
    topic: String,
    variables: HashMap<String, String>,
    importance: f32,
    timestamp: f32,
}

impl Default for DialogueContext {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            topic_stack: VecDeque::new(),
            context_history: VecDeque::with_capacity(10),
            current_focus: None,
            pending_questions: VecDeque::new(),
        }
    }
}

impl DialogueContext {
    pub fn update(&mut self, delta_time: f32) {
        // Update context frames
        self.update_context_frames(delta_time);
        
        // Clean up old contexts
        self.cleanup_old_contexts();
    }

    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn get_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    pub fn has_variable(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn push_topic(&mut self, topic: String) {
        // Save current context before pushing new topic
        self.save_current_context();
        
        self.topic_stack.push_back(topic);
    }

    pub fn pop_topic(&mut self) -> Option<String> {
        let popped = self.topic_stack.pop_back();
        
        // Restore previous context if available
        if let Some(previous_frame) = self.context_history.pop_back() {
            self.variables = previous_frame.variables;
            self.current_focus = Some(previous_frame.topic);
        }
        
        popped
    }

    pub fn get_current_topic(&self) -> Option<&String> {
        self.topic_stack.back()
    }

    pub fn add_pending_question(&mut self, question: String) {
        self.pending_questions.push_back(question);
    }

    pub fn get_next_question(&mut self) -> Option<String> {
        self.pending_questions.pop_front()
    }

    pub fn set_focus(&mut self, focus: String) {
        self.current_focus = Some(focus);
    }

    pub fn get_focus(&self) -> Option<&String> {
        self.current_focus.as_ref()
    }

    pub fn save_current_context(&mut self) {
        if let Some(current_topic) = self.get_current_topic() {
            let frame = ContextFrame {
                topic: current_topic.clone(),
                variables: self.variables.clone(),
                importance: self.calculate_importance(),
                timestamp: 0.0, // Current time should be passed
            };
            
            self.context_history.push_back(frame);
        }
    }

    fn calculate_importance(&self) -> f32 {
        let topic_importance = if let Some(topic) = self.get_current_topic() {
            match topic.as_str() {
                "personal" => 0.8,
                "urgent" => 0.9,
                "casual" => 0.3,
                _ => 0.5,
            }
        } else {
            0.5
        };

        let variable_importance = self.variables.len() as f32 * 0.1;
        let focus_importance = if self.current_focus.is_some() { 0.2 } else { 0.0 };

        (topic_importance + variable_importance + focus_importance).clamp(0.0, 1.0)
    }

    fn update_context_frames(&mut self, delta_time: f32) {
        for frame in &mut self.context_history {
            frame.timestamp += delta_time;
            
            // Decrease importance over time
            frame.importance *= 0.99f32.powf(delta_time);
        }
    }

    fn cleanup_old_contexts(&mut self) {
        // Remove contexts older than a certain threshold or with very low importance
        self.context_history.retain(|frame| {
            frame.timestamp < 1000.0 && frame.importance > 0.1
        });
        
        // Keep only the last 10 contexts
        while self.context_history.len() > 10 {
            self.context_history.pop_front();
        }
    }

    pub fn get_relevant_context(&self, topic: &str) -> Vec<&ContextFrame> {
        self.context_history
            .iter()
            .filter(|frame| {
                frame.topic == topic || frame.importance > 0.7
            })
            .collect()
    }

    pub fn merge_context(&mut self, other: &DialogueContext) {
        // Merge variables, keeping the most recent ones
        for (key, value) in &other.variables {
            if !self.variables.contains_key(key) {
                self.variables.insert(key.clone(), value.clone());
            }
        }

        // Merge pending questions
        for question in &other.pending_questions {
            if !self.pending_questions.contains(question) {
                self.pending_questions.push_back(question.clone());
            }
        }

        // Update focus if other context has one
        if let Some(focus) = &other.current_focus {
            self.current_focus = Some(focus.clone());
        }
    }
}