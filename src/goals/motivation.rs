use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotivationSystem {
    motivations: HashMap<Uuid, Motivation>,
    base_motivation: f32,
    motivation_decay_rate: f32,
    current_focus: Option<Uuid>,
    energy_level: f32,
    reinforcement_history: Vec<MotivationEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motivation {
    goal_id: Uuid,
    strength: f32,
    source: MotivationSource,
    factors: MotivationFactors,
    last_update: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotivationFactors {
    importance: f32,
    urgency: f32,
    confidence: f32,
    interest: f32,
    social_pressure: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotivationSource {
    Internal,
    External,
    Social,
    Achievement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MotivationEvent {
    goal_id: Uuid,
    impact: f32,
    timestamp: f32,
    event_type: MotivationEventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MotivationEventType {
    Success,
    Failure,
    Progress,
    Feedback,
}

impl Default for MotivationSystem {
    fn default() -> Self {
        Self {
            motivations: HashMap::new(),
            base_motivation: 0.5,
            motivation_decay_rate: 0.1,
            current_focus: None,
            energy_level: 1.0,
            reinforcement_history: Vec::new(),
        }
    }
}

impl MotivationSystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update energy level
        self.update_energy(delta_time);

        // Update all motivations
        for motivation in self.motivations.values_mut() {
            self.update_motivation(motivation, delta_time);
        }

        // Update focus based on strongest motivation
        self.update_focus();
    }

    pub fn calculate_initial_motivation(&self, priority: f32) -> f32 {
        let base = self.base_motivation;
        let energy_factor = self.energy_level;
        
        (base * priority * energy_factor).clamp(0.0, 1.0)
    }

    pub fn calculate_current_motivation(
        &self,
        priority: f32,
        progress: f32,
        delta_time: f32
    ) -> f32 {
        let base_motivation = self.calculate_initial_motivation(priority);
        let progress_factor = 1.0 - (progress * 0.5); // Motivation decreases as progress increases
        let time_decay = (-self.motivation_decay_rate * delta_time).exp();

        (base_motivation * progress_factor * time_decay).clamp(0.0, 1.0)
    }

    pub fn add_motivation(&mut self, goal_id: Uuid, source: MotivationSource, factors: MotivationFactors) {
        let motivation = Motivation {
            goal_id,
            strength: self.calculate_motivation_strength(&factors),
            source,
            factors,
            last_update: 0.0,
        };

        self.motivations.insert(goal_id, motivation);
    }

    pub fn record_event(&mut self, goal_id: Uuid, event_type: MotivationEventType, impact: f32) {
        let event = MotivationEvent {
            goal_id,
            impact,
            timestamp: 0.0, // Current time should be passed
            event_type,
        };

        self.reinforcement_history.push(event);
        
        // Update motivation based on event
        if let Some(motivation) = self.motivations.get_mut(&goal_id) {
            match event_type {
                MotivationEventType::Success => {
                    motivation.factors.confidence += impact * 0.2;
                    self.base_motivation += impact * 0.1;
                },
                MotivationEventType::Failure => {
                    motivation.factors.confidence -= impact * 0.2;
                    self.base_motivation -= impact * 0.1;
                },
                MotivationEventType::Progress => {
                    motivation.strength += impact * 0.1;
                },
                MotivationEventType::Feedback => {
                    motivation.factors.social_pressure += impact * 0.15;
                },
            }
            
            // Clamp values
            motivation.factors.confidence = motivation.factors.confidence.clamp(0.0, 1.0);
            motivation.strength = motivation.strength.clamp(0.0, 1.0);
            self.base_motivation = self.base_motivation.clamp(0.1, 1.0);
        }
    }

    fn update_energy(&mut self, delta_time: f32) {
        // Energy slowly regenerates over time
        self.energy_level += delta_time * 0.01;
        
        // Decrease energy based on number of active motivations
        let active_motivations = self.motivations.len() as f32;
        self.energy_level -= delta_time * 0.005 * active_motivations;
        
        self.energy_level = self.energy_level.clamp(0.1, 1.0);
    }

    fn update_motivation(&mut self, motivation: &mut Motivation, delta_time: f32) {
        // Apply time-based decay
        motivation.strength *= (-self.motivation_decay_rate * delta_time).exp();
        
        // Update factors
        motivation.factors.urgency += delta_time * 0.01;
        motivation.factors.interest *= 0.999f32.powf(delta_time);
        
        // Clamp values
        motivation.factors.urgency = motivation.factors.urgency.clamp(0.0, 1.0);
        motivation.factors.interest = motivation.factors.interest.clamp(0.0, 1.0);
        motivation.strength = motivation.strength.clamp(0.0, 1.0);
        
        motivation.last_update += delta_time;
    }

    fn update_focus(&mut self) {
        self.current_focus = self.motivations.iter()
            .max_by(|a, b| a.1.strength.partial_cmp(&b.1.strength).unwrap())
            .map(|(id, _)| *id);
    }

    fn calculate_motivation_strength(&self, factors: &MotivationFactors) -> f32 {
        let importance_weight = 0.3;
        let urgency_weight = 0.2;
        let confidence_weight = 0.2;
        let interest_weight = 0.15;
        let social_weight = 0.15;

        let strength = 
            factors.importance * importance_weight +
            factors.urgency * urgency_weight +
            factors.confidence * confidence_weight +
            factors.interest * interest_weight +
            factors.social_pressure * social_weight;

        strength.clamp(0.0, 1.0)
    }
}