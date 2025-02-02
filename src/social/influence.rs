use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInfluence {
    influence_score: f32,
    influence_radius: f32,
    persuasion_power: f32,
    reputation: HashMap<String, f32>,
    influenced_npcs: HashMap<Uuid, InfluenceLevel>,
    social_status: SocialStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluenceLevel {
    strength: f32,
    duration: f32,
    nature: InfluenceNature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfluenceNature {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialStatus {
    Leader,
    Respected,
    Average,
    Outsider,
    Controversial,
}

impl SocialInfluence {
    pub fn can_influence(&self, target_influence: &SocialInfluence) -> bool {
        self.influence_score > target_influence.influence_score * 0.8
    }

    pub fn calculate_influence_success(&self, target_influence: &SocialInfluence, action_importance: f32) -> f32 {
        let relative_power = (self.influence_score - target_influence.influence_score + 1.0) / 2.0;
        let importance_factor = 1.0 - action_importance * 0.5;
        
        relative_power * importance_factor * self.persuasion_power
    }

    pub fn propagate_influence(&self, network: &HashMap<Uuid, SocialInfluence>, source: Uuid) -> Vec<(Uuid, f32)> {
        let mut propagation = Vec::new();
        
        for (target_id, target_influence) in network {
            if *target_id != source {
                let distance_factor = 1.0 / (1.0 + self.influence_radius);
                let strength = self.influence_score * distance_factor * self.get_compatibility(target_influence);
                
                if strength > 0.1 {
                    propagation.push((*target_id, strength));
                }
            }
        }
        
        propagation
    }

    pub fn get_compatibility(&self, other: &SocialInfluence) -> f32 {
        let status_compatibility = match (&self.social_status, &other.social_status) {
            (SocialStatus::Leader, _) => 0.8,
            (_, SocialStatus::Leader) => 0.6,
            (SocialStatus::Respected, SocialStatus::Respected) => 0.7,
            (SocialStatus::Average, SocialStatus::Average) => 0.5,
            _ => 0.3,
        };

        let reputation_similarity = self.calculate_reputation_similarity(&other.reputation);
        
        (status_compatibility + reputation_similarity) / 2.0
    }

    fn calculate_reputation_similarity(&self, other_reputation: &HashMap<String, f32>) -> f32 {
        let mut total_diff = 0.0;
        let mut count = 0;

        for (context, &value) in &self.reputation {
            if let Some(&other_value) = other_reputation.get(context) {
                total_diff += (value - other_value).abs();
                count += 1;
            }
        }

        if count == 0 {
            return 0.5; // Default similarity when no common reputation contexts
        }

        1.0 - (total_diff / count as f32)
    }

    pub fn merge_influences(&mut self, other: &SocialInfluence, weight: f32) {
        // Merge influence scores
        self.influence_score = self.influence_score * (1.0 - weight) + other.influence_score * weight;
        
        // Merge reputations
        for (context, &value) in &other.reputation {
            let current = self.reputation.entry(context.clone()).or_insert(0.0);
            *current = *current * (1.0 - weight) + value * weight;
        }
        
        // Adjust social status if necessary
        if weight > 0.5 {
            self.social_status = other.social_status.clone();
        }
    }

    pub fn handle_resistance(&mut self, resistance_strength: f32) {
        self.influence_score *= 1.0 - resistance_strength * 0.2;
        self.persuasion_power *= 1.0 - resistance_strength * 0.1;
        
        // Ensure values don't go below minimum thresholds
        self.influence_score = self.influence_score.max(0.1);
        self.persuasion_power = self.persuasion_power.max(0.1);
    }

    pub fn calculate_total_influence(&self) -> f32 {
        let base_influence = self.influence_score;
        let reputation_modifier = self.reputation.values().sum::<f32>() / self.reputation.len().max(1) as f32;
        let network_modifier = self.influenced_npcs.values()
            .map(|level| match level.nature {
                InfluenceNature::Positive => level.strength,
                InfluenceNature::Negative => -level.strength,
                InfluenceNature::Neutral => 0.0,
            })
            .sum::<f32>() / self.influenced_npcs.len().max(1) as f32;

        (base_influence * 0.4 + reputation_modifier * 0.3 + network_modifier * 0.3).clamp(0.0, 1.0)
    }
}