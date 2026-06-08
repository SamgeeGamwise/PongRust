use macroquad::prelude::Rect;
use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;

pub struct CollisionSystem {

}

impl CollisionSystem {
    pub fn update(delta_time: f32, left_paddle: &mut Paddle, right_paddle: &mut Paddle, ball: &mut Ball) {

    }

    pub fn rects_overlap(a: Rect, b: Rect) -> bool {
        a.x < b.x + b.w &&
            a.x + a.w > b.x &&
            a.y < b.y + b.h &&
            a.y + a.h > b.y
    }
}