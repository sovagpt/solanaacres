#[derive(Debug, Clone)]
pub struct StateMachine {
    current_state: State,
    states: HashMap<String, State>,
    transitions: Vec<Transition>,
}

#[derive(Debug, Clone)]
pub struct State {
    name: String,
    on_enter: Option<Box<dyn Fn()>>,
    on_update: Option<Box<dyn Fn(f32)>>,
    on_exit: Option<Box<dyn Fn()>>,
}

#[derive(Debug, Clone)]
pub struct Transition {
    from: String,
    to: String,
    condition: Box<dyn Fn() -> bool>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current_state: State::default(),
            states: HashMap::new(),
            transitions: Vec::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Check transitions
        if let Some(transition) = self.check_transitions() {
            self.change_state(transition.to.clone());
        }

        // Update current state
        if let Some(update) = &self.current_state.on_update {
            update(delta_time);
        }
    }
}