use macroquad::audio::{load_sound, Sound};

pub struct AudioAssets {
    pub game_over: Sound,
    pub pause: Sound,
    pub menu_select: Sound,
    pub wall_hit: Sound,
    pub paddle_hit: Sound,
}

impl AudioAssets {
    pub async fn new() -> Self {
        Self {
            wall_hit: load_sound("assets/sfx/wall_hit.wav").await.unwrap(),
            paddle_hit: load_sound("assets/sfx/paddle_hit.wav").await.unwrap(),
            menu_select: load_sound("assets/sfx/menu_select.wav").await.unwrap(),
            game_over: load_sound("assets/sfx/game_over.wav").await.unwrap(),
            pause: load_sound("assets/sfx/pause.wav").await.unwrap(),
        }
    }
}