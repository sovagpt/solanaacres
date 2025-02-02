use serde::{Serialize, Deserialize};
use super::awareness::AwarenessState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityPerception {
    distortion_level: f32,
    belief_stability: f32,
    reality_anchors: Vec<RealityAnchor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RealityAnchor {
    belief: String,
    strength: f32,
    last_reinforced: f32,
}

impl Default for RealityPerception {
    fn default() -> Self {
        Self {
            distortion_level: 0.0,
            belief_stability: 1.0,
            reality_anchors: Vec::new(),
        }
    }
}

impl RealityPerception {
    pub fn update(&mut self, awareness: &AwarenessState) {
        // Update distortion based on awareness
        let target_distortion = 1.0 - awareness.get_level();
        self.distortion_level += (target_distortion - self.distortion_level) * 0.1;
        
        // Update belief stability
        self.belief_stability = 1.0 - awareness.get_uncertainty();
        
        // Update reality anchors
        self.update_anchors();
    }

    pub fn add_anchor(&mut self, belief: String, strength: f32) {
        self.reality_anchors.push(RealityAnchor {
            belief,
            strength,
            last_reinforced: 0.0,
        });
    }

    pub fn get_distortion(&self) -> f32 {
        self.distortion_level
    }

    pub fn get_stability(&self) -> f32 {
        self.belief_stability
    }

    pub fn process_observation(&mut self, observation: &str, reliability: f32) {
        // Process new observation and its impact on reality perception
        let impact = reliability * (1.0 - self.distortion_level);
        self.belief_stability = (self.belief_stability + impact) / 2.0;

        // Add or reinforce reality anchor
        if reliability > 0.7 {
            self.add_anchor(observation.to_string(), reliability);
        }
    }

    fn update_anchors(&mut self) {
        // Decay old anchors
        self.reality_anchors.retain_mut(|anchor| {
            anchor.strength *= 0.99;
            anchor.strength > 0.1
        });
    }
}