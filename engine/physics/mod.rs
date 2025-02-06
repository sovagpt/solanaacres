pub mod movement;

use std::collections::HashMap;
use uuid::Uuid;

pub struct PhysicsSystem {
    movement_system: movement::MovementSystem,
    physics_bodies: HashMap<Uuid, PhysicsBody>,
}

#[derive(Debug, Clone)]
pub struct PhysicsBody {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    mass: f32,
    friction: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            movement_system: movement::MovementSystem::new(),
            physics_bodies: HashMap::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.movement_system.update(delta_time);
        self.update_physics_bodies(delta_time);
    }

    fn update_physics_bodies(&mut self, delta_time: f32) {
        for body in self.physics_bodies.values_mut() {
            // Update velocity based on acceleration
            body.velocity.x += body.acceleration.x * delta_time;
            body.velocity.y += body.acceleration.y * delta_time;

            // Apply friction
            let friction_force = -1.0 * body.friction;
            body.velocity.x *= friction_force.exp();
            body.velocity.y *= friction_force.exp();

            // Update position based on velocity
            body.position.x += body.velocity.x * delta_time;
            body.position.y += body.velocity.y * delta_time;
        }
    }
}