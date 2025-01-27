#[derive(PartialEq,Clone)]
pub enum CurrentGameState {
    MenuOpen,
    GameStarting(String),
    GameRunning,
    AppIsStopping,
    StoryIsDone,
}

#[derive(PartialEq, Clone,Copy)]
pub enum CurrentMenuState {
    App(bool),
    New(bool),
    Load(bool),
}

#[derive(PartialEq,Clone,Copy)]
pub enum CurrentAwaitClickState {
    Chat,
    Choose,
    Menu
}

#[derive(PartialEq, Clone,Copy)]
pub struct MenuCursorState {
    pub selected : u8,
    pub total : u8
}