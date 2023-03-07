use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};

use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    Answer, Game, GameState, PlayingState,
};
use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::{
    ClientMessage, ServerMessage,
};

use wasm_bindgen_futures::spawn_local;

use yew::Callback;

use crate::routes::index::{CreateLobby, JoinLobby};
use crate::routes::play::CreateJoinLobby;

pub enum PlayState {
    ConnectingError {
        locale_keyid: ConnectingErrorLocaleKeyId,
        error: Option<anyhow::Error>,
    },
    None,
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
                minimum_score_per_question,
                maximum_answer_seconds_per_question,
            }) => {
                let player_name = urlencoding::encode(player_name);
                let count_of_questions_str = count_of_questions
                    .map(|v| format!("&count_of_questions={v}"))
                    .unwrap_or_default();
                let minimum_score_per_question_str = minimum_score_per_question
                    .map(|v| format!("&minimum_score_per_question={v}"))
                    .unwrap_or_default();
                let maximum_answer_seconds_per_question_str = maximum_answer_seconds_per_question
                    .map(|v| format!("&maximum_answer_seconds_per_question={v}"))
                    .unwrap_or_default();
                format!("{web_socket_address_root}/create?player_name={player_name}&just_watch={just_watch}{count_of_questions_str}{minimum_score_per_question_str}{maximum_answer_seconds_per_question_str}")
            }
            CreateJoinLobby::Join(JoinLobby {
                player_name,
                invite_code,
                just_watch,
            }) => {
                let player_name = urlencoding::encode(player_name);
                let invite_code = urlencoding::encode(invite_code);
                format!("{web_socket_address_root}/join/{invite_code}?player_name={player_name}&just_watch={just_watch}")
            }
        };

        log::info!("Connecting to {web_socket_address}");

        match WebSocket::open(&web_socket_address) {
            Ok(web_socket) => {
                let (sink, stream) = web_socket.split();

                let web_socket_stream = Arc::new(tokio::sync::Mutex::new(Some(stream)));
                let web_socket_sink = Arc::new(tokio::sync::Mutex::new(Some(sink)));

                let cloned_web_socket_stream = Arc::clone(&web_socket_stream);
                let cloned_web_socket_sink = Arc::clone(&web_socket_sink);

                spawn_local(async move {
                    let mut last_game_updated_requested_on = wasm_timer::Instant::now();

                    loop {
                        let mut locked_optional_stream =
                            tokio::sync::Mutex::lock(&cloned_web_socket_stream).await;
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
                                                tokio::sync::Mutex::lock(&cloned_web_socket_sink)
                                                    .await;
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
                    web_socket_stream,
                    web_socket_sink,
                }
            }
            Err(err) => PlayState::ConnectingError {
                locale_keyid: ConnectingErrorLocaleKeyId::Open,
                error: Some(anyhow::Error::new(err).context(format!(
                    "Failed opening WebSocket to {web_socket_address} connection."
                ))),
            },
        }
    }

    #[must_use]
    pub fn handle_server_message(&self, server_message: ServerMessage) -> Option<Self> {
        match &server_message {
            ServerMessage::LobbyCreated(game) | ServerMessage::LobbyJoined(game) => {
                match &self {
                    Self::Connecting {
                        web_socket_stream,
                        web_socket_sink,
                        ..
                    } => Some(Self::Playing {
                        web_socket_stream: Arc::clone(web_socket_stream),
                        web_socket_sink: Arc::clone(web_socket_sink),
                        game: Box::new(game.clone()),
                    }),
                    Self::ConnectingError { .. } | PlayState::Playing { .. } | PlayState::None => {
                        log::warn!(
                            "Received {server_message:?} but I am in {self:?}; so doing nothing."
                        );
                        // No-Op
                        None
                    }
                }
            }
            ServerMessage::GameFullUpdate(game_update) => {
                match self {
                    Self::Playing {
                        web_socket_stream,
                        web_socket_sink,
                        ..
                    } => {
                        // log::info!("Received full Game update");
                        Some(Self::Playing {
                            web_socket_stream: Arc::clone(web_socket_stream),
                            web_socket_sink: Arc::clone(web_socket_sink),
                            game: Box::new(game_update.clone()),
                        })
                    }
                    PlayState::Connecting { .. }
                    | PlayState::ConnectingError { .. }
                    | PlayState::None => {
                        log::warn!(
                            "Received {server_message:?} but I am in {self:?}; so doing nothing."
                        );
                        // No-Op
                        None
                    }
                }
            }
            ServerMessage::AnswerNotInTimeLimit => {
                log::error!("Sent answer not in time limit.");
                // TODO: Maybe handle error better?
                None
            }
            ServerMessage::PlayerNameAlreadyInUse => {
                match self {
                    PlayState::Connecting { .. } => {
                        self.exit(Default::default());
                        Some(Self::ConnectingError {
                            locale_keyid:
                                ConnectingErrorLocaleKeyId::HandleMessagePlayerNameAlreadyInUse,
                            error: None,
                        })
                    }
                    PlayState::ConnectingError { .. }
                    | PlayState::Playing { .. }
                    | PlayState::None => {
                        log::warn!(
                            "Received {server_message:?} but I am in {self:?}; so doing nothing."
                        );
                        // No-Op
                        None
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
            match game.game_state {
                GameState::InLobby => {
                    send_to_server(Arc::clone(web_socket_sink), ClientMessage::StartGame);
                }
                GameState::Playing { .. } | GameState::Aftermath { .. } => {
                    log::error!("There is a wish for game start, but I am not in Playing GameState::InLobby; doing nothing.");
                }
            }
        } else {
            log::error!("There is a wish for game start, but I am in {self:?}; doing nothing.");
        }
    }

    pub fn choose_answer(&self, answer: Answer) {
        match &self {
            PlayState::Playing {
                web_socket_sink,
                game,
                ..
            } => match game.game_state {
                GameState::Playing {
                    playing_state: PlayingState::Question { .. },
                    ..
                } => {
                    send_to_server(
                        Arc::clone(web_socket_sink),
                        ClientMessage::ChooseAnswer(answer),
                    );
                }
                GameState::Playing {
                    playing_state: PlayingState::Solution { .. },
                    ..
                }
                | GameState::InLobby
                | GameState::Aftermath { .. } => {
                    log::error!("Client wants to choose {answer:?}, but I am not in Playing GameState::Playing PlayingState::Question; doing nothing.");
                }
            },
            PlayState::Connecting { .. } | PlayState::ConnectingError { .. } | PlayState::None => {
                log::error!(
                    "Client wants to choose {answer:?}, but I am in {self:?}; doing nothing."
                );
            }
        }
    }

    pub fn request_skip(&self) {
        match &self {
            PlayState::Playing {
                web_socket_sink,
                game,
                ..
            } => match game.game_state {
                GameState::Playing {
                    playing_state: PlayingState::Solution { .. },
                    ..
                } => {
                    send_to_server(Arc::clone(web_socket_sink), ClientMessage::RequestSkip);
                }
                GameState::Playing {
                    playing_state: PlayingState::Question { .. },
                    ..
                }
                | GameState::InLobby
                | GameState::Aftermath { .. } => {
                    log::error!("Client wants to skip, but I am not in GameState::Playing PlayingState::Solution; doing nothing.");
                }
            },
            PlayState::Connecting { .. } | PlayState::ConnectingError { .. } | PlayState::None => {
                log::error!("Client wants to skip, but I am in {self:?}; doing nothing.");
            }
        }
    }

    pub fn request_play_again(&self) {
        match &self {
            PlayState::Playing {
                web_socket_sink,
                game,
                ..
            } => match game.game_state {
                GameState::Aftermath { .. } => {
                    send_to_server(Arc::clone(web_socket_sink), ClientMessage::RequestPlayAgain);
                }
                GameState::InLobby | GameState::Playing { .. } => {
                    log::error!("Client wants to play again, but I am not in GameState::Aftermath; doing nothing.");
                }
            },
            PlayState::Connecting { .. } | PlayState::ConnectingError { .. } | PlayState::None => {
                log::error!("Client wants to play again, but I am in {self:?}; doing nothing.");
            }
        }
    }

    pub fn exit(&self, on_closed: Callback<()>) {
        match &self {
            PlayState::Connecting {
                web_socket_stream,
                web_socket_sink,
                ..
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
            PlayState::ConnectingError { .. } | PlayState::None => on_closed.emit(()),
        }
    }
}

impl Debug for PlayState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectingError {
                locale_keyid,
                error,
            } => f
                .debug_struct("PlayState:ConnectingError")
                .field("locale_keyid", locale_keyid)
                .field("error", error)
                .finish(),
            Self::Connecting { .. } => f.debug_struct("PlayState::Connecting").finish(),
            Self::Playing { game, .. } => f
                .debug_struct("PlayState::Playing")
                .field("game", game)
                .finish(),
            Self::None => f.debug_struct("PlayState::None").finish(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ConnectingErrorLocaleKeyId {
    Open,
    MessageReceiveConnectionError,
    MessageReceiveConnectionClose,
    MessageReceiveMessageSendError,
    HandleMessagePlayerNameAlreadyInUse,
}

impl ConnectingErrorLocaleKeyId {
    pub fn key_id(&self) -> &'static str {
        match self {
            ConnectingErrorLocaleKeyId::Open => "error-web-socket-open",
            ConnectingErrorLocaleKeyId::MessageReceiveConnectionError => {
                "error-web-socket-message-receive-connection-error"
            }
            ConnectingErrorLocaleKeyId::MessageReceiveConnectionClose => {
                "error-web-socket-message-receive-connection-close"
            }
            ConnectingErrorLocaleKeyId::MessageReceiveMessageSendError => {
                "error-web-socket-message-receive-message-send-error"
            }
            ConnectingErrorLocaleKeyId::HandleMessagePlayerNameAlreadyInUse => {
                "error-web-socket-handle-message-player-name-already-in-use"
            }
        }
    }
}

fn send_to_server(
    web_socket_sink: Arc<tokio::sync::Mutex<Option<SplitSink<WebSocket, Message>>>>,
    client_message: ClientMessage,
) {
    spawn_local(async move {
        let mut locked_optional_web_socket_sink = tokio::sync::Mutex::lock(&web_socket_sink).await;
        if let Some(locked_web_socket_sink) = &mut *locked_optional_web_socket_sink {
            locked_web_socket_sink
                .send(Message::Bytes(client_message.try_into().unwrap()))
                .await
                .unwrap();
        } else {
            log::error!("Wanted to send {client_message:?} to server, but web_socket_sink is None; doing nothing.");
        }
        drop(locked_optional_web_socket_sink);
    });
}
