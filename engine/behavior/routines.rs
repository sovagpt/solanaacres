pub struct Routine {
    steps: Vec<RoutineStep>,
    current_step: usize,
    repeat: bool,
    active: bool,
}

pub struct RoutineStep {
    action: Box<dyn Action>,
    duration: Option<f32>,
    condition: Option<Box<dyn Fn() -> bool>>,
}

impl Routine {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            repeat: false,
            active: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.active { return; }

        if let Some(step) = self.steps.get(self.current_step) {
            if step.action.execute(delta_time) {
                self.advance_step();
            }
        }
    }

    fn advance_step(&mut self) {
        self.current_step += 1;
        if self.current_step >= self.steps.len() {
            if self.repeat {
                self.current_step = 0;
            } else {
                self.active = false;
            }
        }
    }
}