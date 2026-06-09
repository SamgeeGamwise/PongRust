use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;
use crate::events::game_events::GameEvent;
use crate::GAME_HEIGHT;
use macroquad::prelude::*;
use macroquad_particles::{ColorCurve, Emitter, EmitterConfig};

const PARTICLE_AMOUNT: u32 = 12;
const PARTICLE_LIFETIME: f32 = 0.45;
const PARTICLE_SPEED: f32 = 42.0;
const PARTICLE_SIZE: f32 = 2.0;

const PADDLE_IMPACT_OFFSET: f32 = 55.0;
const WALL_IMPACT_OFFSET: f32 = 2.0;

struct ParticleBurst {
    emitter: Emitter,
    position: Vec2,
    timer: f32,
}

pub struct ParticleSystem {
    bursts: Vec<ParticleBurst>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self { bursts: Vec::new() }
    }

    pub fn update(
        &mut self,
        events: &[GameEvent],
        ball: &Ball,
        left_paddle: &Paddle,
        right_paddle: &Paddle,
    ) {
        for event in events {
            let Some((position, direction)) =
                Self::impact_position_and_direction(event, ball, left_paddle, right_paddle)
            else {
                continue;
            };

            self.spawn_burst(position, direction);
        }
    }

    pub fn draw(&mut self) {
        let dt = get_frame_time();

        for burst in &mut self.bursts {
            burst.timer -= dt;
            burst.emitter.draw(burst.position);
        }

        self.bursts.retain(|burst| burst.timer > 0.0);
    }

    fn spawn_burst(&mut self, position: Vec2, direction: Vec2) {
        let emitter = Emitter::new(Self::emitter_config(direction));

        self.bursts.push(ParticleBurst {
            emitter,
            position,
            timer: PARTICLE_LIFETIME,
        });
    }

    fn impact_position_and_direction(
        event: &GameEvent,
        ball: &Ball,
        left_paddle: &Paddle,
        right_paddle: &Paddle,
    ) -> Option<(Vec2, Vec2)> {
        let ball_center = Self::ball_center(ball);
        let ball_radius = ball.rectangle.h / 2.0;

        match event {
            GameEvent::BallHitLeftPaddle => Some((
                vec2(
                    left_paddle.rectangle.x
                        + left_paddle.rectangle.w
                        + PADDLE_IMPACT_OFFSET,
                    ball_center.y,
                ),
                vec2(1.0, 0.0),
            )),

            GameEvent::BallHitRightPaddle => Some((
                vec2(
                    right_paddle.rectangle.x - PADDLE_IMPACT_OFFSET,
                    ball_center.y,
                ),
                vec2(-1.0, 0.0),
            )),

            GameEvent::BallHitTopWall => Some((
                vec2(
                    ball_center.x,
                    ball_radius + WALL_IMPACT_OFFSET,
                ),
                vec2(0.0, 1.0),
            )),

            GameEvent::BallHitBottomWall => Some((
                vec2(
                    ball_center.x,
                    GAME_HEIGHT - ball_radius - WALL_IMPACT_OFFSET,
                ),
                vec2(0.0, -1.0),
            )),

            _ => None,
        }
    }

    fn ball_center(ball: &Ball) -> Vec2 {
        vec2(
            ball.rectangle.x + ball.rectangle.w / 2.0,
            ball.rectangle.y + ball.rectangle.h / 2.0,
        )
    }

    fn emitter_config(direction: Vec2) -> EmitterConfig {
        EmitterConfig {
            local_coords: false,
            one_shot: true,
            emitting: true,

            amount: PARTICLE_AMOUNT,
            lifetime: PARTICLE_LIFETIME,
            lifetime_randomness: 0.45,
            explosiveness: 1.0,

            initial_velocity: PARTICLE_SPEED,
            initial_velocity_randomness: 0.65,
            initial_direction: direction,
            initial_direction_spread: std::f32::consts::PI * 0.85,

            size: PARTICLE_SIZE,
            size_randomness: 0.7,

            gravity: Vec2::ZERO,

            colors_curve: ColorCurve {
                start: WHITE,
                mid: Color::new(1.0, 1.0, 1.0, 0.45),
                end: Color::new(1.0, 1.0, 1.0, 0.0),
            },

            ..Default::default()
        }
    }
}