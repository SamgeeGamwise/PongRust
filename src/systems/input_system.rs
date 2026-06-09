use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::input::KeyCode::{Down, Up, S, W};
use macroquad::prelude::KeyCode::Space;
use crate::directions::Direction;
use crate::entities::paddle::Paddle;
use crate::events::game_events::GameEvent;

pub struct InputSystem {}
impl InputSystem {
    pub fn update(left_paddle: &mut Paddle, right_paddle: &mut Paddle, events: &mut Vec<GameEvent>) {
        if is_key_pressed(Space) {
            events.push(GameEvent::Pause);
        }

        if left_paddle.is_player {  
            Self::set_paddle_direction(left_paddle, W, S);
        }
        
        if right_paddle.is_player {
            Self::set_paddle_direction(right_paddle, Up, Down);
        }
    }

    pub fn set_paddle_direction(paddle: &mut Paddle, up_key: KeyCode, down_key: KeyCode) {
        if is_key_down(up_key) {
            paddle.direction = Direction::Up;
        } else if is_key_down(down_key) {
            paddle.direction = Direction::Down;
        } else {
            paddle.direction = Direction::None;
        }
    }
}