use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::prelude::{screen_height, screen_width, Color};
use rand::{Rng, RngExt};
use crate::ball::Ball;
use crate::paddle::Paddle;

pub struct EntityFactory { }
impl EntityFactory {
    const paddle_height: f32 = 50.0;
    const paddle_width: f32 = 10.0;
    const paddle_speed: f32 = 250.0;
    const paddle_color: Color = WHITE;
    const paddle_off_wall: f32 = 5.0;

    const ball_height: f32 = 10.0;
    const ball_width: f32 = 10.0;
    const ball_speed: f32 = 200.0;
    const ball_color: Color = WHITE;

    pub fn create_paddle(is_left: bool, is_player: bool) -> Paddle {
        if is_left {
            Paddle {
                position: Vec2::new(Self::paddle_off_wall, EntityFactory::middle_of_screen(screen_height(), Self::paddle_height)),
                size: Vec2::new(Self::paddle_width, Self::paddle_height),
                is_left,
                is_player,
                speed: Self::paddle_speed,
                color: Self::paddle_color,
            }
        } else {
            Paddle {
                position: Vec2::new(screen_width() - Self::paddle_off_wall - Self::paddle_width, EntityFactory::middle_of_screen(screen_height(), Self::paddle_height)),
                size: Vec2::new(Self::paddle_width, Self::paddle_height),
                is_left,
                is_player,
                speed: Self::paddle_speed,
                color: Self::paddle_color,
            }
        }
    }

    pub fn create_ball() -> Ball {
        let mut rng = rand::rng();
        let start_left = rng.random_bool(0.5);
        let random_x: f32 = rng.random();
        let random_y: f32 = 1.0 - random_x;

        if start_left {
            random_x * -1.0;
        }


        Ball {
            position: Vec2::new(
                Self::middle_of_screen(screen_width(), Self::ball_width),
                Self::middle_of_screen(screen_height(), Self::ball_height)
            ),
            direction: Vec2 {
                x: random_x,
                y: random_y
            },
            size:  Vec2::new(Self::ball_width, Self::ball_height),
            speed: Self::ball_speed,
            color: Self::ball_color,
        }
    }

    fn middle_of_screen(screen_size: f32, rectangle_size: f32) -> f32 {
        screen_size / 2.0 - rectangle_size / 2.0
    }
}