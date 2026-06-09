use crate::audio_assets::AudioAssets;
use crate::systems::input_system::InputSystem;
use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;
use crate::entities::entity_factory::EntityFactory;
use crate::events::game_events::GameEvent;
use crate::game_state::GameState;
use crate::systems::*;

pub struct Game {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    pub game_state: GameState,
    pub events: Vec<GameEvent>,
}

impl Game {
    pub fn new(game_state: GameState) -> Self {
        let left_paddle = EntityFactory::create_paddle(true, game_state.left_paddle_player);
        let right_paddle = EntityFactory::create_paddle(false, game_state.right_paddle_player);
        let ball = EntityFactory::create_ball();

        Self {
            left_paddle,
            right_paddle,
            ball,
            events: Vec::new(),
            game_state
        }
    }

    pub fn reset(&mut self) {
        self.ball = EntityFactory::create_ball();
        self.left_paddle = EntityFactory::create_paddle(self.left_paddle.is_left, self.left_paddle.is_player);
        self.right_paddle = EntityFactory::create_paddle(self.right_paddle.is_left, self.right_paddle.is_player);
        self.events.clear();
    }

    pub fn update(&mut self, delta_time: f32, audio_assets: &mut AudioAssets) {
        InputSystem::update(&mut self.left_paddle, &mut self.right_paddle, &mut self.events);
        AiSystem::update(&mut self.left_paddle, &mut self.right_paddle, &self.ball);
        MovementSystem::update(delta_time, &mut self.left_paddle, &mut self.right_paddle, &mut self.ball);
        CollisionSystem::update(&mut self.left_paddle, &mut self.right_paddle, &mut self.ball, &mut self.events);
        ScoreSystem::update(&mut self.events, &mut self.game_state);
        GameStateSystem::update(&mut self.events, &mut self.game_state);
        SfxSystem::update(&mut self.events, audio_assets);
    }

    pub fn draw(&self) -> () {
        RenderSystem::draw(self.left_paddle, self.right_paddle, self.ball, self.game_state);
    }
}
