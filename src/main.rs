pub mod directions;
pub mod state_machine;
pub mod systems;
pub mod entities;
mod game;

use macroquad::prelude::*;
use crate::state_machine::play_state::PlayState;
use crate::state_machine::state_machine::StateMachine;

#[macroquad::main("MyGame")]
async fn main() {
    let mut playing = true;
    let mut state_machine = StateMachine::new();

    (&mut state_machine).push(Box::new(PlayState::new()));


    while playing {
        if is_key_pressed(KeyCode::Escape) {
            playing = false;
        }

        let delta_time = get_frame_time();
        (&mut state_machine).update(delta_time);


        clear_background(BLACK);

        (&mut state_machine).draw();


        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}