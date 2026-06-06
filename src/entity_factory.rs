use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::prelude::{screen_height, screen_width, Color};
use rand::{ RngExt };
use crate::ball::Ball;
use crate::paddle::Paddle;

pub struct EntityFactory { }
impl EntityFactory {
    const PADDLE_HEIGHT: f32 = 50.0;
    const PADDLE_WIDTH: f32 = 10.0;
    const PADDLE_SPEED: f32 = 250.0;
    const PADDLE_COLOR: Color = WHITE;
    const PADDLE_OFF_WALL: f32 = 5.0;

    const BALL_HEIGHT: f32 = 10.0;
    const BALL_WIDTH: f32 = 10.0;
    const BALL_SPEED: f32 = 200.0;
    const BALL_COLOR: Color = WHITE;

    pub fn create_paddle(is_left: bool, is_player: bool) -> Paddle {
        if is_left {
            Paddle {
                position: Vec2::new(Self::PADDLE_OFF_WALL, EntityFactory::middle_of_screen(screen_height(), Self::PADDLE_HEIGHT)),
                size: Vec2::new(Self::PADDLE_WIDTH, Self::PADDLE_HEIGHT),
                is_left,
                is_player,
                speed: Self::PADDLE_SPEED,
                color: Self::PADDLE_COLOR,
            }
        } else {
            Paddle {
                position: Vec2::new(screen_width() - Self::PADDLE_OFF_WALL - Self::PADDLE_WIDTH, EntityFactory::middle_of_screen(screen_height(), Self::PADDLE_HEIGHT)),
                size: Vec2::new(Self::PADDLE_WIDTH, Self::PADDLE_HEIGHT),
                is_left,
                is_player,
                speed: Self::PADDLE_SPEED,
                color: Self::PADDLE_COLOR,
            }
        }
    }

    pub fn create_ball() -> Ball {
        let mut rng = rand::rng();
        let start_left = rng.random_bool(0.5);
        let mut random_x: f32 = rng.random();
        let random_y: f32 = 1.0 - random_x;

        if start_left {
            random_x *= -1.0;
        }


        Ball {
            position: Vec2::new(
                Self::middle_of_screen(screen_width(), Self::BALL_WIDTH),
                Self::middle_of_screen(screen_height(), Self::BALL_HEIGHT)
            ),
            direction: Vec2 {
                x: random_x,
                y: random_y
            },
            size:  Vec2::new(Self::BALL_WIDTH, Self::BALL_HEIGHT),
            speed: Self::BALL_SPEED,
            color: Self::BALL_COLOR,
        }
    }

    fn middle_of_screen(screen_size: f32, rectangle_size: f32) -> f32 {
        screen_size / 2.0 - rectangle_size / 2.0
    }
}