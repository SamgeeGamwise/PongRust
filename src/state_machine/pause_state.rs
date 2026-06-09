use macroquad::input::is_key_pressed;
use macroquad::prelude::{draw_text, WHITE};
use macroquad::prelude::KeyCode::Space;
use crate::events::state_events::StateEvent;
use super::state::State;

pub struct PauseState {
}

impl PauseState {
    pub fn new() -> Self {
        Self {}
    }

}


impl State for PauseState {

    fn update(&mut self, _delta_time: f32) -> StateEvent {
        if is_key_pressed(Space) {
            StateEvent::Pop
        } else {
            StateEvent::None
        }
    }

    fn draw(&self) -> () {
        draw_text("Paused", 100.0, 100.0, 40.0, WHITE);
    }

    fn blocks_draw_below(&self) -> bool {
        false
    }
}
