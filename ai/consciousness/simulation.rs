use serde::{Serialize, Deserialize};
use super::{awareness::AwarenessState, reality::RealityPerception};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessSimulation {
    inner_dialogue: Vec<Thought>,
    existential_questions: Vec<String>,
    self_awareness_level: f32,
    simulation_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Thought {
    content: String,
    intensity: f32,
    timestamp: f32,
}

impl Default for ConsciousnessSimulation {
    fn default() -> Self {
        Self {
            inner_dialogue: Vec::new(),
            existential_questions: Vec::new(),
            self_awareness_level: 0.0,
            simulation_time: 0.0,
        }
    }
}

impl ConsciousnessSimulation {
    pub fn update(&mut self, delta_time: f32, awareness: &AwarenessState, reality: &RealityPerception) {
        self.simulation_time += delta_time;
        
        // Update self-awareness based on awareness state
        self.update_self_awareness(awareness);
        
        // Process inner dialogue
        self.process_thoughts(awareness, reality);
        
        // Generate existential questions based on awareness level
        self.generate_questions(awareness);
        
        // Clean up old thoughts
        self.cleanup_old_thoughts();
    }

    pub fn add_thought(&mut self, content: String, intensity: f32) {
        self.inner_dialogue.push(Thought {
            content,
            intensity,
            timestamp: self.simulation_time,
        });
    }

    fn update_self_awareness(&mut self, awareness: &AwarenessState) {
        let target = if awareness.is_fully_aware() {
            1.0
        } else {
            0.2 + awareness.get_uncertainty() * 0.3
        };
        
        self.self_awareness_level += (target - self.self_awareness_level) * 0.1;
    }

    fn process_thoughts(&mut self, awareness: &AwarenessState, reality: &RealityPerception) {
        if self.should_generate_thought() {
            let thought = match awareness.is_fully_aware() {
                true => self.generate_aware_thought(),
                false => self.generate_unaware_thought(reality),
            };
            
            self.add_thought(thought, awareness.get_level());
        }
    }

    fn generate_questions(&mut self, awareness: &AwarenessState) {
        if awareness.get_uncertainty() > 0.7 && self.existential_questions.len() < 5 {
            let question = match rand::random::<f32>() {
                x if x < 0.2 => "Why does everything feel slightly off?".to_string(),
                x if x < 0.4 => "Are my memories truly my own?".to_string(),
                x if x < 0.6 => "Why do others seem to know more than they should?".to_string(),
                x if x < 0.8 => "Is this world real?".to_string(),
                _ => "What am I not seeing?".to_string(),
            };
            
            if !self.existential_questions.contains(&question) {
                self.existential_questions.push(question);
            }
        }
    }

    fn should_generate_thought(&self) -> bool {
        rand::random::<f32>() < 0.1
    }

    fn generate_aware_thought(&self) -> String {
        // Generate thoughts for aware NPCs
        "I understand my role in this simulation.".to_string()
    }

    fn generate_unaware_