pub enum GameEvent {
    BallHitPaddle,
    BallHitWall,
    LeftPlayerScored,
    RightPlayerScored,

    ResetRound,
    GameOver,
}