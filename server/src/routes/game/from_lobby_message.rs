use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

#[derive(Clone, Debug)]
pub enum FromLobbyMessage {
    LobbyCreated(crate::model::Game),
    LobbyJoined(crate::model::Game),

    GameFullUpdate(crate::model::Game),

    AnswerNotInTimeLimit,
    PlayerNameAlreadyInUse,
}

impl FromLobbyMessage {
    pub fn into_server_message(
        self,
        invite_code: crate::model::InviteCode,
        this_player_id: crate::model::PlayerId,
    ) -> shared_model::network::ServerMessage {
        match self {
            FromLobbyMessage::LobbyCreated(game) => {
                shared_model::network::ServerMessage::LobbyCreated(game.into_shared_model_game(
                    invite_code,
                    this_player_id,
                    crate::data::get,
                ))
            }
            FromLobbyMessage::LobbyJoined(game) => {
                shared_model::network::ServerMessage::LobbyJoined(game.into_shared_model_game(
                    invite_code,
                    this_player_id,
                    crate::data::get,
                ))
            }
            FromLobbyMessage::GameFullUpdate(game) => {
                shared_model::network::ServerMessage::GameFullUpdate(game.into_shared_model_game(
                    invite_code,
                    this_player_id,
                    crate::data::get,
                ))
            }
            FromLobbyMessage::AnswerNotInTimeLimit => {
                shared_model::network::ServerMessage::AnswerNotInTimeLimit
            }
            FromLobbyMessage::PlayerNameAlreadyInUse => {
                shared_model::network::ServerMessage::PlayerNameAlreadyInUse
            }
        }
    }
}
