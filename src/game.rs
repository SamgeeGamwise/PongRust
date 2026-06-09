use crate::systems::input_system::InputSystem;
use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;
use crate::entities::entity_factory::EntityFactory;
use crate::events::game_events::GameEvent;
use crate::systems::ai_system::AiSystem;
use crate::systems::collision_system::CollisionSystem;
use crate::systems::game_state_system::GameStateSystem;
use crate::systems::movement_system::MovementSystem;
use crate::systems::render_system::RenderSystem;
use crate::systems::score_system::ScoreSystem;

pub struct Game {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    score_system: ScoreSystem,
    pub events: Vec<GameEvent>,
}

impl Game {
    pub fn new(left_paddle_player: bool, right_paddle_player: bool) -> Self {
        let left_paddle = EntityFactory::create_paddle(true, left_paddle_player);
        let right_paddle = EntityFactory::create_paddle(false, right_paddle_player);
        let ball = EntityFactory::create_ball();

        Self {
            left_paddle,
            right_paddle,
            ball,
            events: Vec::new(),
            score_system: ScoreSystem::new()
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        InputSystem::update(&mut self.left_paddle, &mut self.right_paddle);
        AiSystem::update(&mut self.left_paddle, &mut self.right_paddle, &self.ball);
        MovementSystem::update(delta_time, &mut self.left_paddle, &mut self.right_paddle, &mut self.ball);
        CollisionSystem::update(&mut self.left_paddle, &mut self.right_paddle, &mut self.ball, &mut self.events);
        self.score_system.update(&mut self.events);
        GameStateSystem::update(&mut self.events);
    }

    pub fn draw(&self) -> () {
        RenderSystem::draw(self.left_paddle, self.right_paddle, self.ball);
    }
}
