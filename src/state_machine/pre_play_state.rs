use macroquad::prelude::{draw_text, WHITE};
use crate::events::state_events::StateEvent;
use crate::timer::Timer;
use super::state::State;

pub struct PrePlayState {
    timer: Timer
}

impl PrePlayState {
    const PRE_GAME_TIMER: f32 = 2.0;

    pub fn new() -> Self {
        Self {
            timer: Timer::new(Self::PRE_GAME_TIMER),
        }
    }

}


impl State for PrePlayState {

    fn update(&mut self, delta_time: f32) -> StateEvent {
        if self.timer.finished() {
            StateEvent::Pop
        } else {
            self.timer.update(delta_time);
            StateEvent::None
        }
    }

    fn draw(&self) -> () {
        draw_text("Get Ready", 100.0, 100.0, 40.0, WHITE);
    }

    fn blocks_draw_below(&self) -> bool {
        false
    }
}
