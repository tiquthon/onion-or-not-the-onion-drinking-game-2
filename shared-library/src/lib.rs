#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Game {
    pub game_state: GameState,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameState {
    InLobby,
    Playing,
    Aftermath,
}
