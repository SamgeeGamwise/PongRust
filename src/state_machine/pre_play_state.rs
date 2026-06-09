use crate::events::game_events::GameEvent;
use crate::events::state_events::StateEvent;
use crate::game::Game;
use crate::state_machine::play_state::PlayState;
use crate::timer::Timer;
use super::state::State;

pub struct PrePlayState {
    game: Game,
    timer: Timer
}

impl PrePlayState {
    const PRE_GAME_TIMER: f32 = 2.0;

    pub fn new() -> Self {
        let game = Game::new(true, false);
        Self {
            game,
            timer: Timer::new(Self::PRE_GAME_TIMER),
        }
    }

}


impl State for PrePlayState {

    fn enter(&self) -> () {

    }

    fn exit(&self) -> () {

    }

    fn update(&mut self, delta_time: f32) -> StateEvent {
        if self.timer.finished() {
            StateEvent::Switch(Box::new(PlayState::new()))
        } else {
            self.timer.update(delta_time);
            StateEvent::None
        }
    }

    fn draw(&self) -> () {
        self.game.draw();
    }
}
