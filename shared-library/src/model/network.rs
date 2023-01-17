use crate::model::game::Game;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientMessage {
    CreateLobby {
        player_name: String,
        count_of_questions: Option<u64>,
        minimum_score_of_questions: Option<u64>,
        timer: Option<u64>,
    },
    JoinLobby {
        player_name: String,
        invite_code: String,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerMessage {
    LobbyCreated,
    LobbyJoined,
    GameFullUpdate(Game),
}
