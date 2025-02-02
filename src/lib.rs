pub mod ai;
pub mod engine;
pub mod entities;
pub mod network;
pub mod error;
pub mod config;

use bevy::prelude::*;

/// Configuration for the HelloWorld simulation
#[derive(Resource, Debug, Clone)]
pub struct HelloWorldConfig {
    pub simulation_speed: f32,
    pub max_npcs: usize,
    pub world_size: Vec2,
}

impl Default for HelloWorldConfig {
    fn default() -> Self {
        Self {
            simulation_speed: 1.0,
            max_npcs: 100,
            world_size: Vec2::new(1000.0, 1000.0),
        }
    }
}

/// Global state of the simulation
#[derive(Resource, Debug)]
pub struct WorldState {
    pub time: f32,
    pub active_npcs: usize,
    pub total_interactions: usize,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            time: 0.0,
            active_npcs: 0,
            total_interactions: 0,
        }
    }
}

/// Error type for the simulation
#[derive(Debug, thiserror::Error)]
pub enum HelloWorldError {
    #[error("AI Error: {0}")]
    AiError(String),
    
    #[error("Engine Error: {0}")]
    EngineError(String),
    
    #[error("Network Error: {0}")]
    NetworkError(String),
}

pub type Result<T> = std::result::Result<T, HelloWorldError>;