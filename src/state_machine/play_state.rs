use macroquad::prelude::{is_key_down};
use macroquad::prelude::KeyCode::{Down, Up, S, W};
use crate::ai::Ai;
use crate::ball::Ball;
use crate::directions::Direction;
use crate::entity_factory::EntityFactory;
use crate::paddle::Paddle;
use super::state::State;

pub struct PlayState {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
}

impl PlayState {
    pub fn new(left_paddle_player: bool, right_paddle_player: bool) -> Self {
        let left_paddle = EntityFactory::create_paddle(true, left_paddle_player);
        let right_paddle = EntityFactory::create_paddle(false, right_paddle_player);
        let ball = EntityFactory::create_ball();

        Self {
            left_paddle,
            right_paddle,
            ball
        }
    }

}


impl State for PlayState {

    fn enter(&self) -> () {

    }

    fn exit(&self) -> () {

    }

    fn update(&mut self, delta_time: f32) -> () {
        let left_direction = if self.left_paddle.is_player {
            if is_key_down(W) {
                Direction::Up
            } else if is_key_down(S) {
                Direction::Down
            } else {
                Direction::None
            }
        } else {
            Ai::move_paddle(self.left_paddle, self.ball)
        };

        let right_direction = if self.right_paddle.is_player {
            if is_key_down(Up) {
                Direction::Up
            } else if is_key_down(Down) {
                Direction::Down
            } else {
                Direction::None
            }
        } else {
            Ai::move_paddle(self.right_paddle, self.ball)
        };


        self.left_paddle.update(delta_time, left_direction);
        self.right_paddle.update(delta_time, right_direction);
        self.ball.update(delta_time);
    }

    fn draw(&self) -> () {
        self.left_paddle.draw();
        self.right_paddle.draw();
        self.ball.draw();
    }
}
