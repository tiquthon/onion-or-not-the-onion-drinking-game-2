use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ToLobbyMessage {
    Register {
        name: crate::model::PlayerName,
        just_watch: bool,
        register_type: RegisterType,
    },
    Disconnect,
    ClientMessage(shared_model::network::ClientMessage),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RegisterType {
    Creator,
    Joiner,
}
