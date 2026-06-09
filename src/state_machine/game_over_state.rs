use macroquad::audio::play_sound_once;
use macroquad::prelude::{
    draw_text, is_key_pressed, KeyCode, WHITE, YELLOW,
};
use crate::audio_assets::AudioAssets;
use crate::events::state_events::StateEvent;
use crate::game_state::GameState;
use crate::state_machine::menu_state::MenuState;
use crate::state_machine::play_state::PlayState;
use crate::state_machine::pre_play_state::PrePlayState;

use super::state::State;

pub struct GameOverState {
    selected_index: usize,
    game_state: GameState,
}

impl GameOverState {
    pub fn new(game_state: GameState) -> Self {
        Self {
            game_state,
            selected_index: 0,
        }
    }

    fn move_up(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = 1;
        } else {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        self.selected_index = (self.selected_index + 1) % 2;
    }
}

impl State for GameOverState {
    fn update(&mut self, _delta_time: f32, audio_assets: &mut AudioAssets) -> StateEvent {
        if is_key_pressed(KeyCode::W) {
            play_sound_once(&audio_assets.menu_select);
            self.move_up();
        }

        if is_key_pressed(KeyCode::S) {
            play_sound_once(&audio_assets.menu_select);
            self.move_down();
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            play_sound_once(&audio_assets.menu_select);

            match self.selected_index {
                0 => {
                    self.game_state.reset();
                    
                    return StateEvent::SwitchMany(vec![
                        Box::new(PlayState::new(self.game_state) ),
                        Box::new(PrePlayState::new()),
                    ]);
                }

                1 => {
                    return StateEvent::Switch(Box::new(MenuState::new()));
                }

                _ => {}
            }
        }

        StateEvent::None
    }

    fn draw(&self) {
        draw_text("Game Over!", 100.0, 100.0, 40.0, WHITE);

        let options = ["Play Again", "Go To Menu"];

        for (index, option) in options.iter().enumerate() {
            let y = 160.0 + index as f32 * 35.0;

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
                28.0,
                color,
            );
        }
    }

    fn blocks_draw_below(&self) -> bool {
        false
    }
}