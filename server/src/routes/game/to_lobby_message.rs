use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

use crate::routes::game::from_lobby_message::FromLobbyMessage;

#[derive(Clone, Debug)]
pub enum ToLobbyMessage {
    Register {
        client_info: ClientInfo,
        name: crate::model::PlayerName,
        just_watch: bool,
        register_type: RegisterType,
    },
    Disconnect {
        client_info: ClientInfo,
    },
    IntervalUpdate,
    ClientMessage {
        client_info: ClientInfo,
        client_message: shared_model::network::ClientMessage,
    },
}

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub callback: tokio::sync::mpsc::UnboundedSender<FromLobbyMessage>,
    pub player_id: crate::model::PlayerId,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RegisterType {
    Creator,
    Joiner,
}
