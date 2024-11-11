#[derive(PartialEq,Clone, Copy)]
pub enum CurrentGameState {
    MenuOpen,
    GameStarting,
    GameRunning,
    GameIsStopping,
    AwaitingKeyPress,
    StoryIsDone,
}