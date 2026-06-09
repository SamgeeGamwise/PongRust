use macroquad::prelude::{measure_text, BLACK, WHITE};
use macroquad::text::draw_text;
use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;
use crate::game_state::GameState;
use crate::GAME_WIDTH;

pub struct RenderSystem {

}

impl RenderSystem {
    const SCORE_FONT_SIZE: u16 = 20;
    const TITLE_FONT_SIZE: u16 = 50;
    const UI_PADDING: f32 = 10.0;
    const FONT_SCALE: f32 = 1.0;


    pub fn draw(left_paddle: Paddle, right_paddle: Paddle, ball: Ball, game_state: GameState) {
        left_paddle.draw();
        right_paddle.draw();
        ball.draw();
        Self::draw_ui(game_state);
    }

    fn draw_ui(game_state: GameState) {
        let left_score_dimensions = measure_text(game_state.left_paddle_score.to_string(), None, Self::SCORE_FONT_SIZE, Self::FONT_SCALE);
        let right_score_dimensions = measure_text(game_state.right_paddle_score.to_string(), None, Self::SCORE_FONT_SIZE, Self::FONT_SCALE);
        let title_dimensions = measure_text("Pong", None, Self::TITLE_FONT_SIZE, Self::FONT_SCALE);

        draw_text(
            game_state.left_paddle_score.to_string(),
            Self::UI_PADDING,
            Self::UI_PADDING + left_score_dimensions.height,
            Self::SCORE_FONT_SIZE as f32,
            WHITE
        );

        draw_text(
            game_state.right_paddle_score.to_string(),
            GAME_WIDTH - right_score_dimensions.width - Self::UI_PADDING,
            Self::UI_PADDING + right_score_dimensions.height,
            Self::SCORE_FONT_SIZE as f32,
            WHITE
        );
        draw_text(
            "Pong",
            (GAME_WIDTH - title_dimensions.width) / 2.0,
            title_dimensions.height + Self::UI_PADDING,
            Self::TITLE_FONT_SIZE as f32,
            WHITE
        );
    }
}