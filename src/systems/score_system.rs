use crate::events::game_events::GameEvent;

pub struct ScoreSystem {
    left_paddle_score: i8,
    right_paddle_score: i8,
    volley_count: i16,
}
impl ScoreSystem {
    const SCORE_TO_WIN: i8 = 10;

    pub fn new() -> Self {
        Self {
            left_paddle_score: 0,
            right_paddle_score: 0,
            volley_count: 0
        }
    }
    pub fn update(&mut self, events: &mut Vec<GameEvent>) {
        for event in &mut *events {
            match event {
                GameEvent::LeftPlayerScored => {
                    self.left_paddle_score += 1;
                },
                GameEvent::RightPlayerScored => {
                    self.right_paddle_score += 1;
                },
                GameEvent::BallHitPaddle => {
                    self.volley_count += 1;
                },
                _ => {}
            }
        }

        if self.left_paddle_score > Self::SCORE_TO_WIN || self.right_paddle_score > Self::SCORE_TO_WIN {
            events.push(GameEvent::GameOver);
        }
    }
}