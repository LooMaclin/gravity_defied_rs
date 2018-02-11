#[derive(Debug)]
pub enum GameState {
    Initial,
    LevelRendered,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Initial
    }
}
