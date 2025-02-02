use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub simulation: SimulationConfig,
    pub ai: AiConfig,
    pub network: NetworkConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationConfig {
    pub tick_rate: f32,
    pub world_size: Vec2,
    pub max_entities: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiConfig {
    pub max_npcs: usize,
    pub memory_decay_rate: f32,
    pub interaction_radius: f32,
    pub awareness_threshold: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub ws_port: u16,
    pub max_connections: usize,
    pub tick_rate: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            simulation: SimulationConfig {
                tick_rate: 60.0,
                world_size: Vec2::new(1000.0, 1000.0),
                max_entities: 1000,
            },
            ai: AiConfig {
                max_npcs: 100,
                memory_decay_rate: 0.1,
                interaction_radius: 50.0,
                awareness_threshold: 0.8,
            },
            network: NetworkConfig {
                ws_port: 8080,
                max_connections: 100,
                tick_rate: 20.0,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config.toml").required(false))
            .add_source(config::Environment::with_prefix("HELLOWORLD"))
            .build()?
            .try_deserialize()?;

        Ok(config)
    }
}