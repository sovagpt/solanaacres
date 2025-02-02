use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningEngine {
    belief_network: HashMap<String, Vec<String>>,
    logical_rules: Vec<LogicalRule>,
    inference_cache: HashMap<String, InferenceResult>,
    reasoning_confidence: f32,
    processing_load: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalRule {
    premises: Vec<String>,
    conclusion: String,
    confidence: f32,
    context_requirements: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    conclusion: String,
    reasoning_path: Vec<String>,
    confidence: f32,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningStrategy {
    Deductive,
    Inductive,
    Abductive,
    Analogical,
}

impl Default for ReasoningEngine {
    fn default() -> Self {
        Self {
            belief_network: HashMap::new(),
            logical_rules: Vec::new(),
            inference_cache: HashMap::new(),
            reasoning_confidence: 0.7,
            processing_load: 0.0,
        }
    }
}

impl ReasoningEngine {
    pub fn update(&mut self, delta_time: f32) {
        // Decay processing load
        self.processing_load *= 0.95f32.powf(delta_time);
        
        // Clean up old cache entries
        self.cleanup_cache();
    }

    pub fn analyze(&mut self, input: &str) -> String {
        self.processing_load += 0.1;

        // Check cache first
        if let Some(cached) = self.inference_cache.get(input) {
            if cached.timestamp < 100.0 { // Cache valid for 100 time units
                return cached.conclusion.clone();
            }
        }

        // Perform reasoning
        let result = self.reason_about(input);
        
        // Cache result
        self.inference_cache.insert(input.to_string(), InferenceResult {
            conclusion: result.clone(),
            reasoning_path: vec![],
            confidence: self.reasoning_confidence,
            timestamp: 0.0,
        });

        result
    }

    pub fn evaluate_option(&self, option: &str, context: &HashMap<String, f32>) -> f32 {
        let mut score = 0.5;

        // Apply logical rules
        for rule in &self.logical_rules {
            if self.rule_applies(rule, option, context) {
                score += rule.confidence * 0.2;
            }
        }

        // Consider belief network
        if let Some(related_beliefs) = self.belief_network.get(option) {
            let belief_score = related_beliefs.iter()
                .filter(|belief| context.contains_key(*belief))
                .count() as f32 / related_beliefs.len() as f32;
            
            score += belief_score * 0.3;
        }

        score.clamp(0.0, 1.0)
    }

    pub fn add_logical_rule(&mut self, rule: LogicalRule) {
        self.logical_rules.push(rule);
    }

    pub fn add_belief_connection(&mut self, belief: String, connected: String) {
        self.belief_network
            .entry(belief)
            .or_default()
            .push(connected);
    }

    fn reason_about(&self, input: &str) -> String {
        // Try different reasoning strategies
        let mut conclusions = vec![
            self.deductive_reasoning(input),
            self.inductive_reasoning(input),
            self.abductive_reasoning(input),
        ];

        // Remove None values and sort by confidence
        conclusions.retain(|c| c.is_some());
        conclusions.sort_by(|a, b| {
            b.as_ref().unwrap().confidence.partial_cmp(&a.as_ref().unwrap().confidence).unwrap()
        });

        // Return most confident conclusion or default
        conclusions
            .first()
            .and_then(|c| c.as_ref())
            .map(|c| c.conclusion.clone())
            .unwrap_or_else(|| "No conclusion reached".to_string())
    }

    fn deductive_reasoning(&self, premise: &str) -> Option<InferenceResult> {
        for rule in &self.logical_rules {
            if rule.premises.iter().any(|p| p == premise) {
                return Some(InferenceResult {
                    conclusion: rule.conclusion.clone(),
                    reasoning_path: vec![premise.to_string()],
                    confidence: rule.confidence,
                    timestamp: 0.0,
                });
            }
        }
        None
    }

    fn inductive_reasoning(&self, observation: &str) -> Option<InferenceResult> {
        if let Some(patterns) = self.belief_network.get(observation) {
            let most_common = patterns.iter()
                .max_by_key(|&pattern| {
                    patterns.iter().filter(|&p| p == pattern).count()
                })?;

            Some(InferenceResult {
                conclusion: format!("Based on patterns, this suggests {}", most_common),
                reasoning_path: vec![observation.to_string()],
                confidence: 0.6, // Inductive reasoning has lower confidence
                timestamp: 0.0,
            })
        } else {
            None
        }
    }

    fn abductive_reasoning(&self, observation: &str) -> Option<InferenceResult> {
        // Find rules that could explain the observation
        let explaining_rules: Vec<_> = self.logical_rules.iter()
            .filter(|rule| rule.conclusion.contains(observation))
            .collect();

        if let Some(best_explanation) = explaining_rules.first() {
            Some(InferenceResult {
                conclusion: format!("This might be because {}", 
                    best_explanation.premises.join(" and ")),
                reasoning_path: vec![observation.to_string()],
                confidence: 0.5, // Abductive reasoning has lowest confidence
                timestamp: 0.0,
            })
        } else {
            None
        }
    }

    fn rule_applies(&self, rule: &LogicalRule, situation: &str, context: &HashMap<String, f32>) -> bool {
        // Check context requirements
        for req in &rule.context_requirements {
            if !context.contains_key(req) {
                return false;
            }
        }

        // Check premises
        rule.premises.iter().any(|premise| situation.contains(premise))
    }

    fn cleanup_cache(&mut self) {
        self.inference_cache.retain(|_, result| result.timestamp < 100.0);
    }

    pub fn get_confidence(&self) -> f32 {
        self.reasoning_confidence
    }

    pub fn get_load(&self) -> f32 {
        self.processing_load
    }
}