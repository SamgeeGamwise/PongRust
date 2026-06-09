use crate::events::state_events::StateEvent;
use crate::state_machine::state::State;

pub struct StateMachine {
    states: Vec<Box<dyn State>>
}

impl StateMachine {
    pub fn new() -> Self {
        let states = Vec::new();

        Self {
            states,
        }
    }
    pub fn push(&mut self, state: Box<dyn State>) {
        self.states.push(state);
    }

    pub fn pop(&mut self) {
        self.states.pop();
    }

    pub fn peek(&mut self) -> Option<&mut (dyn State + '_)> {
        match self.states.last_mut() {
            Some(state) => Some(state.as_mut()),
            None => None,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let event = match self.peek() {
            Some(state) => state.update(delta_time),
            None => StateEvent::None,
        };

        self.apply_state_event(event);
    }
    fn apply_state_event(&mut self, event: StateEvent) {
        match event {
            StateEvent::None => {}

            StateEvent::Push(state) => {
                self.push(state);
            }

            StateEvent::Pop => {
                self.pop();
            }

            StateEvent::Switch(state) => {
                self.pop();
                self.push(state);
            }
        }
    }
    pub fn draw(&mut self) {
        if let Some(state) = self.peek() {
            state.draw();
        }
    }
}