use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;

use fluent_templates::LanguageIdentifier;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};

use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::Game;
use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::{
    ClientMessage, ServerMessage,
};

use wasm_bindgen_futures::spawn_local;

use yew::{html, Callback, Component, Context, ContextHandle, Html};

use aftermath::AftermathComponent;
use connecting::ConnectingComponent;
use game::GameComponent;
use lobby::LobbyComponent;

use crate::routes::index::{CreateLobby, JoinLobby};

pub mod aftermath;
pub mod connecting;
pub mod game;
pub mod lobby;

pub struct PlayComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,

    state: PlayState,
}

impl PlayComponent {
    fn handle_server_message(&mut self, server_message: ServerMessage) -> bool {
        match &server_message {
            ServerMessage::LobbyCreated(game) | ServerMessage::LobbyJoined(game) => {
                match &self.state {
                    PlayState::Connecting {
                        web_socket_stream,
                        web_socket_sink,
                    } => {
                        self.state = PlayState::Lobby {
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
                        log::warn!(
                            "Received {server_message:?} and but I am in {:?}; so doing nothing.",
                            self.state
                        );
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
}

impl Component for PlayComponent {
    type Message = PlayComponentMsg;
    type Properties = PlayComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(ctx.link().callback(PlayComponentMsg::MessageContextUpdated))
            .expect("Missing LanguageIdentifier context.");

        let web_socket_address =
            option_env!("BUILD_WEBSOCKET_ADDRESS").unwrap_or("ws://localhost:8000/api/ws");
        let play_state = match WebSocket::open(web_socket_address) {
            Ok(web_socket) => {
                let (sink, stream) = web_socket.split();

                let stream_arc = Arc::new(tokio::sync::Mutex::new(Some(stream)));
                let cloned_stream_arc = Arc::clone(&stream_arc);

                let sink_arc = Arc::new(tokio::sync::Mutex::new(Some(sink)));
                let cloned_sink_arc = Arc::clone(&sink_arc);

                let web_socket_message_received_callback = ctx
                    .link()
                    .callback(PlayComponentMsg::WebSocketMessageReceived);
                let web_socket_closed_callback =
                    ctx.link().callback(|_| PlayComponentMsg::WebSocketClosed);

                spawn_local(async move {
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
                                    Poll::Ready(Some(msg)) => {
                                        web_socket_message_received_callback.emit(msg)
                                    }
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

                    web_socket_closed_callback.emit(());
                });

                let initial_message = match &ctx.props().create_join_lobby {
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
        };

        Self {
            langid,
            _context_listener: context_listener,

            state: play_state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlayComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
            PlayComponentMsg::WebSocketClosed => {
                log::info!("Web Socket Closed");
                true
            }
            PlayComponentMsg::WebSocketMessageReceived(msg) => match msg {
                Ok(Message::Bytes(bytes)) => {
                    let server_message = ServerMessage::try_from(&bytes[..]).unwrap();
                    self.handle_server_message(server_message)
                }
                Ok(Message::Text(_)) | Err(_) => panic!(),
            },
            PlayComponentMsg::ExitGame => match &self.state {
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
                    let close_callback = ctx.props().on_exit_game.clone();
                    spawn_local(async move {
                        let mut locked_cloned_websocket_stream =
                            cloned_websocket_stream.lock().await;
                        let mut locked_cloned_websocket_sink = cloned_web_socket_sink.lock().await;
                        let optional_websocket_stream = locked_cloned_websocket_stream.take();
                        let optional_websocket_sink = locked_cloned_websocket_sink.take();
                        if let (Some(stream), Some(sink)) =
                            (optional_websocket_stream, optional_websocket_sink)
                        {
                            let websocket = stream.reunite(sink).unwrap();
                            websocket.close(Some(1000), Some("by client")).unwrap();
                        }
                        close_callback.emit(());
                    });
                    false
                }
                PlayState::ConnectingError { .. } => false,
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_exit_game_wish = ctx.link().callback(|_| PlayComponentMsg::ExitGame);

        match &self.state {
            PlayState::ConnectingError { .. } => {
                html! { <ConnectingComponent /> }
            }
            PlayState::Connecting { .. } => {
                html! { <ConnectingComponent /> }
            }
            PlayState::Lobby { game, .. } => {
                html! { <LobbyComponent game={AsRef::as_ref(game).clone()} {on_exit_game_wish} /> }
            }
            PlayState::Game { .. } => {
                html! { <GameComponent /> }
            }
            PlayState::Aftermath { .. } => {
                html! { <AftermathComponent /> }
            }
        }
    }
}

pub enum PlayComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
    WebSocketClosed,
    WebSocketMessageReceived(Result<Message, WebSocketError>),

    ExitGame,
}

#[derive(yew::Properties, PartialEq)]
pub struct PlayComponentProps {
    pub create_join_lobby: CreateJoinLobby,
    pub on_exit_game: Callback<()>,
}

#[derive(PartialEq, Clone)]
pub enum CreateJoinLobby {
    Create(CreateLobby),
    Join(JoinLobby),
}

enum PlayState {
    ConnectingError {
        error: anyhow::Error,
    },
    Connecting {
        web_socket_stream: Arc<tokio::sync::Mutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<tokio::sync::Mutex<Option<SplitSink<WebSocket, Message>>>>,
    },
    // TODO
    #[allow(dead_code)]
    Lobby {
        web_socket_stream: Arc<tokio::sync::Mutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<tokio::sync::Mutex<Option<SplitSink<WebSocket, Message>>>>,
        game: Box<Game>,
    },
    // TODO
    #[allow(dead_code)]
    Game {
        web_socket_stream: Arc<tokio::sync::Mutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<tokio::sync::Mutex<Option<SplitSink<WebSocket, Message>>>>,
    },
    // TODO
    #[allow(dead_code)]
    Aftermath {
        web_socket_stream: Arc<tokio::sync::Mutex<Option<SplitStream<WebSocket>>>>,
        web_socket_sink: Arc<tokio::sync::Mutex<Option<SplitSink<WebSocket, Message>>>>,
    },
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
