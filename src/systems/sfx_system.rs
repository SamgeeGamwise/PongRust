use macroquad::audio::play_sound_once;
use crate::audio_assets::AudioAssets;
use crate::events::game_events::GameEvent;

pub struct SfxSystem {

}

impl SfxSystem {
    pub fn update(events: &mut Vec<GameEvent>, audio_assets: &mut AudioAssets) {
        for event in &mut *events {
            match event {
                GameEvent::LeftPlayerScored | GameEvent::RightPlayerScored => {
                    play_sound_once(&audio_assets.pause);
                },
                GameEvent::GameOver => {
                    play_sound_once(&audio_assets.game_over);
                },
                GameEvent::BallHitLeftPaddle | GameEvent::BallHitRightPaddle => {
                    play_sound_once(&audio_assets.paddle_hit);
                },
                GameEvent::BallHitTopWall | GameEvent::BallHitBottomWall => {
                    play_sound_once(&audio_assets.wall_hit);
                },
                GameEvent::Pause => {
                    play_sound_once(&audio_assets.pause);
                }
                _ => {}
            }
        }
    }
}