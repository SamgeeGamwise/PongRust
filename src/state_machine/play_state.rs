use crate::events::game_events::GameEvent;
use crate::events::state_events::StateEvent;
use crate::game::Game;
use crate::state_machine::pre_play_state::PrePlayState;
use super::state::State;

pub struct PlayState {
    game: Game
}

impl PlayState {
    pub fn new() -> Self {
        let game = Game::new(true, false);

        Self {
            game
        }
    }

}


impl State for PlayState {

    fn enter(&self) -> () {

    }

    fn exit(&self) -> () {

    }

    fn update(&mut self, delta_time: f32) -> StateEvent {
        self.game.update(delta_time);

        for event in &self.game.events {
            match event {
                GameEvent::GameOver => {
                    return StateEvent::Switch(Box::new(PlayState::new()));
                },
                GameEvent::ResetRound => {
                    return StateEvent::Switch(Box::new(PrePlayState::new()));
                }
                _ => { }
            }
        }

        self.game.events.clear();

        StateEvent::None
    }

    fn draw(&self) -> () {
        self.game.draw();
    }
}
