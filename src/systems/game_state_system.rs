use crate::events::game_events::GameEvent;
use crate::game_state::GameState;

pub struct GameStateSystem;

impl GameStateSystem {
    const SCORE_TO_WIN: i8 = 3;

    pub fn update(events: &mut Vec<GameEvent>, game_state: &mut GameState) {
        if game_state.left_paddle_score >= Self::SCORE_TO_WIN || game_state.right_paddle_score >= Self::SCORE_TO_WIN {
            events.push(GameEvent::GameOver);
        } else {
            let mut new_events = Vec::new();

            for event in &mut *events {
                match event {
                    GameEvent::LeftPlayerScored | GameEvent::RightPlayerScored => {
                        new_events.push(GameEvent::ResetRound);
                    }
                    _ => {}
                }
            }

            events.append(&mut new_events);
        }
    }
}