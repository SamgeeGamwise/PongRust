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
    const SPEED_UP_PERCENTAGE: f32 = 1.1;
    pub fn bounce(&mut self, intercepted: bool) {
        if intercepted {
            self.direction.x *= -1.0;
        } else {
            self.direction.y *= -1.0;
        }
    }

    pub fn speed_up(&mut self) {
        self.speed *= Self::SPEED_UP_PERCENTAGE;
    }

    pub fn draw(&self) {
        draw_rectangle(self.rectangle.x, self.rectangle.y, self.rectangle.w, self.rectangle.h, self.color);
    }
}