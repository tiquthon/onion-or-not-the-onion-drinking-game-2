use std::sync::Arc;

use actix_web::{web, Error, HttpRequest, HttpResponse};

use actix_ws::Message;

use futures_util::StreamExt;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::{
    ClientMessage, ServerMessage,
};

#[tracing::instrument(name = "Websocket", skip(req, body, server))]
pub async fn ws(
    req: HttpRequest,
    body: web::Payload,
    server: web::Data<Arc<tokio::sync::Mutex<crate::model::Server>>>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let server: Arc<tokio::sync::Mutex<crate::model::Server>> = Arc::clone(&server);

    actix_web::rt::spawn(async move {
        let mut connection_store = ConnectionStore::new();

        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Binary(bytes) => {
                    match ClientMessage::try_from(&*bytes) {
                        Ok(client_message) => {
                            if let Some(server_message) =
                                handle_message(client_message, &server, &mut connection_store).await
                            {
                                let binary_server_message_result: Result<Vec<u8>, _> =
                                    server_message.try_into();
                                match binary_server_message_result {
                                    Ok(binary_server_message) => {
                                        if let Err(error) =
                                            session.binary(binary_server_message).await
                                        {
                                            // TODO: HANDLE CLOSED ERROR
                                            tracing::error!(
                                                "It seems like client closed connection ({error})."
                                            );
                                        }
                                    }
                                    Err(error) => {
                                        // TODO: HANDLE ERROR
                                        tracing::error!(
                                            "Failed converting ServerMessage ({error})."
                                        );
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            // TODO: HANDLE ERROR
                            tracing::error!("Failed parsing ClientMessage ({error}).");
                        }
                    }
                }
                Message::Close(optional_close_reason) => {
                    tracing::info!(
                        "WebSocket connection closed by client ({optional_close_reason:?})"
                    );
                    return;
                }
                Message::Ping(bytes) => {
                    if let Err(error) = session.pong(&bytes).await {
                        tracing::info!("Connection closed while sending pong ({error})");
                        return;
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

        tracing::info!("Closing WebSocket connection, because received None message");
        let _ = session.close(None).await;
    });

    Ok(response)
}

async fn handle_message(
    message: ClientMessage,
    server: &Arc<tokio::sync::Mutex<crate::model::Server>>,
    connection_store: &mut ConnectionStore,
) -> Option<ServerMessage> {
    match message {
        ClientMessage::CreateLobby {
            player_name,
            just_watch,
            count_of_questions,
            minimum_score_per_question,
            maximum_answer_time_per_question,
        } => {
            handler_create_lobby(
                server,
                connection_store,
                player_name,
                just_watch,
                count_of_questions,
                minimum_score_per_question,
                maximum_answer_time_per_question,
            )
            .await
        }
        ClientMessage::JoinLobby { .. } => {
            tracing::info!("JOIN LOBBY");
            None
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct ConnectionStore {
    connected_player_id: crate::model::PlayerId,
    in_lobby: Option<crate::model::InviteCode>,
}

impl ConnectionStore {
    pub fn new() -> Self {
        Self {
            connected_player_id: crate::model::PlayerId::generate(),
            in_lobby: None,
        }
    }
}

impl Default for ConnectionStore {
    fn default() -> Self {
        Self::new()
    }
}

async fn handler_create_lobby(
    server: &Arc<tokio::sync::Mutex<crate::model::Server>>,
    connection_store: &mut ConnectionStore,
    player_name: String,
    just_watch: bool,
    count_of_questions: Option<u64>,
    minimum_score_per_question: Option<i64>,
    maximum_answer_time_per_question: Option<u64>,
) -> Option<ServerMessage> {
    let new_invite_code = crate::model::InviteCode::generate();

    let new_game = crate::model::Game {
        configuration: crate::model::GameConfiguration {
            count_of_questions,
            minimum_score_per_question,
            maximum_answer_time_per_question,
        },
        game_state: crate::model::GameState::InLobby,
        players: vec![crate::model::Player {
            id: connection_store.connected_player_id,
            name: player_name.try_into().unwrap(),
            play_type: match just_watch {
                true => crate::model::PlayType::Watcher,
                false => crate::model::PlayType::Player { points: 0 },
            },
        }],
    };

    connection_store.in_lobby = Some(new_invite_code.clone());

    let mut locked_server = server.lock().await;

    locked_server
        .games
        .insert(new_invite_code.clone(), new_game.clone());

    drop(locked_server);

    Some(ServerMessage::LobbyCreated(
        new_game.into_shared_model_game(
            new_invite_code,
            connection_store.connected_player_id,
            crate::data::get,
        ),
    ))
}
