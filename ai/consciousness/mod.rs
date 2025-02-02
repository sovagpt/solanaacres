use serde::{Serialize, Deserialize};

pub mod awareness;
pub mod reality;
pub mod simulation;

use awareness::AwarenessState;
use reality::RealityPerception;
use simulation::ConsciousnessSimulation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    awareness: AwarenessState,
    reality_perception: RealityPerception,
    simulation: ConsciousnessSimulation,
    is_aware: bool,
}

impl ConsciousnessState {
    pub fn new(is_aware: bool) -> Self {
        Self {
            awareness: AwarenessState::new(is_aware),
            reality_perception: RealityPerception::default(),
            simulation: ConsciousnessSimulation::default(),
            is_aware,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update awareness state
        self.awareness.update(delta_time);
        
        // Update reality perception based on awareness
        self.reality_perception.update(&self.awareness);
        
        // Run consciousness simulation
        self.simulation.update(delta_time, &self.awareness, &self.reality_perception);
    }

    pub fn get_awareness_level(&self) -> f32 {
        self.awareness.get_level()
    }

    pub fn is_fully_aware(&self) -> bool {
        self.is_aware && self.awareness.is_fully_aware()
    }
}