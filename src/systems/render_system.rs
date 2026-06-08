use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;

pub struct RenderSystem {

}

impl RenderSystem {
    pub fn draw(left_paddle: Paddle, right_paddle: Paddle, ball: Ball) {
        left_paddle.draw();
        right_paddle.draw();
        ball.draw();
    }
}