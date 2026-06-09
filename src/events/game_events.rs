pub enum GameEvent {
    BallHitPaddle,
    BallHitWall,
    LeftPlayerScored,
    RightPlayerScored,

    Pause,
    ResetRound,
    GameOver,
}