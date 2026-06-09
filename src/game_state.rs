#[derive(Clone, Copy)]
pub struct GameState {
    pub left_paddle_player: bool,
    pub right_paddle_player: bool,
    pub left_paddle_score: i8,
    pub right_paddle_score: i8,
    pub volley_count: i16,
}

impl GameState {
    pub fn new(left_paddle_player: bool, right_paddle_player: bool) -> Self {
        Self {
            left_paddle_player,
            right_paddle_player,
            left_paddle_score: 0,
            right_paddle_score: 0,
            volley_count: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.left_paddle_score = 0;
        self.right_paddle_score = 0;
        self.volley_count = 0;
    }
}