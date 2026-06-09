use derive_more::Constructor;
use macroquad::color::Color;
use macroquad::prelude::{draw_rectangle, Rect, Vec2};

#[derive(Constructor, Clone, Copy)]
pub struct Ball {
    pub rectangle: Rect,
    pub direction: Vec2,
    pub speed: f32,
    pub color: Color
}

impl Ball {
    pub fn draw(&self) {
        draw_rectangle(self.rectangle.x, self.rectangle.y, self.rectangle.w, self.rectangle.h, self.color);
    }
}