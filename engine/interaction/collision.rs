use std::collections::HashSet;
use crate::physics::Vector2;

pub struct CollisionSystem {
    collision_pairs: HashSet<(Uuid, Uuid)>,
    collision_map: HashMap<Uuid, CollisionInfo>,
}

#[derive(Debug, Clone)]
pub struct CollisionInfo {
    bounds: BoundingBox,
    collision_layer: u32,
    collision_mask: u32,
}

impl CollisionSystem {
    pub fn new() -> Self {
        Self {
            collision_pairs: HashSet::new(),
            collision_map: HashMap::new(),
        }
    }

    pub fn check_collision(&self, entity1: &Entity, entity2: &Entity) -> bool {
        if let (Some(info1), Some(info2)) = (
            self.collision_map.get(&entity1.id),
            self.collision_map.get(&entity2.id)
        ) {
            info1.bounds.intersects(&info2.bounds) &&
            (info1.collision_layer & info2.collision_mask != 0)
        } else {
            false
        }
    }
}