use crate::events::game_events::GameEvent;

pub struct GameStateSystem;

impl GameStateSystem {
    pub fn update(events: &mut Vec<GameEvent>) {
        let mut new_events = Vec::new();

        for event in &mut *events {
            match event {
                GameEvent::LeftPlayerScored | GameEvent::RightPlayerScored => {
                    new_events.push(GameEvent::ResetRound);
                },
                _ => {}
            }
        }
        
        events.append(&mut new_events);
    }
}