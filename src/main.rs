use bevy::prelude::*;

mod ai;
mod engine;
mod entities;
mod network;
mod error;
mod config;

/// Main entry point for the HelloWorld simulation
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_ai_systems,
            update_engine_state,
            handle_network_events,
        ))
        .run();
}

/// Game states for the simulation
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    Running,
    Paused,
}

/// Initial setup of the simulation
fn setup(mut commands: Commands) {
    // Initialize AI Director
    commands.insert_resource(ai::AiDirector::default());
    
    // Initialize Engine State
    commands.insert_resource(engine::EngineState::default());
    
    // Initialize Network Manager
    commands.insert_resource(network::NetworkManager::default());
}

/// Updates AI systems each frame
fn update_ai_systems(
    mut ai_director: ResMut<ai::AiDirector>,
    time: Res<Time>,
) {
    ai_director.update(time.delta_seconds());
}

/// Updates engine state
fn update_engine_state(
    mut engine_state: ResMut<engine::EngineState>,
    time: Res<Time>,
) {
    engine_state.update(time.delta_seconds());
}

/// Handles incoming network events
fn handle_network_events(
    mut net_manager: ResMut<network::NetworkManager>,
) {
    net_manager.process_events();
}