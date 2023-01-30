use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};

use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, GameState};
use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::{
    ClientMessage, ServerMessage,
};

use wasm_bindgen_futures::spawn_local;

use yew::Callback;

use crate::routes::index::{CreateLobby, JoinLobby};
use crate::routes::play::CreateJoinLobby;

pub enum PlayState {
    ConnectingError {
        error: anyhow::Error,
    },
    Connecting {
        web_socket_stream: Arc<tokio::sync::Mutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<tokio::sync::Mutex<Option<SplitSink<WebSocket, Message>>>>,
    },
    Playing {
        web_socket_stream: Arc<tokio::sync::Mutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<tokio::sync::Mutex<Option<SplitSink<WebSocket, Message>>>>,
        game: Box<Game>,
    },
}

impl PlayState {
    pub fn connect(
        web_socket_address_root: &str,
        on_message_received: Callback<Result<Message, WebSocketError>>,
        on_connection_closed: Callback<()>,
        create_join_lobby: &CreateJoinLobby,
    ) -> Self {
        let web_socket_address = match create_join_lobby {
            CreateJoinLobby::Create(CreateLobby {
                player_name,
                just_watch,
                count_of_questions,
                minimum_score_of_questions,
                timer,
            }) => {
                let player_name = urlencoding::encode(player_name);
                let count_of_questions = count_of_questions
                    .map(|v| format!("&count_of_questions={v}"))
                    .unwrap_or_default();
                let minimum_score_of_questions = minimum_score_of_questions
                    .map(|v| format!("&count_of_questions={v}"))
                    .unwrap_or_default();
                let timer = timer
                    .map(|v| format!("&count_of_questions={v}"))
                    .unwrap_or_default();
                format!("{web_socket_address_root}/create?player_name={player_name}&just_watch={just_watch}{count_of_questions}{minimum_score_of_questions}{timer}")
            }
            CreateJoinLobby::Join(JoinLobby {
                player_name,
                invite_code,
                just_watch,
            }) => {
                format!("{web_socket_address_root}/join/{invite_code}?player_name={player_name}&just_watch={just_watch}")
            }
        };

        log::info!("Connecting to {web_socket_address}");

        match WebSocket::open(&web_socket_address) {
            Ok(web_socket) => {
                let (sink, stream) = web_socket.split();

                let stream_arc = Arc::new(tokio::sync::Mutex::new(Some(stream)));
                let cloned_stream_arc = Arc::clone(&stream_arc);

                let sink_arc = Arc::new(tokio::sync::Mutex::new(Some(sink)));
                let cloned_sink_arc = Arc::clone(&sink_arc);

                spawn_local(async move {
                    let mut last_game_updated_requested_on = wasm_timer::Instant::now();

                    loop {
                        let mut locked_optional_stream =
                            tokio::sync::Mutex::lock(&cloned_stream_arc).await;
                        match &mut *locked_optional_stream {
                            Some(locked_stream) => {
                                let stream_result_optional_poll = futures_util::poll!(
                                    futures_util::stream::StreamExt::next(locked_stream)
                                );

                                // Drop MutexGuard before async sleep, so that other parts can access the Mutex.
                                drop(locked_optional_stream);

                                match stream_result_optional_poll {
                                    Poll::Ready(Some(msg)) => on_message_received.emit(msg),
                                    Poll::Ready(None) => {
                                        // SplitStream returned None value => exiting loop
                                        break;
                                    }
                                    Poll::Pending => {
                                        // No value in stream yet, but stream still open.

                                        // Send a Ping to keep the connection open.
                                        if last_game_updated_requested_on.elapsed().as_secs_f64()
                                            > 30.0
                                        {
                                            last_game_updated_requested_on =
                                                wasm_timer::Instant::now();

                                            let mut locked_optional_sink =
                                                tokio::sync::Mutex::lock(&cloned_sink_arc).await;
                                            if let Some(locked_sink) = &mut *locked_optional_sink {
                                                locked_sink
                                                    .send(Message::Bytes(
                                                        ClientMessage::RequestFullUpdate
                                                            .try_into()
                                                            .unwrap(),
                                                    ))
                                                    .await
                                                    .unwrap();
                                            }
                                            drop(locked_optional_sink);
                                        }

                                        /* No value in stream yet, but stream still open
                                         * => sleep to give possible "exit"-action from user time
                                         * to replace Option<SplitStream<_>> with None
                                         */
                                        gloo_timers::future::sleep(Duration::from_millis(100))
                                            .await;
                                    }
                                }
                            }
                            None => {
                                // No SplitStream exists => exiting loop
                                drop(locked_optional_stream);
                                break;
                            }
                        }
                    }

                    on_connection_closed.emit(());
                });

                PlayState::Connecting {
                    web_socket_stream: stream_arc,
                    web_socket_sink: sink_arc,
                }
            }
            Err(err) => PlayState::ConnectingError {
                error: anyhow::Error::new(err).context(format!(
                    "Failed opening WebSocket to {web_socket_address} connection."
                )),
            },
        }
    }

    pub fn handle_server_message(&mut self, server_message: ServerMessage) -> bool {
        // bool is ShouldRender
        match &server_message {
            ServerMessage::LobbyCreated(game) | ServerMessage::LobbyJoined(game) => {
                match &self {
                    PlayState::Connecting {
                        web_socket_stream,
                        web_socket_sink,
                    } => {
                        *self = PlayState::Playing {
                            web_socket_stream: Arc::clone(web_socket_stream),
                            web_socket_sink: Arc::clone(web_socket_sink),
                            game: Box::new(game.clone()),
                        };
                        true
                    }
                    PlayState::ConnectingError { .. } | PlayState::Playing { .. } => {
                        log::warn!(
                            "Received {server_message:?} but I am in {self:?}; so doing nothing."
                        );
                        // No-Op
                        false
                    }
                }
            }
            ServerMessage::GameFullUpdate(game_update) => {
                match &mut *self {
                    PlayState::Playing { game, .. } => {
                        log::info!("Received Game Full Update");
                        *game = Box::new(game_update.clone());
                        true
                    }
                    PlayState::Connecting { .. } | PlayState::ConnectingError { .. } => {
                        log::warn!(
                            "Received {server_message:?} but I am in {self:?}; so doing nothing."
                        );
                        // No-Op
                        false
                    }
                }
            }
        }
    }

    pub fn wish_for_game_start(&self) {
        if let PlayState::Playing {
            web_socket_sink,
            game,
            ..
        } = self
        {
            if matches!(game.game_state, GameState::InLobby) {
                let cloned_web_socket_sink = Arc::clone(web_socket_sink);
                spawn_local(async move {
                    let mut locked_optional_sink =
                        tokio::sync::Mutex::lock(&cloned_web_socket_sink).await;
                    if let Some(locked_sink) = &mut *locked_optional_sink {
                        let message = ClientMessage::StartGame;
                        locked_sink
                            .send(Message::Bytes(message.try_into().unwrap()))
                            .await
                            .unwrap();
                    } else {
                        log::error!("There is a wish for game start, but I do not have a WebSocket sink to write to.");
                    }
                });
            } else {
                log::error!("There is a wish for game start, but I am not in Playing GameState::InLobby; doing nothing.");
            }
        } else {
            log::error!("There is a wish for game start, but I am in {self:?}; doing nothing.");
        }
    }

    pub fn exit(&mut self, on_closed: Callback<()>) {
        match &self {
            PlayState::Connecting {
                web_socket_stream,
                web_socket_sink,
            }
            | PlayState::Playing {
                web_socket_stream,
                web_socket_sink,
                ..
            } => {
                let cloned_websocket_stream = Arc::clone(web_socket_stream);
                let cloned_web_socket_sink = Arc::clone(web_socket_sink);
                spawn_local(async move {
                    let mut locked_cloned_websocket_stream =
                        tokio::sync::Mutex::lock(&cloned_websocket_stream).await;
                    let mut locked_cloned_websocket_sink =
                        tokio::sync::Mutex::lock(&cloned_web_socket_sink).await;
                    let optional_websocket_stream =
                        Option::take(&mut locked_cloned_websocket_stream);
                    let optional_websocket_sink = Option::take(&mut locked_cloned_websocket_sink);
                    drop(locked_cloned_websocket_sink);
                    drop(locked_cloned_websocket_stream);
                    if let (Some(stream), Some(sink)) =
                        (optional_websocket_stream, optional_websocket_sink)
                    {
                        let websocket = stream.reunite(sink).unwrap();
                        websocket.close(Some(1000), Some("by client")).unwrap();
                    }
                    on_closed.emit(());
                });
            }
            PlayState::ConnectingError { .. } => {}
        }
    }
}

impl Debug for PlayState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayState::ConnectingError { error } => f
                .debug_struct("PlayState:ConnectingError")
                .field("error", error)
                .finish(),
            PlayState::Connecting { .. } => f.debug_struct("PlayState::Connecting").finish(),
            PlayState::Playing { game, .. } => f
                .debug_struct("PlayState::Lobby")
                .field("game", game)
                .finish(),
        }
    }
}
