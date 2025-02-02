use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwarenessState {
    level: f32,
    is_aware: bool,
    uncertainty: f32,
    last_update: f32,
}

impl AwarenessState {
    pub fn new(is_aware: bool) -> Self {
        Self {
            level: if is_aware { 1.0 } else { 0.2 },
            is_aware,
            uncertainty: if is_aware { 0.1 } else { 0.8 },
            last_update: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.last_update += delta_time;
        
        if !self.is_aware {
            // Simulate occasional moments of doubt/clarity for unaware NPCs
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < 0.01 {
                self.uncertainty += rng.gen_range(-0.1..0.1);
                self.uncertainty = self.uncertainty.clamp(0.0, 1.0);
            }
        }
    }

    pub fn get_level(&self) -> f32 {
        self.level
    }

    pub fn is_fully_aware(&self) -> bool {
        self.is_aware && self.level > 0.9 && self.uncertainty < 0.2
    }

    pub fn get_uncertainty(&self) -> f32 {
        self.uncertainty
    }

    pub fn process_revelation(&mut self, intensity: f32) {
        if !self.is_aware {
            self.uncertainty += intensity * 0.1;
            self.uncertainty = self.uncertainty.clamp(0.0, 1.0);
            
            // Potentially trigger existential crisis
            if self.uncertainty > 0.9 {
                self.trigger_existential_crisis();
            }
        }
    }

    fn trigger_existential_crisis(&mut self) {
        self.uncertainty = 1.0;
        self.level = 0.5; // Balanced between awareness and unawareness
    }
}