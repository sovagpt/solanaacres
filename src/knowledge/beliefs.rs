use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefSystem {
    beliefs: HashMap<String, Belief>,
    core_beliefs: Vec<String>,
    belief_conflicts: Vec<(String, String, f32)>,
    belief_stability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    content: String,
    strength: f32,
    evidence: Vec<Evidence>,
    challenges: Vec<Challenge>,
    last_update: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Evidence {
    description: String,
    strength: f32,
    source: String,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Challenge {
    description: String,
    impact: f32,
    resolution: Option<bool>,
    timestamp: f32,
}

impl Default for BeliefSystem {
    fn default() -> Self {
        Self {
            beliefs: HashMap::new(),
            core_beliefs: Vec::new(),
            belief_conflicts: Vec::new(),
            belief_stability: 1.0,
        }
    }
}

impl BeliefSystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update belief strengths based on evidence and challenges
        for belief in self.beliefs.values_mut() {
            self.update_belief_strength(belief, delta_time);
        }

        // Check for and resolve conflicts
        self.resolve_conflicts();

        // Update overall stability
        self.update_stability();
    }

    pub fn add_belief(&mut self, content: String, initial_strength: f32, is_core: bool) {
        let belief = Belief {
            content: content.clone(),
            strength: initial_strength,
            evidence: Vec::new(),
            challenges: Vec::new(),
            last_update: 0.0,
        };

        self.beliefs.insert(content.clone(), belief);
        
        if is_core {
            self.core_beliefs.push(content);
        }
    }

    pub fn add_evidence(&mut self, belief_content: &str, evidence: Evidence) {
        if let Some(belief) = self.beliefs.get_mut(belief_content) {
            belief.evidence.push(evidence);
            self.update_belief_strength(belief, 0.0);
        }
    }

    pub fn challenge_belief(&mut self, belief_content: &str, challenge: Challenge) {
        if let Some(belief) = self.beliefs.get_mut(belief_content) {
            belief.challenges.push(challenge);
            self.update_belief_strength(belief, 0.0);
        }
    }

    pub fn get_belief_strength(&self, content: &str) -> Option<f32> {
        self.beliefs.get(content).map(|b| b.strength)
    }

    pub fn is_belief_stable(&self, content: &str) -> bool {
        if let Some(belief) = self.beliefs.get(content) {
            belief.strength > 0.7 && belief.challenges.is_empty()
        } else {
            false
        }
    }

    fn update_belief_strength(&mut self, belief: &mut Belief, delta_time: f32) {
        // Calculate evidence strength
        let evidence_strength: f32 = belief.evidence
            .iter()
            .map(|e| e.strength)
            .sum::<f32>() / belief.evidence.len().max(1) as f32;

        // Calculate challenge impact
        let challenge_impact: f32 = belief.challenges
            .iter()
            .filter(|c| c.resolution.is_none())
            .map(|c| c.impact)
            .sum::<f32>() / belief.challenges.len().max(1) as f32;

        // Update strength based on evidence and challenges
        let target_strength = evidence_strength * (1.0 - challenge_impact);
        belief.strength = (belief.strength * 0.9 + target_strength * 0.1).clamp(0.0, 1.0);
        belief.last_update = 0.0; // Should be current time
    }

    fn resolve_conflicts(&mut self) {
        let mut resolved_conflicts = Vec::new();

        for (belief1, belief2, impact) in &self.belief_conflicts {
            if let (Some(b1), Some(b2)) = (self.beliefs.get(belief1), self.beliefs.get(belief2)) {
                if b1.strength > 0.8 && b2.strength > 0.8 {
                    // Strong conflict between strong beliefs
                    resolved_conflicts.push((belief1.clone(), belief2.clone(), *impact));
                }
            }
        }

        // Handle resolved conflicts
        for (b1, b2, impact) in resolved_conflicts {
            if let (Some(belief1), Some(belief2)) = (self.beliefs.get_mut(&b1), self.beliefs.get_mut(&b2)) {
                belief1.strength *= 1.0 - impact;
                belief2.strength *= 1.0 - impact;
            }
        }
    }

    fn update_stability(&mut self) {
        let total_beliefs = self.beliefs.len() as f32;
        if total_beliefs == 0.0 {
            self.belief_stability = 1.0;
            return;
        }

        let stable_beliefs = self.beliefs.values()
            .filter(|b| b.strength > 0.7 && b.challenges.is_empty())
            .count() as f32;

        self.belief_stability = stable_beliefs / total_beliefs;
    }
}