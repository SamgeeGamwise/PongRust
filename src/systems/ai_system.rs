use crate::entities::ball::Ball;
use crate::directions::Direction;
use crate::entities::paddle::Paddle;

pub struct AiSystem {

}

impl AiSystem {
    const BUFFER: f32 = 5.0;
    pub fn move_paddle(paddle: &mut Paddle, ball: Ball) -> Direction {
        let middle_of_paddle = paddle.position.y + (paddle.size.y / 2.0);
        let middle_of_ball = ball.position.y + (ball.size.y / 2.0);

        if middle_of_paddle - middle_of_ball < -Self::BUFFER {
            Direction::Down
        } else if middle_of_paddle - middle_of_ball > Self::BUFFER {
            Direction::Up
        } else {
            Direction::None
        }
    }

    pub fn update(left_paddle: &mut Paddle, right_paddle: &mut Paddle, ball: Ball) {
        if !left_paddle.is_player {
            left_paddle.direction = Self::move_paddle(left_paddle, ball);
        }

        if !right_paddle.is_player {
            right_paddle.direction = Self::move_paddle(right_paddle, ball);
        }
    }
}