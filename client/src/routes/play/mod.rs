use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use fluent_templates::LanguageIdentifier;

use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};

use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::Game;
use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::{
    ClientMessage, ServerMessage,
};

use wasm_bindgen_futures::spawn_local;

use yew::{html, Component, Context, ContextHandle, Html};

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
                    PlayState::Connecting { web_socket_sink } => {
                        match web_socket_sink {
                            Ok(web_socket_sink) => {
                                self.state = PlayState::Lobby {
                                    web_socket_sink: web_socket_sink.clone(),
                                    game: Box::new(game.clone()),
                                };
                                true
                            }
                            Err(_) => {
                                log::warn!(
                                    "Received {server_message:?} in {:?}, but web_socket_sink is Err.",
                                    self.state
                                );
                                // No-Op
                                false
                            }
                        }
                    }
                    PlayState::Lobby { .. } | PlayState::Game | PlayState::Aftermath => {
                        log::warn!(
                            "Received {server_message:?} and but I am in {:?}; doing nothing.",
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
        use anyhow::Context;

        let (langid, context_listener) = ctx
            .link()
            .context(ctx.link().callback(PlayComponentMsg::MessageContextUpdated))
            .expect("Missing LanguageIdentifier context.");

        let web_socket_address =
            option_env!("BUILD_WEBSOCKET_ADDRESS").unwrap_or("ws://localhost:8000/api/ws");
        let web_socket_sink = WebSocket::open(web_socket_address)
            .map(|web_socket: WebSocket| {
                let (sink, mut stream) = web_socket.split();

                let web_socket_message_received_callback = ctx
                    .link()
                    .callback(PlayComponentMsg::WebSocketMessageReceived);
                let web_socket_closed_callback =
                    ctx.link().callback(|_| PlayComponentMsg::WebSocketClosed);

                spawn_local(async move {
                    while let Some(msg) = stream.next().await {
                        web_socket_message_received_callback.emit(msg);
                    }
                    web_socket_closed_callback.emit(());
                });

                let sink_rc = Rc::new(RefCell::new(sink));

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

                let sink_cloned = Rc::clone(&sink_rc);
                spawn_local(async move { send(&sink_cloned, initial_message).await });

                sink_rc
            })
            .with_context(|| {
                format!("Failed opening WebSocket to {web_socket_address} connection.")
            });

        Self {
            langid,
            _context_listener: context_listener,

            state: PlayState::Connecting { web_socket_sink },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.state {
            PlayState::Connecting { .. } => {
                html! { <ConnectingComponent /> }
            }
            PlayState::Lobby { game, .. } => {
                html! { <LobbyComponent game={AsRef::as_ref(game).clone()} /> }
            }
            PlayState::Game => {
                html! { <GameComponent /> }
            }
            PlayState::Aftermath => {
                html! { <AftermathComponent /> }
            }
        }
    }
}

pub enum PlayComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
    WebSocketClosed,
    WebSocketMessageReceived(Result<Message, WebSocketError>),
}

#[derive(yew::Properties, PartialEq)]
pub struct PlayComponentProps {
    pub create_join_lobby: CreateJoinLobby,
}

#[derive(PartialEq, Clone)]
pub enum CreateJoinLobby {
    Create(CreateLobby),
    Join(JoinLobby),
}

enum PlayState {
    Connecting {
        // TODO
        #[allow(dead_code)]
        web_socket_sink: anyhow::Result<Rc<RefCell<SplitSink<WebSocket, Message>>>>,
    },
    // TODO
    #[allow(dead_code)]
    Lobby {
        web_socket_sink: Rc<RefCell<SplitSink<WebSocket, Message>>>,
        game: Box<Game>,
    },
    // TODO
    #[allow(dead_code)]
    Game,
    // TODO
    #[allow(dead_code)]
    Aftermath,
}

impl Debug for PlayState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayState::Connecting { .. } => f.debug_struct("PlayState::Connecting").finish(),
            PlayState::Lobby { game, .. } => f
                .debug_struct("PlayState::Lobby")
                .field("game", game)
                .finish(),
            PlayState::Game => f.debug_struct("PlayState::Game").finish(),
            PlayState::Aftermath => f.debug_struct("PlayState::Aftermath").finish(),
        }
    }
}

#[allow(clippy::await_holding_refcell_ref)]
async fn send(sink_cloned: &Rc<RefCell<SplitSink<WebSocket, Message>>>, message: ClientMessage) {
    RefCell::borrow_mut(sink_cloned)
        .send(Message::Bytes(message.try_into().unwrap()))
        .await
        .unwrap();
}
