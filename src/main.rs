pub mod directions;
pub mod state_machine;
pub mod systems;
pub mod entities;
mod game;
pub mod events;
pub mod timer;

use macroquad::prelude::*;
use crate::state_machine::play_state::PlayState;
use crate::state_machine::pre_play_state::PrePlayState;
use crate::state_machine::state_machine::StateMachine;

pub const GAME_WIDTH: f32 = 720.0;
pub const GAME_HEIGHT: f32 = 480.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut playing = true;
    let mut state_machine = StateMachine::new();

    state_machine.push(Box::new(PrePlayState::new()));


    while playing {
        if is_key_pressed(KeyCode::Escape) {
            playing = false;
        }

        let delta_time = get_frame_time();


        state_machine.update(delta_time);

        let scale = f32::min(
            screen_width() / GAME_WIDTH,
            screen_height() / GAME_HEIGHT,
        );

        let viewport_width = (GAME_WIDTH * scale).round();
        let viewport_height = (GAME_HEIGHT * scale).round();

        let viewport_x = ((screen_width() - viewport_width) / 2.0).round();
        let viewport_y_top = ((screen_height() - viewport_height) / 2.0).round();
        let viewport_y = (screen_height() - viewport_y_top - viewport_height).round();

        clear_background(WHITE);

        set_camera(&Camera2D {
            target: vec2(GAME_WIDTH / 2.0, GAME_HEIGHT / 2.0),
            zoom: vec2(2.0 / GAME_WIDTH, 2.0 / GAME_HEIGHT),
            viewport: Some((
                viewport_x as i32,
                viewport_y as i32,
                viewport_width as i32,
                viewport_height as i32,
            )),
            ..Default::default()
        });

        draw_rectangle(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT, BLACK);
        state_machine.draw();

        set_default_camera();

        next_frame().await
    }
}