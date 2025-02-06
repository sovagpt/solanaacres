pub struct UpdateSystem {
    components: HashMap<Uuid, Vec<Box<dyn Component>>>,
    update_order: Vec<UpdatePhase>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UpdatePhase {
    PrePhysics,
    Physics,
    PostPhysics,
    Render,
}

pub trait Component: Send + Sync {
    fn update(&mut self, delta_time: f32);
    fn get_phase(&self) -> UpdatePhase;
}

impl UpdateSystem {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            update_order: vec![
                UpdatePhase::PrePhysics,
                UpdatePhase::Physics,
                UpdatePhase::PostPhysics,
                UpdatePhase::Render,
            ],
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for phase in &self.update_order {
            for components in self.components.values_mut() {
                for component in components.iter_mut() {
                    if component.get_phase() == *phase {
                        component.update(delta_time);
                    }
                }
            }
        }
    }

    pub fn add_component(&mut self, entity_id: Uuid, component: Box<dyn Component>) {
        self.components
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(component);
    }

    pub fn remove_components(&mut self, entity_id: &Uuid) {
        self.components.remove(entity_id);
    }
}