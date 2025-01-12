#[derive(PartialEq,Clone, Copy)]
pub enum CurrentGameState {
    MenuOpen(CurrentMenuState),
    GameStarting,
    GameRunning,
    GameIsStopping,
    AwaitingKeyPress,
    StoryIsDone,
}

#[derive(PartialEq, Clone,Copy)]
pub enum CurrentMenuState {
    App,
    New,
    Load
}

#[derive(PartialEq,Clone,Copy)]
pub enum CurrentAwaitClickState {
    Chat,
    Choose,
    Menu
}