pub trait State {
    fn enter(&self) -> ();
    fn exit(&self) -> ();
    fn update(&mut self, delta_time: f32) -> ();
    fn draw(&self) -> ();
}