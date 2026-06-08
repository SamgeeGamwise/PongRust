use macroquad::prelude::screen_height;
use crate::entities::ball::Ball;
use crate::directions::Direction;
use crate::entities::paddle::Paddle;

pub struct MovementSystem {

}

impl MovementSystem {
    pub fn update(delta_time: f32, left_paddle: &mut Paddle, right_paddle: &mut Paddle, ball: &mut Ball) {
        Self::move_paddle(delta_time, left_paddle);
        Self::move_paddle(delta_time, right_paddle);

        Self::move_ball(delta_time, ball);
    }

    pub fn move_paddle(delta_time: f32, paddle: &mut Paddle) {
        match paddle.get_direction() {
            Direction::Up => {
                paddle.position.y -= paddle.speed * delta_time ;
            },
            Direction::Down => {
                paddle.position.y += paddle.speed * delta_time ;
            },
            Direction::None => { },
        }
    }

    pub fn move_ball(delta_time: f32, ball: &mut Ball) {
        let new_position = ball.position + ((ball.speed * ball.direction) * delta_time);

        if new_position.y < 0.0 || new_position.y + ball.size.y > screen_height(){
            println!("Bounce!");
            ball.direction.y *= -1.0;
            ball.position += (ball.speed * ball.direction) * delta_time;
        } else {
            ball.position = new_position;
        }
    }
}