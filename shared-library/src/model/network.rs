use crate::model::game::Game;

#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ClientMessage {
    RequestFullUpdate,
}

impl TryFrom<&[u8]> for ClientMessage {
    type Error = ClientMessageTryFromByteSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(bincode::deserialize(value)?)
    }
}

impl TryInto<Vec<u8>> for ClientMessage {
    type Error = ClientMessageTryIntoByteVecError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(bincode::serialize(&self)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ClientMessageTryFromByteSliceError {
    #[error("Failed deserializing ClientMessage ({0})")]
    Deserialize(bincode::Error),
}

impl From<bincode::Error> for ClientMessageTryFromByteSliceError {
    fn from(value: bincode::Error) -> Self {
        Self::Deserialize(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ClientMessageTryIntoByteVecError {
    #[error("Failed serializing ClientMessage ({0})")]
    Serialize(bincode::Error),
}

impl From<bincode::Error> for ClientMessageTryIntoByteVecError {
    fn from(value: bincode::Error) -> Self {
        Self::Serialize(value)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ServerMessage {
    LobbyCreated(Game),
    LobbyJoined(Game),

    GameFullUpdate(Game),
}

impl ServerMessage {
    pub fn replace_this_player_id_with(&mut self, this_player_id: crate::model::game::PlayerId) {
        match self {
            Self::LobbyCreated(game) | Self::LobbyJoined(game) | Self::GameFullUpdate(game) => {
                game.this_player_id = this_player_id;
            }
        }
    }
}

impl TryFrom<&[u8]> for ServerMessage {
    type Error = ServerMessageTryFromByteSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(bincode::deserialize(value)?)
    }
}

impl TryInto<Vec<u8>> for ServerMessage {
    type Error = ServerMessageTryIntoByteVecError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(bincode::serialize(&self)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerMessageTryFromByteSliceError {
    #[error("Failed deserializing ServerMessage ({0})")]
    Deserialize(bincode::Error),
}

impl From<bincode::Error> for ServerMessageTryFromByteSliceError {
    fn from(value: bincode::Error) -> Self {
        Self::Deserialize(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerMessageTryIntoByteVecError {
    #[error("Failed serializing ServerMessage ({0})")]
    Serialize(bincode::Error),
}

impl From<bincode::Error> for ServerMessageTryIntoByteVecError {
    fn from(value: bincode::Error) -> Self {
        Self::Serialize(value)
    }
}
