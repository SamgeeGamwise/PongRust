use macroquad::prelude::{
    draw_text, is_key_pressed, KeyCode, WHITE, YELLOW,
};

use crate::events::state_events::StateEvent;
use crate::game_state::GameState;
use crate::state_machine::play_state::PlayState;
use crate::state_machine::pre_play_state::PrePlayState;
use super::state::State;

pub struct MenuState {
    selected_index: usize,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
        }
    }

    fn move_up(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = 2;
        } else {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        self.selected_index = (self.selected_index + 1) % 3;
    }
}

impl State for MenuState {
    fn update(&mut self, _delta_time: f32) -> StateEvent {
        if is_key_pressed(KeyCode::W) {
            self.move_up();
        }

        if is_key_pressed(KeyCode::S) {
            self.move_down();
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            match self.selected_index {
                0 => {
                    return StateEvent::SwitchMany(vec![
                        Box::new(PlayState::new(GameState::new(true, false))),
                        Box::new(PrePlayState::new()),
                    ]);
                }

                1 => {
                    return StateEvent::SwitchMany(vec![
                        Box::new(PlayState::new(GameState::new(true, true))),
                        Box::new(PrePlayState::new()),
                    ]);
                }

                2 => {
                    std::process::exit(0);
                }

                _ => {}
            }
        }

        StateEvent::None
    }

    fn draw(&self) {
        draw_text("Pong", 100.0, 80.0, 50.0, WHITE);

        let options = ["1P", "2P", "Quit"];

        for (index, option) in options.iter().enumerate() {
            let y = 150.0 + index as f32 * 40.0;

            let prefix = if index == self.selected_index {
                "> "
            } else {
                "  "
            };

            let color = if index == self.selected_index {
                YELLOW
            } else {
                WHITE
            };

            draw_text(
                &format!("{}{}", prefix, option),
                100.0,
                y,
                30.0,
                color,
            );
        }
    }
}