use crate::events::game_events::GameEvent;
use crate::game_state::GameState;

pub struct ScoreSystem { }
impl ScoreSystem {
    pub fn update(events: &mut Vec<GameEvent>, game_state: &mut GameState) {
        for event in &mut *events {
            match event {
                GameEvent::LeftPlayerScored => {
                    game_state.left_paddle_score += 1;
                },
                GameEvent::RightPlayerScored => {
                    game_state.right_paddle_score += 1;
                },
                GameEvent::BallHitLeftPaddle | GameEvent::BallHitRightPaddle => {
                    game_state.volley_count += 1;
                },
                _ => {}
            }
        }
    }
}