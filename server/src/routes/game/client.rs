use actix_ws::{Message, MessageStream, Session};

use futures_util::StreamExt;

use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

use crate::routes::game::from_lobby_message::FromLobbyMessage;
use crate::routes::game::lobbies_storage::LobbiesStorage;
use crate::routes::game::to_lobby_message::{ClientInfo, RegisterType, ToLobbyMessage};

pub async fn start_client_network_task(
    player_name: crate::model::PlayerName,
    invite_code: crate::model::InviteCode,
    just_watch: bool,
    lobbies: LobbiesStorage,
    mut session: Session,
    mut msg_stream: MessageStream,
    client_type: ClientType,
) {
    let (unbounded_sender_to_lobby, mut broadcast_receiver_from_lobby) =
        lobbies.retrieve(&invite_code).await.unwrap();
    let (unbounded_sender_from_lobby, mut unbounded_receiver_from_lobby) =
        tokio::sync::mpsc::unbounded_channel::<FromLobbyMessage>();

    let this_player_id = crate::model::PlayerId::generate();

    let mut cloned_session = session.clone();
    let cloned_invite_code = invite_code.clone();
    tokio::spawn(async move {
        while let Ok(from_lobby_message) = broadcast_receiver_from_lobby.recv().await {
            let server_message =
                from_lobby_message.into_server_message(cloned_invite_code.clone(), this_player_id);
            let server_message: Vec<u8> = server_message.try_into().unwrap();
            cloned_session.binary(server_message).await.unwrap();
        }
    });

    let mut cloned_session = session.clone();
    let cloned_invite_code = invite_code.clone();
    tokio::spawn(async move {
        while let Some(from_lobby_message) = unbounded_receiver_from_lobby.recv().await {
            let server_message =
                from_lobby_message.into_server_message(cloned_invite_code.clone(), this_player_id);
            let server_message: Vec<u8> = server_message.try_into().unwrap();
            cloned_session.binary(server_message).await.unwrap();
        }
    });

    tokio::task::spawn_local(async move {
        unbounded_sender_to_lobby
            .send(ToLobbyMessage::Register {
                client_info: ClientInfo {
                    callback: unbounded_sender_from_lobby.clone(),
                    player_id: this_player_id,
                },
                name: player_name,
                just_watch,
                register_type: client_type.into(),
            })
            .unwrap();

        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Binary(bytes) => {
                    match shared_model::network::ClientMessage::try_from(&*bytes) {
                        Ok(client_message) => {
                            unbounded_sender_to_lobby
                                .send(ToLobbyMessage::ClientMessage {
                                    client_info: ClientInfo {
                                        callback: unbounded_sender_from_lobby.clone(),
                                        player_id: this_player_id,
                                    },
                                    client_message,
                                })
                                .unwrap();
                        }
                        Err(error) => {
                            // TODO: HANDLE ERROR
                            tracing::error!("Failed parsing ClientMessage ({error}).");
                            break;
                        }
                    }
                }
                Message::Close(optional_close_reason) => {
                    tracing::info!(
                        "WebSocket connection closed by client ({optional_close_reason:?})"
                    );
                    unbounded_sender_to_lobby
                        .send(ToLobbyMessage::Disconnect {
                            client_info: ClientInfo {
                                callback: unbounded_sender_from_lobby.clone(),
                                player_id: this_player_id,
                            },
                        })
                        .unwrap();
                    break;
                }
                Message::Ping(bytes) => {
                    if let Err(error) = session.pong(&bytes).await {
                        tracing::info!("Connection closed while sending pong ({error})");
                        break;
                    }
                }
                Message::Continuation(item) => {
                    // TODO: Maybe don't ignore
                    // "Websocket protocol continuation frame" https://stackoverflow.com/a/25409934
                    tracing::warn!("Got Continuation Frame with data ({item:?})");
                }
                Message::Text(_) | Message::Pong(_) | Message::Nop => {
                    // IGNORE
                }
            }
        }
    });
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ClientType {
    LobbyCreator,
    LobbyJoiner,
}

#[allow(clippy::from_over_into)]
impl Into<RegisterType> for ClientType {
    fn into(self) -> RegisterType {
        match self {
            ClientType::LobbyCreator => RegisterType::Creator,
            ClientType::LobbyJoiner => RegisterType::Joiner,
        }
    }
}
