pub enum GameEvent {
    BallHitLeftPaddle,
    BallHitRightPaddle,
    BallHitTopWall,
    BallHitBottomWall,
    LeftPlayerScored,
    RightPlayerScored,

    Pause,
    ResetRound,
    GameOver,
}