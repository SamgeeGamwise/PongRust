use derive_more::Constructor;
use macroquad::color::Color;
use macroquad::prelude::{draw_rectangle, screen_height, Vec2};

#[derive(Constructor, Clone, Copy)]
pub struct Ball {
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub size: Vec2,
    pub color: Color
}

impl Ball {
    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, self.color);
    }
}