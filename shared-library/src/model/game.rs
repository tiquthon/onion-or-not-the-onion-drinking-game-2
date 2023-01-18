#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game {
    // TODO: pub invite_code: String,
    pub game_state: GameState,
    // TODO: pub players: Vec<Player>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            game_state: GameState::None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GameState {
    InLobby,
    Playing,
    Aftermath,
    None,
}

pub struct Player {
    // TODO: pub uuid: Uuid,
    pub name: String,
    pub points: u16,
}
