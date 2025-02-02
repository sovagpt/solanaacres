use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesireSystem {
    desires: HashMap<String, Desire>,
    active_desires: HashSet<String>,
    desire_weights: HashMap<String, f32>,
    satisfaction_thresholds: HashMap<String, f32>,
    current_mood: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desire {
    name: String,
    intensity: f32,
    urgency: f32,
    satisfaction: f32,
    category: DesireCategory,
    last_update: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DesireCategory {
    Basic,       // Food, rest, etc.
    Social,      // Interaction, belonging
    Achievement, // Goals, success
    Growth,      // Learning, improvement
    Security,    // Safety, stability
}

impl DesireSystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update desire intensities
        for desire in self.desires.values_mut() {
            self.update_desire_intensity(desire, delta_time);
        }

        // Update active desires
        self.update_active_desires();

        // Update mood based on desire satisfaction
        self.update_mood();
    }

    pub fn add_desire(&mut self, name: String, category: DesireCategory, initial_intensity: f32) {
        let desire = Desire {
            name: name.clone(),
            intensity: initial_intensity,
            urgency: 0.0,
            satisfaction: 0.0,
            category,
            last_update: 0.0,
        };

        self.desires.insert(name.clone(), desire);
        self.desire_weights.insert(name.clone(), 1.0);
        self.satisfaction_thresholds.insert(name, 0.7);
    }

    pub fn satisfy_desire(&mut self, name: &str, amount: f32) {
        if let Some(desire) = self.desires.get_mut(name) {
            desire.satisfaction = (desire.satisfaction + amount).clamp(0.0, 1.0);
            desire.intensity *= 1.0 - amount.clamp(0.0, 1.0);
            desire.last_update = 0.0; // Current time should be passed
        }
    }

    pub fn get_strongest_desire(&self) -> Option<&Desire> {
        self.desires.values()
            .max_by(|a, b| {
                let a_priority = a.intensity * self.desire_weights.get(&a.name).unwrap_or(&1.0);
                let b_priority = b.intensity * self.desire_weights.get(&b.name).unwrap_or(&1.0);
                a_priority.partial_cmp(&b_priority).unwrap()
            })
    }

    pub fn get_active_desires(&self) -> Vec<&Desire> {
        self.active_desires.iter()
            .filter_map(|name| self.desires.get(name))
            .collect()
    }

    fn initialize_basic_desires(&mut self) {
        self.add_desire("Social Interaction".to_string(), DesireCategory::Social, 0.5);
        self.add_desire("Achievement".to_string(), DesireCategory::Achievement, 0.3);
        self.add_desire("Learning".to_string(), DesireCategory::Growth, 0.4);
        self.add_desire("Security".to_string(), DesireCategory::Security, 0.6);
    }

    fn update_desire_intensity(&mut self, desire: &mut Desire, delta_time: f32) {
        let base_increase = match desire.category {
            DesireCategory::Basic => 0.1,
            DesireCategory::Social => 0.05,
            DesireCategory::Achievement => 0.03,
            DesireCategory::Growth => 0.02,
            DesireCategory::Security => 0.04,
        };

        // Increase intensity based on time and category
        desire.intensity += base_increase * delta_time * (1.0 - desire.satisfaction);
        
        // Update urgency based on intensity and time since last update
        desire.urgency = desire.intensity * (desire.last_update / 100.0).min(1.0);

        // Clamp values
        desire.intensity = desire.intensity.clamp(0.0, 1.0);
        desire.urgency = desire.urgency.clamp(0.0, 1.0);
    }

    fn update_active_desires(&mut self) {
        self.active_desires.clear();

        for (name, desire) in &self.desires {
            let threshold = self.satisfaction_thresholds.get(name).unwrap_or(&0.7);
            if desire.intensity > *threshold {
                self.active_desires.insert(name.clone());
            }
        }
    }

    fn update_mood(&mut self) {
        let total_satisfaction: f32 = self.desires.values()
            .map(|d| d.satisfaction * self.desire_weights.get(&d.name).unwrap_or(&1.0))
            .sum();
        
        let total_weight: f32 = self.desire_weights.values().sum();
        
        self.current_mood = if total_weight > 0.0 {
            total_satisfaction / total_weight
        } else {
            0.5
        };
    }

    pub fn adjust_desire_weight(&mut self, name: &str, weight: f32) {
        if self.desires.contains_key(name) {
            self.desire_weights.insert(name.to_string(), weight.clamp(0.0, 2.0));
        }
    }

    pub fn get_mood(&self) -> f32 {
        self.current_mood
    }

    pub fn get_category_satisfaction(&self, category: &DesireCategory) -> f32 {
        let category_desires: Vec<_> = self.desires.values()
            .filter(|d| d.category == *category)
            .collect();

        if category_desires.is_empty() {
            return 1.0;
        }

        category_desires.iter()
            .map(|d| d.satisfaction)
            .sum::<f32>() / category_desires.len() as f32
    }
}