use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("AI System Error: {0}")]
    AiSystem(String),

    #[error("Engine Error: {0}")]
    Engine(String),

    #[error("Network Error: {0}")]
    Network(String),

    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("Entity Error: {0}")]
    Entity(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;