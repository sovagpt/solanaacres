use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    current_emotion: String,
    emotion_intensity: f32,
    emotion_history: VecDeque<EmotionEvent>,
    emotional_traits: HashMap<String, f32>,
    mood: f32,
    empathy_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionEvent {
    emotion: String,
    intensity: f32,
    trigger: String,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalResponse {
    base_text: String,
    emotional_modifiers: Vec<EmotionalModifier>,
    intensity_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalModifier {
    emotion: String,
    text_modification: String,
    intensity_threshold: f32,
}

impl Default for EmotionalState {
    fn default() -> Self {
        let mut emotional_traits = HashMap::new();
        emotional_traits.insert("stability".to_string(), 0.5);
        emotional_traits.insert("volatility".to_string(), 0.3);
        emotional_traits.insert("expressiveness".to_string(), 0.6);

        Self {
            current_emotion: "neutral".to_string(),
            emotion_intensity: 0.5,
            emotion_history: VecDeque::with_capacity(100),
            emotional_traits,
            mood: 0.5,
            empathy_level: 0.5,
        }
    }
}

impl EmotionalState {
    pub fn update(&mut self, delta_time: f32) {
        // Decay emotional intensity over time
        self.decay_emotion(delta_time);
        
        // Update mood based on recent emotional history
        self.update_mood();
        
        // Clean up old emotion events
        self.cleanup_history();
    }

    pub fn process_emotion(&mut self, emotion: String, intensity: f32, trigger: String) {
        let event = EmotionEvent {
            emotion: emotion.clone(),
            intensity,
            trigger,
            timestamp: 0.0, // Current time should be passed
        };

        self.emotion_history.push_back(event);
        
        // Update current emotion if new intensity is higher
        if intensity > self.emotion_intensity {
            self.current_emotion = emotion;
            self.emotion_intensity = intensity;
        }
    }

    pub fn modify_response(&self, base_response: String) -> EmotionalResponse {
        let mut modifiers = Vec::new();
        
        // Add emotion-specific modifiers
        match self.current_emotion.as_str() {
            "happy" => modifiers.push(EmotionalModifier {
                emotion: "happy".to_string(),
                text_modification: "!".to_string(),
                intensity_threshold: 0.5,
            }),
            "sad" => modifiers.push(EmotionalModifier {
                emotion: "sad".to_string(),
                text_modification: "...".to_string(),
                intensity_threshold: 0.3,
            }),
            "angry" => modifiers.push(EmotionalModifier {
                emotion: "angry".to_string(),
                text_modification: "!".to_string(),
                intensity_threshold: 0.6,
            }),
            _ => {}
        }

        EmotionalResponse {
            base_text: base_response,
            emotional_modifiers: modifiers,
            intensity_level: self.emotion_intensity,
        }
    }

    pub fn get_current_emotion(&self) -> String {
        self.current_emotion.clone()
    }

    pub fn get_empathetic_response(&self, target_emotion: &str) -> String {
        let empathy_factor = self.empathy_level;
        let response = match target_emotion {
            "happy" => if empathy_factor > 0.5 {
                "I'm glad to hear that!"
            } else {
                "That's good."
            },
            "sad" => if empathy_factor > 0.5 {
                "I'm sorry you're feeling that way..."
            } else {
                "That must be difficult."
            },
            "angry" => if empathy_factor > 0.5 {
                "I understand why you'd feel that way."
            } else {
                "I see."
            },
            _ => "I understand.",
        };

        response.to_string()
    }

    fn decay_emotion(&mut self, delta_time: f32) {
        let stability = self.emotional_traits.get("stability").unwrap_or(&0.5);
        let decay_rate = 0.1 * (1.0 - stability);
        
        self.emotion_intensity *= (1.0 - decay_rate * delta_time).max(0.0);
        
        if self.emotion_intensity < 0.1 {
            self.current_emotion = "neutral".to_string();
            self.emotion_intensity = 0.0;
        }
    }

    fn update_mood(&mut self) {
        if self.emotion_history.is_empty() {
            return;
        }

        // Calculate average emotion valence from recent history
        let recent_emotions: Vec<_> = self.emotion_history.iter().take(10).collect();
        let mut mood_sum = 0.0;

        for event in recent_emotions.iter() {
            mood_sum += match event.emotion.as_str() {
                "happy" | "excited" | "content" => 1.0,
                "sad" | "angry" | "frustrated" => -1.0,
                _ => 0.0,
            } * event.intensity;
        }

        let target_mood = (mood_sum / recent_emotions.len() as f32).clamp(-1.0, 1.0);
        self.mood += (target_mood - self.mood) * 0.1;
    }

    fn cleanup_history(&mut self) {
        while self.emotion_history.len() > 100 {
            self.emotion_history.pop_front();
        }
    }

    pub fn adjust_trait(&mut self, trait_name: &str, value: f32) {
        if let Some(trait_value) = self.emotional_traits.get_mut(trait_name) {
            *trait_value = value.clamp(0.0, 1.0);
        }
    }

    pub fn set_empathy_level(&mut self, level: f32) {
        self.empathy_level = level.clamp(0.0, 1.0);
    }
}