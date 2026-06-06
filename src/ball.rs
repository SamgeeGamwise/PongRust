use derive_more::Constructor;
use macroquad::color::Color;
use macroquad::prelude::{draw_rectangle, screen_height, Vec2};

#[derive(Constructor, Clone, Copy)]
pub struct Ball {
    pub position: Vec2,
    pub direction: Vec2,
    pub size: Vec2,
    pub speed: f32,
    pub color: Color
}

impl Ball {
    pub fn update(&mut self, delta_time: f32) {
        let new_position = self.position + ((self.speed * self.direction) * delta_time);

        if new_position.y < 0.0 || new_position.y > screen_height(){
            println!("Bounce!");
            self.direction.y *= -1.0;
            self.position += (self.speed * self.direction) * delta_time;
            todo!("account for ball height");
        } else {
            self.position = new_position;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, self.color);
    }
}