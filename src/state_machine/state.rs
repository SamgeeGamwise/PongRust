use crate::events::state_events::StateEvent;

pub trait State {
    fn enter(&self) -> ();
    fn exit(&self) -> ();
    fn update(&mut self, delta_time: f32) -> StateEvent;
    fn draw(&self) -> ();
}