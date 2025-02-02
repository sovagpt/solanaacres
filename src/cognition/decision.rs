use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMaker {
    decision_history: Vec<Decision>,
    decision_weights: HashMap<String, f32>,
    current_strategy: DecisionStrategy,
    uncertainty_threshold: f32,
    processing_load: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    options: Vec<String>,
    chosen: String,
    context: HashMap<String, f32>,
    confidence: f32,
    outcome: Option<DecisionOutcome>,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOutcome {
    success: bool,
    impact: f32,
    feedback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionStrategy {
    Rational,
    Intuitive,
    RiskAverse,
    RiskSeeking,
    Balanced,
}

impl Default for DecisionMaker {
    fn default() -> Self {
        Self {
            decision_history: Vec::new(),
            decision_weights: HashMap::new(),
            current_strategy: DecisionStrategy::Balanced,
            uncertainty_threshold: 0.3,
            processing_load: 0.0,
        }
    }
}

impl DecisionMaker {
    pub fn update(&mut self, delta_time: f32) {
        // Decay processing load
        self.processing_load *= 0.95f32.powf(delta_time);
        
        // Update weights based on decision outcomes
        self.update_weights();
        
        // Adjust strategy based on recent outcomes
        self.adjust_strategy();
    }

    pub fn decide(&mut self, options: Vec<String>, context: &HashMap<String, f32>) -> String {
        self.processing_load += 0.1;

        let mut best_option = options[0].clone();
        let mut best_score = f32::NEG_INFINITY;

        for option in &options {
            let score = self.evaluate_option(option, context);
            if score > best_score {
                best_score = score;
                best_option = option.clone();
            }
        }

        // Record decision
        let decision = Decision {
            options,
            chosen: best_option.clone(),
            context: context.clone(),
            confidence: self.calculate_confidence(best_score),
            outcome: None,
            timestamp: 0.0, // Current time should be passed
        };

        self.decision_history.push(decision);

        best_option
    }

    pub fn evaluate_option(&self, option: &str, context: &HashMap<String, f32>) -> f32 {
        let base_score = match self.current_strategy {
            DecisionStrategy::Rational => self.rational_evaluation(option, context),
            DecisionStrategy::Intuitive => self.intuitive_evaluation(option, context),
            DecisionStrategy::RiskAverse => self.risk_averse_evaluation(option, context),
            DecisionStrategy::RiskSeeking => self.risk_seeking_evaluation(option, context),
            DecisionStrategy::Balanced => {
                (self.rational_evaluation(option, context) + 
                 self.intuitive_evaluation(option, context)) / 2.0
            }
        };

        // Apply decision weights
        if let Some(weight) = self.decision_weights.get(option) {
            base_score * weight
        } else {
            base_score
        }
    }

    pub fn record_outcome(&mut self, outcome: DecisionOutcome) {
        if let Some(last_decision) = self.decision_history.last_mut() {
            last_decision.outcome = Some(outcome);
        }
    }

    fn rational_evaluation(&self, option: &str, context: &HashMap<String, f32>) -> f32 {
        let mut score = 0.5; // Base score

        // Consider context factors
        for (factor, value) in context {
            match factor.as_str() {
                "risk" => score -= value * 0.2,
                "benefit" => score += value * 0.3,
                "cost" => score -= value * 0.25,
                "time" => score -= value * 0.15,
                _ => score += value * 0.1,
            }
        }

        score.clamp(0.0, 1.0)
    }

    fn intuitive_evaluation(&self, option: &str, context: &HashMap<String, f32>) -> f32 {
        let mut score = 0.5;

        // Quick pattern matching from past decisions
        if let Some(similar_decision) = self.find_similar_decision(option, context) {
            if let Some(outcome) = &similar_decision.outcome {
                score += if outcome.success { 0.3 } else { -0.2 };
            }
        }

        // Consider "gut feeling" factors
        if let Some(familiarity) = context.get("familiarity") {
            score += familiarity * 0.2;
        }

        score.clamp(0.0, 1.0)
    }

    fn risk_averse_evaluation(&self, option: &str, context: &HashMap<String, f32>) -> f32 {
        let base_score = self.rational_evaluation(option, context);
        let risk_factor = context.get("risk").unwrap_or(&0.5);
        
        base_score * (1.0 - risk_factor)
    }

    fn risk_seeking_evaluation(&self, option: &str, context: &HashMap<String, f32>) -> f32 {
        let base_score = self.rational_evaluation(option, context);
        let potential_gain = context.get("benefit").unwrap_or(&0.5);
        
        base_score * (1.0 + potential_gain)
    }

    fn find_similar_decision(&self, option: &str, context: &HashMap<String, f32>) -> Option<&Decision> {
        self.decision_history.iter()
            .filter(|d| d.chosen == option)
            .max_by(|a, b| {
                let a_similarity = self.calculate_context_similarity(&a.context, context);
                let b_similarity = self.calculate_context_similarity(&b.context, context);
                a_similarity.partial_cmp(&b_similarity).unwrap()
            })
    }

    fn calculate_context_similarity(&self, context1: &HashMap<String, f32>, context2: &HashMap<String, f32>) -> f32 {
        let mut similarity = 0.0;
        let mut count = 0;

        for (key, value1) in context1 {
            if let Some(value2) = context2.get(key) {
                similarity += 1.0 - (value1 - value2).abs();
                count += 1;
            }
        }

        if count == 0 {
            0.0
        } else {
            similarity / count as f32
        }
    }

    fn calculate_confidence(&self, score: f32) -> f32 {
        let uncertainty = 1.0 - score.abs();
        if uncertainty > self.uncertainty_threshold {
            0.5 * (1.0 - (uncertainty - self.uncertainty_threshold))
        } else {
            score
        }
    }

    fn update_weights(&mut self) {
        for decision in &self.decision_history {
            if let Some(outcome) = &decision.outcome {
                let weight = self.decision_weights
                    .entry(decision.chosen.clone())
                    .or_insert(1.0);
                
                *weight *= if outcome.success {
                    1.1
                } else {
                    0.9
                };
                
                *weight = weight.clamp(0.5, 2.0);
            }
        }
    }

    fn adjust_strategy(&mut self) {
        let recent_outcomes: Vec<_> = self.decision_history.iter()
            .rev()
            .take(5)
            .filter_map(|d| d.outcome.as_ref())
            .collect();

        if recent_outcomes.len() < 3 {
            return;
        }

        let success_rate = recent_outcomes.iter()
            .filter(|o| o.success)
            .count() as f32 / recent_outcomes.len() as f32;

        self.current_strategy = match success_rate {
            x if x < 0.3 => DecisionStrategy::RiskAverse,
            x if x < 0.5 => DecisionStrategy::Rational,
            x if x > 0.8 => DecisionStrategy::RiskSeeking,
            _ => DecisionStrategy::Balanced,
        };
    }

    pub fn get_load(&self) -> f32 {
        self.processing_load
    }
}