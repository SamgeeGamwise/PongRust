use derive_more::Constructor;
use macroquad::color::Color;
use macroquad::prelude::{draw_rectangle, Vec2};
use crate::directions::Direction;

#[derive(Constructor, Clone, Copy)]
pub struct Paddle {
    pub position: Vec2,
    pub size: Vec2,
    pub is_left: bool,
    pub is_player: bool,
    pub speed: f32,
    pub color: Color,
    pub direction: Direction,
}

impl Paddle {
    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, self.color);
    }
    
    pub fn get_direction(&self) -> Direction {
        self.direction
    }
    
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}