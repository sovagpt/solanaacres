use std::collections::HashMap;
use crate::physics::Vector2;

pub struct ProximitySystem {
    spatial_hash: HashMap<(i32, i32), Vec<Uuid>>,
    entity_positions: HashMap<Uuid, Vector2>,
    cell_size: f32,
}

impl ProximitySystem {
    pub fn new() -> Self {
        Self {
            spatial_hash: HashMap::new(),
            entity_positions: HashMap::new(),
            cell_size: 32.0,
        }
    }

    pub fn update_position(&mut self, entity_id: Uuid, position: Vector2) {
        // Remove from old cell
        if let Some(old_pos) = self.entity_positions.get(&entity_id) {
            let old_cell = self.get_cell_coords(*old_pos);
            if let Some(entities) = self.spatial_hash.get_mut(&old_cell) {
                entities.retain(|id| *id != entity_id);
            }
        }

        // Add to new cell
        let new_cell = self.get_cell_coords(position);
        self.spatial_hash
            .entry(new_cell)
            .or_insert_with(Vec::new)
            .push(entity_id);
        
        self.entity_positions.insert(entity_id, position);
    }

    pub fn get_nearby_entities(&self, position: Vector2, radius: f32) -> Vec<Uuid> {
        let mut nearby = Vec::new();
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_cell = self.get_cell_coords(position);

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = (center_cell.0 + dx, center_cell.1 + dy);
                if let Some(entities) = self.spatial_hash.get(&cell) {
                    nearby.extend(entities.iter());
                }
            }
        }

        nearby
    }

    fn get_cell_coords(&self, position: Vector2) -> (i32, i32) {
        (
            (position.x / self.cell_size).floor() as i32,
            (position.y / self.cell_size).floor() as i32,
        )
    }
}