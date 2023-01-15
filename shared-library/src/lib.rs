#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Game {
    pub game_state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            game_state: GameState::None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameState {
    InLobby,
    Playing,
    Aftermath,
    None,
}
