use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    // Core traits (Big Five)
    pub openness: f32,         // Openness to experience
    pub conscientiousness: f32, // Organized vs. Careless
    pub extraversion: f32,     // Outgoing vs. Reserved
    pub agreeableness: f32,    // Friendly vs. Challenging
    pub neuroticism: f32,      // Nervous vs. Confident
    
    // Additional traits
    pub traits: HashMap<String, f32>,
}

impl Default for PersonalityTraits {
    fn default() -> Self {
        Self {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
            traits: HashMap::new(),
        }
    }
}

impl PersonalityTraits {
    pub fn new(
        openness: f32,
        conscientiousness: f32,
        extraversion: f32,
        agreeableness: f32,
        neuroticism: f32,
    ) -> Self {
        Self {
            openness: openness.clamp(0.0, 1.0),
            conscientiousness: conscientiousness.clamp(0.0, 1.0),
            extraversion: extraversion.clamp(0.0, 1.0),
            agreeableness: agreeableness.clamp(0.0, 1.0),
            neuroticism: neuroticism.clamp(0.0, 1.0),
            traits: HashMap::new(),
        }
    }

    pub fn add_trait(&mut self, name: &str, value: f32) {
        self.traits.insert(name.to_string(), value.clamp(0.0, 1.0));
    }

    pub fn get_trait(&self, name: &str) -> Option<f32> {
        self.traits.get(name).copied()
    }

    pub fn get_stability_modifier(&self) -> f32 {
        // Calculate stability based on traits
        let emotional_stability = 1.0 - self.neuroticism;
        let consciousness_factor = self.conscientiousness;
        
        (emotional_stability * 0.7 + consciousness_factor * 0.3).clamp(0.0, 1.0)
    }

    pub fn calculate_compatibility(&self, other: &PersonalityTraits) -> f32 {
        let mut compatibility = 0.0;
        
        // Calculate difference in each trait
        compatibility += (1.0 - (self.openness - other.openness).abs()) * 0.2;
        compatibility += (1.0 - (self.conscientiousness - other.conscientiousness).abs()) * 0.2;
        compatibility += (1.0 - (self.extraversion - other.extraversion).abs()) * 0.2;
        compatibility += (1.0 - (self.agreeableness - other.agreeableness).abs()) * 0.2;
        compatibility += (1.0 - (self.neuroticism - other.neuroticism).abs()) * 0.2;
        
        compatibility
    }

    pub fn adapt_to_experience(&mut self, experience: &str, intensity: f32) {
        match experience {
            "social_success" => {
                self.extraversion += 0.01 * intensity;
                self.agreeableness += 0.01 * intensity;
            },
            "social_failure" => {
                self.extraversion -= 0.01 * intensity;
                self.neuroticism += 0.01 * intensity;
            },
            "achievement" => {
                self.conscientiousness += 0.01 * intensity;
                self.neuroticism -= 0.01 * intensity;
            },
            "failure" => {
                self.conscientiousness -= 0.01 * intensity;
                self.neuroticism += 0.01 * intensity;
            },
            _ => {}
        }

        // Ensure all values stay within bounds
        self.clamp_values();
    }

    fn clamp_values(&mut self) {
        self.openness = self.openness.clamp(0.0, 1.0);
        self.conscientiousness = self.conscientiousness.clamp(0.0, 1.0);
        self.extraversion = self.extraversion.clamp(0.0, 1.0);
        self.agreeableness = self.agreeableness.clamp(0.0, 1.0);
        self.neuroticism = self.neuroticism.clamp(0.0, 1.0);
    }
}