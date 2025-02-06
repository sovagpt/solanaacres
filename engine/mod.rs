use std::sync::Arc;
use tokio::sync::RwLock;

pub mod simulation;
pub mod interaction;
pub mod behavior;
pub mod physics;
pub mod systems;

pub struct Engine {
    world: Arc<RwLock<World>>,
    event_manager: EventManager,
    time_manager: TimeManager,
    scheduler: Scheduler,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            world: Arc::new(RwLock::new(World::default())),
            event_manager: EventManager::new(),
            time_manager: TimeManager::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub async fn update(&mut self, delta_time: f32) {
        self.time_manager.update(delta_time);
        self.scheduler.update(delta_time);
        
        let mut world = self.world.write().await;
        world.update(delta_time);
    }
}