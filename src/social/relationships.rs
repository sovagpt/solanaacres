use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    relationship_type: RelationshipType,
    strength: f32,
    trust: f32,
    history: Vec<Interaction>,
    shared_experiences: Vec<String>,
    emotional_bond: f32,
    last_interaction: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipType {
    Friend,
    Enemy,
    Acquaintance,
    Family,
    Mentor,
    Student,
    Rival,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Interaction {
    interaction_type: String,
    impact: f32,
    timestamp: f32,
}

impl Relationship {
    pub fn new(relationship_type: RelationshipType) -> Self {
        Self {
            relationship_type,
            strength: match relationship_type {
                RelationshipType::Family => 0.8,
                RelationshipType::Friend => 0.6,
                RelationshipType::Mentor | RelationshipType::Student => 0.5,
                RelationshipType::Acquaintance => 0.3,
                RelationshipType::Rival => -0.3,
                RelationshipType::Enemy => -0.6,
                RelationshipType::Neutral => 0.0,
            },
            trust: match relationship_type {
                RelationshipType::Family => 0.9,
                RelationshipType::Friend => 0.7,
                RelationshipType::Mentor | RelationshipType::Student => 0.6,
                RelationshipType::Acquaintance => 0.4,
                RelationshipType::Rival => 0.2,
                RelationshipType::Enemy => 0.1,
                RelationshipType::Neutral => 0.5,
            },
            history: Vec::new(),
            shared_experiences: Vec::new(),
            emotional_bond: 0.0,
            last_interaction: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.last_interaction += delta_time;
        
        // Decay relationship strength over time if no interactions
        if self.last_interaction > 100.0 {
            self.strength *= 0.99;
            self.trust *= 0.99;
        }
    }

    pub fn add_interaction(&mut self, interaction_type: String, impact: f32, timestamp: f32) {
        self.history.push(Interaction {
            interaction_type,
            impact,
            timestamp,
        });

        // Update relationship metrics based on interaction
        self.strength = (self.strength + impact * 0.2).clamp(-1.0, 1.0);
        self.trust = (self.trust + impact * 0.1).clamp(0.0, 1.0);
        self.last_interaction = 0.0;

        // Potentially change relationship type based on new strength
        self.update_relationship_type();
    }

    pub fn add_shared_experience(&mut self, experience: String) {
        self.shared_experiences.push(experience);
        self.emotional_bond = (self.emotional_bond + 0.1).clamp(0.0, 1.0);
    }

    pub fn get_relationship_score(&self) -> f32 {
        let recency_factor = (-self.last_interaction / 1000.0).exp();
        let history_factor = self.calculate_history_factor();
        
        (self.strength * 0.4 + 
         self.trust * 0.3 + 
         self.emotional_bond * 0.2 + 
         history_factor * 0.1) * recency_factor
    }

    fn calculate_history_factor(&self) -> f32 {
        if self.history.is_empty() {
            return 0.0;
        }

        let total_impact: f32 = self.history.iter()
            .map(|interaction| interaction.impact)
            .sum();

        (total_impact / self.history.len() as f32).clamp(-1.0, 1.0)
    }

    fn update_relationship_type(&mut self) {
        let score = self.get_relationship_score();
        
        self.relationship_type = match score {
            s if s > 0.8 => RelationshipType::Friend,
            s if s > 0.4 => RelationshipType::Acquaintance,
            s if s > -0.2 => RelationshipType::Neutral,
            s if s > -0.6 => RelationshipType::Rival,
            _ => RelationshipType::Enemy,
        };
    }

    pub fn get_trust_level(&self) -> f32 {
        self.trust
    }

    pub fn get_shared_history(&self) -> &[String] {
        &self.shared_experiences
    }

    pub fn get_recent_interactions(&self, count: usize) -> Vec<&Interaction> {
        self.history.iter()
            .rev()
            .take(count)
            .collect()
    }
}