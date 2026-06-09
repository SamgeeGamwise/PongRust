use crate::entities::ball::Ball;
use crate::directions::Direction;
use crate::entities::paddle::Paddle;
use crate::GAME_HEIGHT;

pub struct MovementSystem {

}

impl MovementSystem {
    pub fn update(delta_time: f32, left_paddle: &mut Paddle, right_paddle: &mut Paddle, ball: &mut Ball) {
        Self::move_paddle(delta_time, left_paddle);
        Self::move_paddle(delta_time, right_paddle);

        Self::move_ball(delta_time, ball);
    }

    pub fn move_paddle(delta_time: f32, paddle: &mut Paddle) {
        match paddle.direction {
            Direction::Up => {
                let mut new_y = paddle.rectangle.y - paddle.speed * delta_time;

                if new_y < 0.0 {
                    new_y = 0.0;
                }

                paddle.rectangle.y = new_y;
            },
            Direction::Down => {
                let mut new_y = paddle.rectangle.y + paddle.speed * delta_time;

                if new_y + paddle.rectangle.h > GAME_HEIGHT {
                    new_y = GAME_HEIGHT - paddle.rectangle.h;
                }

                paddle.rectangle.y = new_y;
            },
            Direction::None => { },
        }
    }

    pub fn move_ball(delta_time: f32, ball: &mut Ball) {
        let new_x = ball.rectangle.x + ((ball.speed * ball.direction.x) * delta_time);
        let new_y = ball.rectangle.y + ((ball.speed * ball.direction.y) * delta_time);

        if new_y < 0.0 || new_y + ball.rectangle.h > GAME_HEIGHT {
            ball.direction.y *= -1.0;
            ball.rectangle.x += (ball.speed * ball.direction.x) * delta_time;
            ball.rectangle.y += (ball.speed * ball.direction.y) * delta_time;
        } else {
            ball.rectangle.x = new_x;
            ball.rectangle.y = new_y;
        }
    }
}