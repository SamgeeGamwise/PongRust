use crate::events::state_events::StateEvent;

pub trait State {
    fn update(&mut self, delta_time: f32) -> StateEvent;
    fn draw(&self) -> ();
    fn blocks_update_below(&self) -> bool {
        true
    }

    fn blocks_draw_below(&self) -> bool {
        true
    }
}