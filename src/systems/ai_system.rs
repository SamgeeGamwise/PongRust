use crate::entities::ball::Ball;
use crate::directions::Direction;
use crate::entities::paddle::Paddle;
use crate::{GAME_WIDTH, GAME_HEIGHT};

pub struct AiSystem {

}

impl AiSystem {
    const BUFFER: f32 = 2.0;
    pub fn move_paddle(paddle: &mut Paddle, ball: &Ball) -> Direction {
        let middle_of_paddle = paddle.rectangle.y + (paddle.rectangle.h / 2.0);
        let middle_of_ball = ball.rectangle.y + (ball.rectangle.h / 2.0);
        let ball_going_away = paddle.is_left && ball.direction.x > 0.0 || !paddle.is_left && ball.direction.x < 0.0;
        let ball_is_halfway = paddle.is_left && ball.rectangle.x < GAME_WIDTH / 2.0 || !paddle.is_left && ball.rectangle.x > GAME_WIDTH / 2.0;

        if ball_going_away {
            if middle_of_paddle + Self::BUFFER < GAME_HEIGHT / 2.0 {
                Direction::Down
            } else if middle_of_paddle - Self::BUFFER > GAME_HEIGHT / 2.0 {
                Direction::Up
            } else {
                Direction::None
            }
        } else if ball_is_halfway {
            if middle_of_paddle - middle_of_ball < -Self::BUFFER {
                Direction::Down
            } else if middle_of_paddle - middle_of_ball > Self::BUFFER {
                Direction::Up
            } else {
                Direction::None
            }
        } else {
            Direction::None
        }
    }

    pub fn update(left_paddle: &mut Paddle, right_paddle: &mut Paddle, ball: &Ball) {
        if !left_paddle.is_player {
            left_paddle.direction = Self::move_paddle(left_paddle, ball);
        }

        if !right_paddle.is_player {
            right_paddle.direction = Self::move_paddle(right_paddle, ball);
        }
    }
}