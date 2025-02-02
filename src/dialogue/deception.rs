use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeceptionSystem {
    known_truths: HashMap<String, Truth>,
    active_deceptions: HashMap<Uuid, Deception>,
    deception_history: Vec<DeceptionEvent>,
    trust_levels: HashMap<Uuid, f32>,
    deception_threshold: f32,
    conscience_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Truth {
    content: String,
    importance: f32,
    protected: bool,
    known_by: HashSet<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deception {
    id: Uuid,
    truth: String,
    lie: String,
    target: Option<Uuid>,
    motivation: DeceptionMotivation,
    success_rate: f32,
    active_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeceptionEvent {
    deception_id: Uuid,
    success: bool,
    detected: bool,
    consequence: Option<String>,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeceptionMotivation {
    Protection,
    SocialHarmony,
    PersonalGain,
    AvoidConflict,
    MaintainTrust,
}

impl Default for DeceptionSystem {
    fn default() -> Self {
        Self {
            known_truths: HashMap::new(),
            active_deceptions: HashMap::new(),
            deception_history: Vec::new(),
            trust_levels: HashMap::new(),
            deception_threshold: 0.7,
            conscience_level: 0.5,
        }
    }
}

impl DeceptionSystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update active deceptions
        for deception in self.active_deceptions.values_mut() {
            deception.active_time += delta_time;
            
            // Decrease success rate over time
            deception.success_rate *= 0.99f32.powf(delta_time);
        }
        
        // Clean up old deceptions with very low success rates
        self.cleanup_deceptions();
    }

    pub fn should_deceive(&self, content: &str) -> f32 {
        let mut deception_score = 0.0;

        // Check if content relates to any protected truths
        for truth in self.known_truths.values() {
            if content.contains(&truth.content) && truth.protected {
                deception_score += truth.importance;
            }
        }

        // Apply conscience modifier
        deception_score *= 1.0 - self.conscience_level;

        // Return deception level if above threshold
        if deception_score > self.deception_threshold {
            deception_score
        } else {
            0.0
        }
    }

    pub fn create_deception(&mut self, truth: String, lie: String, target: Option<Uuid>, motivation: DeceptionMotivation) -> Uuid {
        let deception = Deception {
            id: Uuid::new_v4(),
            truth,
            lie,
            target,
            motivation,
            success_rate: 1.0,
            active_time: 0.0,
        };

        let id = deception.id;
        self.active_deceptions.insert(id, deception);
        id
    }

    pub fn add_truth(&mut self, content: String, importance: f32, protected: bool) {
        let truth = Truth {
            content,
            importance,
            protected,
            known_by: HashSet::new(),
        };

        self.known_truths.insert(truth.content.clone(), truth);
    }

    pub fn record_deception_result(&mut self, deception_id: Uuid, success: bool, detected: bool, consequence: Option<String>) {
        let event = DeceptionEvent {
            deception_id,
            success,
            detected,
            consequence,
            timestamp: 0.0, // Current time should be passed
        };

        self.deception_history.push(event);

        // Update success rate if deception is still active
        if let Some(deception) = self.active_deceptions.get_mut(&deception_id) {
            let impact = if success { 0.1 } else { -0.2 };
            deception.success_rate = (deception.success_rate + impact).clamp(0.0, 1.0);

            // If detected, update trust levels
            if detected {
                if let Some(target) = deception.target {
                    self.adjust_trust(target, -0.2);
                }
            }
        }
    }

    pub fn get_alternative_response(&self, truth: &str, deception_level: f32) -> Option<String> {
        // Find active deception for this truth
        for deception in self.active_deceptions.values() {
            if deception.truth == truth && deception.success_rate > 0.3 {
                return Some(deception.lie.clone());
            }
        }

        // Generate a new deceptive response if none exists
        if deception_level > self.deception_threshold {
            Some(self.generate_deceptive_response(truth))
        } else {
            None
        }
    }

    fn generate_deceptive_response(&self, truth: &str) -> String {
        // Simple response generation - could be made more sophisticated
        match self.conscience_level {
            x if x > 0.7 => format!("I'd rather not discuss {}", truth),
            x if x > 0.4 => format!("I'm not sure about {}", truth),
            _ => format!("I don't know anything about {}", truth),
        }
    }

    fn cleanup_deceptions(&mut self) {
        self.active_deceptions.retain(|_, deception| {
            deception.success_rate > 0.2 && deception.active_time < 1000.0
        });
    }

    fn adjust_trust(&mut self, target: Uuid, change: f32) {
        let trust = self.trust_levels.entry(target).or_insert(0.5);
        *trust = (*trust + change).clamp(0.0, 1.0);
    }

    pub fn get_trust_level(&self, target: Uuid) -> f32 {
        *self.trust_levels.get(&target).unwrap_or(&0.5)
    }

    pub fn set_conscience_level(&mut self, level: f32) {
        self.conscience_level = level.clamp(0.0, 1.0);
    }
}