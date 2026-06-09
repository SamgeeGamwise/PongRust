use crate::game::Game;
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

    fn update(&mut self, delta_time: f32) -> () {
        self.game.update(delta_time);
    }

    fn draw(&self) -> () {
        self.game.draw();
    }
}
