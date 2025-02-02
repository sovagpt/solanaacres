use serde::{Serialize, Deserialize};

pub mod traits;
pub mod emotions;
pub mod behavior;
pub mod generation;

use traits::PersonalityTraits;
use emotions::{EmotionalState, Emotion};
use behavior::BehaviorProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub traits: PersonalityTraits,
    pub emotional_state: EmotionalState,
    pub behavior_profile: BehaviorProfile,
    pub stability: f32,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            traits: PersonalityTraits::default(),
            emotional_state: EmotionalState::default(),
            behavior_profile: BehaviorProfile::default(),
            stability: 0.5,
        }
    }
}

impl Personality {
    pub fn generate() -> Self {
        generation::generate_personality()
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update emotional state
        self.emotional_state.update(delta_time);
        
        // Update behavior based on current state
        self.behavior_profile.update(&self.traits, &self.emotional_state);
        
        // Adjust stability based on emotional changes
        self.update_stability();
    }

    pub fn react_to_event(&mut self, event: &str, intensity: f32) {
        // Generate emotional response
        let emotion = self.emotional_state.generate_response(event, intensity);
        
        // Modify behavior based on emotion
        self.behavior_profile.adapt_to_emotion(&emotion);
        
        // Update personality stability
        self.stability = (self.stability + self.calculate_stability_impact(intensity)) / 2.0;
    }

    fn update_stability(&mut self) {
        let emotional_impact = self.emotional_state.get_volatility();
        self.stability = (self.stability * 0.9 + (1.0 - emotional_impact) * 0.1).clamp(0.0, 1.0);
    }

    fn calculate_stability_impact(&self, intensity: f32) -> f32 {
        let base_impact = 1.0 - intensity;
        let trait_modifier = self.traits.get_stability_modifier();
        (base_impact * trait_modifier).clamp(0.0, 1.0)
    }
}