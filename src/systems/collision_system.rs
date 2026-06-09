use macroquad::prelude::Rect;
use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;
use crate::events::game_events::GameEvent;
use crate::{GAME_HEIGHT, GAME_WIDTH};

pub struct CollisionSystem {

}

impl CollisionSystem {
    pub fn update(left_paddle: &mut Paddle, right_paddle: &mut Paddle, ball: &mut Ball, events: &mut Vec<GameEvent>) {
        Self::paddle_hits_wall(left_paddle);
        Self::paddle_hits_wall(right_paddle);
        Self::ball_hits_paddle(left_paddle, ball, events);
        Self::ball_hits_paddle(right_paddle, ball, events);
        Self::ball_hits_wall(ball, events);
        Self::ball_scores(ball, events);
    }

    fn ball_hits_paddle(paddle: &mut Paddle, ball: &mut Ball, events: &mut Vec<GameEvent>) {
        if Self::rects_overlap(paddle.rectangle, ball.rectangle) {
            ball.bounce(true);
            events.push(GameEvent::BallHitPaddle);

            if paddle.is_left {
                ball.rectangle.x = paddle.rectangle.x + paddle.rectangle.w;
            } else {
                ball.rectangle.x = paddle.rectangle.x - ball.rectangle.w;
            }
        }
    }

    fn ball_hits_wall(ball: &mut Ball, events: &mut Vec<GameEvent>) {
        if ball.rectangle.y < 0.0 {
            ball.bounce(false);
            ball.rectangle.y = 0.0;
            events.push(GameEvent::BallHitWall);
        } else if ball.rectangle.y + ball.rectangle.h > GAME_HEIGHT {
            ball.bounce(false);
            ball.rectangle.y = GAME_HEIGHT - ball.rectangle.h;
            events.push(GameEvent::BallHitWall);
        }
    }

    fn ball_scores(ball: &mut Ball, events: &mut Vec<GameEvent>) {
        if ball.rectangle.x + ball.rectangle.w < 0.0 {
            events.push(GameEvent::RightPlayerScored);
        } else if ball.rectangle.x > GAME_WIDTH {
            events.push(GameEvent::LeftPlayerScored);
        }
    }

    fn paddle_hits_wall(paddle: &mut Paddle) {
        if paddle.rectangle.y < 0.0 {
            paddle.rectangle.y = 0.0;
        } else if paddle.rectangle.y + paddle.rectangle.h > GAME_HEIGHT {
            paddle.rectangle.y = GAME_HEIGHT - paddle.rectangle.h;
        }
    }

    fn rects_overlap(a: Rect, b: Rect) -> bool {
        a.x < b.x + b.w &&
            a.x + a.w > b.x &&
            a.y < b.y + b.h &&
            a.y + a.h > b.y
    }
}