use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};

use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::Game;
use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::{
    ClientMessage, ServerMessage,
};

use tokio::sync::Mutex as TokioMutex;

use wasm_bindgen_futures::spawn_local;

use yew::Callback;

use crate::routes::index::{CreateLobby, JoinLobby};
use crate::routes::play::CreateJoinLobby;

pub enum PlayState {
    ConnectingError {
        error: anyhow::Error,
    },
    Connecting {
        web_socket_stream: Arc<TokioMutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<TokioMutex<Option<SplitSink<WebSocket, Message>>>>,
    },
    Lobby {
        web_socket_stream: Arc<TokioMutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<TokioMutex<Option<SplitSink<WebSocket, Message>>>>,
        game: Box<Game>,
    },
    // TODO
    #[allow(dead_code)]
    Game {
        web_socket_stream: Arc<TokioMutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<TokioMutex<Option<SplitSink<WebSocket, Message>>>>,
    },
    // TODO
    #[allow(dead_code)]
    Aftermath {
        web_socket_stream: Arc<TokioMutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<TokioMutex<Option<SplitSink<WebSocket, Message>>>>,
    },
}

impl PlayState {
    pub fn connect(
        web_socket_address: &str,
        on_message_received: Callback<Result<Message, WebSocketError>>,
        on_connection_closed: Callback<()>,
        create_join_lobby: &CreateJoinLobby,
    ) -> Self {
        match WebSocket::open(web_socket_address) {
            Ok(web_socket) => {
                let (sink, stream) = web_socket.split();

                let stream_arc = Arc::new(TokioMutex::new(Some(stream)));
                let cloned_stream_arc = Arc::clone(&stream_arc);

                let sink_arc = Arc::new(TokioMutex::new(Some(sink)));
                let cloned_sink_arc = Arc::clone(&sink_arc);

                spawn_local(async move {
                    loop {
                        let mut locked_optional_stream = TokioMutex::lock(&cloned_stream_arc).await;
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

                let initial_message = match create_join_lobby {
                    CreateJoinLobby::Create(CreateLobby {
                        player_name,
                        just_watch,
                        count_of_questions,
                        minimum_score_of_questions,
                        timer,
                    }) => ClientMessage::CreateLobby {
                        player_name: player_name.clone(),
                        just_watch: *just_watch,
                        count_of_questions: *count_of_questions,
                        minimum_score_per_question: *minimum_score_of_questions,
                        maximum_answer_time_per_question: *timer,
                    },
                    CreateJoinLobby::Join(JoinLobby {
                        player_name,
                        invite_code,
                        just_watch,
                    }) => ClientMessage::JoinLobby {
                        player_name: player_name.clone(),
                        invite_code: invite_code.clone(),
                        just_watch: *just_watch,
                    },
                };

                spawn_local(async move {
                    let mut locked_cloned_sink = cloned_sink_arc.lock().await;
                    locked_cloned_sink
                        .as_mut()
                        .unwrap()
                        .send(Message::Bytes(initial_message.try_into().unwrap()))
                        .await
                        .unwrap();
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
                        *self = PlayState::Lobby {
                            web_socket_stream: Arc::clone(web_socket_stream),
                            web_socket_sink: Arc::clone(web_socket_sink),
                            game: Box::new(game.clone()),
                        };
                        true
                    }
                    PlayState::ConnectingError { .. }
                    | PlayState::Lobby { .. }
                    | PlayState::Game { .. }
                    | PlayState::Aftermath { .. } => {
                        log::warn!("Received {server_message:?} and but I am in {self:?}; so doing nothing.");
                        // No-Op
                        false
                    }
                }
            }
            ServerMessage::GameFullUpdate(_) => todo!(),
            ServerMessage::ErrorNewNameEmpty => todo!(),
            ServerMessage::ErrorUnknownInviteCode => todo!(),
        }
    }

    pub fn exit(&mut self, on_closed: Callback<()>) {
        match &self {
            PlayState::Connecting {
                web_socket_stream,
                web_socket_sink,
            }
            | PlayState::Lobby {
                web_socket_stream,
                web_socket_sink,
                ..
            }
            | PlayState::Game {
                web_socket_stream,
                web_socket_sink,
            }
            | PlayState::Aftermath {
                web_socket_stream,
                web_socket_sink,
            } => {
                let cloned_websocket_stream = Arc::clone(web_socket_stream);
                let cloned_web_socket_sink = Arc::clone(web_socket_sink);
                spawn_local(async move {
                    let mut locked_cloned_websocket_stream =
                        TokioMutex::lock(&cloned_websocket_stream).await;
                    let mut locked_cloned_websocket_sink =
                        TokioMutex::lock(&cloned_web_socket_sink).await;
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
            PlayState::Lobby { game, .. } => f
                .debug_struct("PlayState::Lobby")
                .field("game", game)
                .finish(),
            PlayState::Game { .. } => f.debug_struct("PlayState::Game").finish(),
            PlayState::Aftermath { .. } => f.debug_struct("PlayState::Aftermath").finish(),
        }
    }
}
