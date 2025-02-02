use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionSystem {
    sensory_inputs: HashMap<String, SensoryInput>,
    attention_focus: Option<String>,
    perceived_patterns: HashSet<String>,
    perception_filters: Vec<PerceptionFilter>,
    sensory_memory: Vec<PerceivedEvent>,
    clarity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryInput {
    input_type: String,
    intensity: f32,
    confidence: f32,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceivedEvent {
    content: String,
    source: String,
    intensity: f32,
    emotional_valence: f32,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionFilter {
    pattern: String,
    priority: f32,
    emotional_threshold: f32,
    active: bool,
}

impl Default for PerceptionSystem {
    fn default() -> Self {
        Self {
            sensory_inputs: HashMap::new(),
            attention_focus: None,
            perceived_patterns: HashSet::new(),
            perception_filters: Vec::new(),
            sensory_memory: Vec::new(),
            clarity: 0.8,
        }
    }
}

impl PerceptionSystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update sensory inputs
        self.update_sensory_inputs(delta_time);
        
        // Update attention focus
        self.update_attention();
        
        // Clean up old sensory memory
        self.cleanup_memory();
    }

    pub fn process_input(&mut self, input: &str) -> String {
        // Create sensory input
        let sensory_input = SensoryInput {
            input_type: "text".to_string(),
            intensity: 1.0,
            confidence: self.clarity,
            timestamp: 0.0,
        };

        self.sensory_inputs.insert(input.to_string(), sensory_input);

        // Apply perception filters
        let filtered_input = self.apply_filters(input);

        // Check for patterns
        let patterns = self.detect_patterns(&filtered_input);
        for pattern in patterns {
            self.perceived_patterns.insert(pattern);
        }

        // Store in sensory memory
        self.record_perception(&filtered_input);

        filtered_input
    }

    pub fn add_filter(&mut self, filter: PerceptionFilter) {
        self.perception_filters.push(filter);
    }

    pub fn set_attention_focus(&mut self, focus: Option<String>) {
        self.attention_focus = focus;
    }

    pub fn get_clarity(&self) -> f32 {
        self.clarity
    }

    fn update_sensory_inputs(&mut self, delta_time: f32) {
        // Decay sensory input intensities
        for input in self.sensory_inputs.values_mut() {
            input.intensity *= 0.95f32.powf(delta_time);
            input.timestamp += delta_time;
        }

        // Remove weak inputs
        self.sensory_inputs.retain(|_, input| input.intensity > 0.1);
    }

    fn update_attention(&mut self) {
        // Find strongest sensory input
        if let Some((strongest_input, _)) = self.sensory_inputs.iter()
            .max_by(|a, b| a.1.intensity.partial_cmp(&b.1.intensity).unwrap()) {
            self.attention_focus = Some(strongest_input.clone());
        }
    }

    fn apply_filters(&self, input: &str) -> String {
        let mut filtered = input.to_string();

        // Apply active filters in priority order
        let mut active_filters: Vec<_> = self.perception_filters.iter()
            .filter(|f| f.active)
            .collect();
        active_filters.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());

        for filter in active_filters {
            if input.contains(&filter.pattern) {
                filtered = filtered.replace(&filter.pattern, "");
            }
        }

        filtered
    }

    fn detect_patterns(&self, input: &str) -> Vec<String> {
        let mut patterns = Vec::new();

        // Simple pattern detection based on keywords
        let words: Vec<&str> = input.split_whitespace().collect();
        
        // Check for repetition patterns
        for window in words.windows(2) {
            if window[0] == window[1] {
                patterns.push(format!("repetition: {}", window[0]));
            }
        }

        // Check for emotional patterns
        if input.contains("happy") || input.contains("joy") {
            patterns.push("positive_emotion".to_string());
        }
        if input.contains("sad") || input.contains("angry") {
            patterns.push("negative_emotion".to_string());
        }

        // Check for question patterns
        if input.contains("?") {
            patterns.push("question".to_string());
        }

        patterns
    }

    fn record_perception(&mut self, content: &str) {
        let event = PerceivedEvent {
            content: content.to_string(),
            source: "text_input".to_string(),
            intensity: 1.0,
            emotional_valence: self.calculate_emotional_valence(content),
            timestamp: 0.0,
        };

        self.sensory_memory.push(event);
    }

    fn calculate_emotional_valence(&self, content: &str) -> f32 {
        let mut valence = 0.0;

        // Simple sentiment analysis
        for word in content.split_whitespace() {
            match word.to_lowercase().as_str() {
                "happy" | "good" | "great" | "excellent" => valence += 0.2,
                "sad" | "bad" | "terrible" | "awful" => valence -= 0.2,
                _ => {}
            }
        }

        valence.clamp(-1.0, 1.0)
    }

    fn cleanup_memory(&mut self) {
        // Keep only recent perceptions
        self.sensory_memory.retain(|event| event.timestamp < 100.0);
        
        // Limit memory size
        while self.sensory_memory.len() > 100 {
            self.sensory_memory.remove(0);
        }
    }

    pub fn get_recent_perceptions(&self, count: usize) -> Vec<&PerceivedEvent> {
        self.sensory_memory.iter()
            .rev()
            .take(count)
            .collect()
    }

    pub fn get_active_patterns(&self) -> &HashSet<String> {
        &self.perceived_patterns
    }
}