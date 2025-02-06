pub struct MovementSystem {
    entities: HashMap<Uuid, MovementComponent>,
    path_cache: HashMap<Uuid, Path>,
}

#[derive(Debug, Clone)]
pub struct MovementComponent {
    position: Vector2,
    target: Option<Vector2>,
    speed: f32,
    moving: bool,
}

#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Vector2>,
    current_point: usize,
}

impl MovementSystem {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            path_cache: HashMap::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for (entity_id, movement) in &mut self.entities {
            if let Some(target) = movement.target {
                if movement.moving {
                    self.update_movement(entity_id, movement, delta_time);
                }
            }
        }
    }

    fn update_movement(&mut self, entity_id: &Uuid, movement: &mut MovementComponent, delta_time: f32) {
        if let Some(target) = movement.target {
            let direction = Vector2 {
                x: target.x - movement.position.x,
                y: target.y - movement.position.y,
            };

            let distance = (direction.x * direction.x + direction.y * direction.y).sqrt();

            if distance > 1.0 {
                let normalized_direction = Vector2 {
                    x: direction.x / distance,
                    y: direction.y / distance,
                };

                movement.position.x += normalized_direction.x * movement.speed * delta_time;
                movement.position.y += normalized_direction.y * movement.speed * delta_time;
            } else {
                movement.moving = false;
                movement.target = None;
            }
        }
    }

    pub fn move_to(&mut self, entity_id: Uuid, target: Vector2) {
        if let Some(movement) = self.entities.get_mut(&entity_id) {
            movement.target = Some(target);
            movement.moving = true;
        }
    }

    pub fn stop(&mut self, entity_id: Uuid) {
        if let Some(movement) = self.entities.get_mut(&entity_id) {
            movement.moving = false;
            movement.target = None;
        }
    }
}