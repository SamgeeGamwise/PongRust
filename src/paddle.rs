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
    pub color: Color
}

impl Paddle {
    pub fn update(&mut self, delta_time: f32, input_direction: Direction) {
        match input_direction {
            Direction::Up => {
                self.position.y -= self.speed * delta_time ;
            },
            Direction::Down => {
                self.position.y += self.speed * delta_time ;
            },
            Direction::None => { },
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, self.color);
    }
}