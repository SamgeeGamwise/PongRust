use macroquad::input::{is_key_down, KeyCode};
use macroquad::input::KeyCode::{Down, Up, S, W};
use crate::directions::Direction;
use crate::entities::paddle::Paddle;

pub struct InputSystem {}
impl InputSystem {
    pub fn update(left_paddle: &mut Paddle, right_paddle: &mut Paddle) {
        Self::set_paddle_direction(left_paddle, W, S);
        Self::set_paddle_direction(right_paddle, Up, Down);
    }

    pub fn set_paddle_direction(paddle: &mut Paddle, up_key: KeyCode, down_key: KeyCode) {
        if is_key_down(up_key) {
            paddle.set_direction(Direction::Up);
        } else if is_key_down(down_key) {
            paddle.set_direction(Direction::Down);
        } else {
            paddle.set_direction(Direction::None);
        }
    }
}