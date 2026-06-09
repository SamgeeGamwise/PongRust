use macroquad::prelude::Rect;
use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;

pub struct CollisionSystem {

}

impl CollisionSystem {
    pub fn update(left_paddle: &mut Paddle, right_paddle: &mut Paddle, ball: &mut Ball) {
        if Self::rects_overlap(left_paddle.rectangle, ball.rectangle) {
            ball.direction.x *= -1.0;
            ball.rectangle.x = left_paddle.rectangle.x + left_paddle.rectangle.w;
        }

        if Self::rects_overlap(right_paddle.rectangle, ball.rectangle) {
            ball.direction.x *= -1.0;
            ball.rectangle.x = right_paddle.rectangle.x - ball.rectangle.w;
        }
    }

    pub fn rects_overlap(a: Rect, b: Rect) -> bool {
        a.x < b.x + b.w &&
            a.x + a.w > b.x &&
            a.y < b.y + b.h &&
            a.y + a.h > b.y
    }
}