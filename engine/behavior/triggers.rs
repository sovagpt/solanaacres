pub struct TriggerSystem {
    triggers: Vec<Trigger>,
    active_triggers: HashSet<Uuid>,
}

pub struct Trigger {
    id: Uuid,
    condition: Box<dyn Fn() -> bool>,
    action: Box<dyn Fn()>,
    one_shot: bool,
}

impl TriggerSystem {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
            active_triggers: HashSet::new(),
        }
    }

    pub fn add_trigger(&mut self, condition: impl Fn() -> bool + 'static, 
                      action: impl Fn() + 'static, one_shot: bool) -> Uuid {
        let id = Uuid::new_v4();
        self.triggers.push(Trigger {
            id,
            condition: Box::new(condition),
            action: Box::new(action),
            one_shot,
        });
        id
    }

    pub fn update(&mut self) {
        let mut triggered = Vec::new();
        
        for trigger in &self.triggers {
            if (trigger.condition)() {
                (trigger.action)();
                if trigger.one_shot {
                    triggered.push(trigger.id);
                }
            }
        }

        // Remove one-shot triggers that fired
        self.triggers.retain(|t| !triggered.contains(&t.id));
    }
}