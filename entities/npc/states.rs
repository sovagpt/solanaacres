use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCState {
    current_state: State,
    previous_state: Option<State>,
    state_duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum State {
    Idle,
    Walking,
    Working,
    Socializing,
    Sleeping,
    Trading,
    Thinking,
}

impl NPCState {
    pub fn new() -> Self {
        Self {
            current_state: State::Idle,
            previous_state: None,
            state_duration: 0.0,
        }
    }

    pub fn change_state(&mut self, new_state: State) {
        self.previous_state = Some(self.current_state.clone());
        self.current_state = new_state;
        self.state_du