use crate::events::game_events::GameEvent;
use crate::events::state_events::StateEvent;
use crate::game::Game;
use crate::game_state::GameState;
use crate::state_machine::game_over_state::GameOverState;
use crate::state_machine::pause_state::PauseState;
use crate::state_machine::pre_play_state::PrePlayState;
use super::state::State;

pub struct PlayState {
    game: Game
}

impl PlayState {
    pub fn new(game_state: GameState) -> Self {
        let game = Game::new(game_state);

        Self {
            game
        }
    }

}


impl State for PlayState {
    fn update(&mut self, delta_time: f32) -> StateEvent {
        self.game.update(delta_time);

        for event in &self.game.events {
            match event {
                GameEvent::Pause => {
                    return StateEvent::Push(Box::new(PauseState::new()));
                },
                GameEvent::GameOver => {
                    return StateEvent::Push(Box::new(GameOverState::new(self.game.game_state)));
                },
                GameEvent::ResetRound => {
                    self.game.reset();
                    return StateEvent::Push(Box::new(PrePlayState::new()));
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
