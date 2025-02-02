use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod decision;
pub mod reasoning;
pub mod perception;
pub mod bias;

use decision::DecisionMaker;
use reasoning::ReasoningEngine;
use perception::PerceptionSystem;
use bias::BiasSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionSystem {
    decision_maker: DecisionMaker,
    reasoning: ReasoningEngine,
    perception: PerceptionSystem,
    bias: BiasSystem,
    working_memory: Vec<Thought>,
    cognitive_load: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    content: String,
    source: CognitiveSource,
    confidence: f32,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitiveSource {
    Perception,
    Memory,
    Reasoning,
    Intuition,
    External,
}

impl Default for CognitionSystem {
    fn default() -> Self {
        Self {
            decision_maker: DecisionMaker::default(),
            reasoning: ReasoningEngine::default(),
            perception: PerceptionSystem::default(),
            bias: BiasSystem::default(),
            working_memory: Vec::new(),
            cognitive_load: 0.0,
        }
    }
}

impl CognitionSystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update each cognitive subsystem
        self.perception.update(delta_time);
        self.reasoning.update(delta_time);
        self.decision_maker.update(delta_time);
        self.bias.update(delta_time);

        // Update working memory and cognitive load
        self.update_working_memory();
        self.update_cognitive_load(delta_time);
    }

    pub fn process_input(&mut self, input: &str, source: CognitiveSource) -> Option<Thought> {
        // First, process through perception system
        let perceived = self.perception.process_input(input);

        // Apply biases to perception
        let biased_input = self.bias.apply_biases(&perceived);

        // Reason about the input
        let reasoning_result = self.reasoning.analyze(&biased_input);

        // Create a thought from the processing
        let thought = Thought {
            content: reasoning_result,
            source,
            confidence: self.calculate_confidence(),
            timestamp: 0.0, // Current time should be passed
        };

        // Add to working memory
        self.working_memory.push(thought.clone());

        Some(thought)
    }

    pub fn make_decision(&mut self, options: Vec<String>, context: HashMap<String, f32>) -> String {
        // Consider biases
        let biased_context = self.bias.influence_context(context);

        // Process through reasoning
        let analyzed_options = options.iter()
            .map(|opt| self.reasoning.evaluate_option(opt, &biased_context))
            .collect();

        // Make final decision
        self.decision_maker.decide(analyzed_options, &biased_context)
    }

    fn update_working_memory(&mut self) {
        // Remove old thoughts
        self.working_memory.retain(|thought| {
            thought.timestamp < 100.0 // Keep thoughts from last 100 time units
        });

        // Sort by confidence
        self.working_memory.sort_by(|a, b| {
            b.confidence.partial_cmp(&a.confidence).unwrap()
        });

        // Keep only the most relevant thoughts
        while self.working_memory.len() > 10 {
            self.working_memory.pop();
        }
    }

    fn update_cognitive_load(&mut self, delta_time: f32) {
        // Base load from working memory
        let memory_load = self.working_memory.len() as f32 * 0.1;

        // Add load from active processes
        let process_load = self.perception.get_load() +
            self.reasoning.get_load() +
            self.decision_maker.get_load();

        // Calculate target load
        let target_load = (memory_load + process_load).clamp(0.0, 1.0);

        // Smoothly transition to target
        self.cognitive_load += (target_load - self.cognitive_load) * delta_time;
    }

    fn calculate_confidence(&self) -> f32 {
        // Base confidence
        let base = 0.5;

        // Adjust based on cognitive load
        let load_factor = 1.0 - self.cognitive_load;

        // Adjust based on bias influence
        let bias_factor = 1.0 - self.bias.get_total_bias_influence();

        // Calculate final confidence
        (base * load_factor * bias_factor).clamp(0.0, 1.0)
    }

    pub fn get_cognitive_state(&self) -> CognitiveState {
        CognitiveState {
            load: self.cognitive_load,
            active_thoughts: self.working_memory.len(),
            dominant_bias: self.bias.get_dominant_bias(),
            perception_clarity: self.perception.get_clarity(),
            reasoning_confidence: self.reasoning.get_confidence(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CognitiveState {
    pub load: f32,
    pub active_thoughts: usize,
    pub dominant_bias: String,
    pub perception_clarity: f32,
    pub reasoning_confidence: f32,
}