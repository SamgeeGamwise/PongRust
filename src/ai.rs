use crate::ball::Ball;
use crate::directions::Direction;
use crate::paddle::Paddle;

pub struct Ai {

}

impl Ai {
    const BUFFER: f32 = 5.0;
    pub fn move_paddle(paddle: Paddle, ball: Ball) -> Direction {
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

}