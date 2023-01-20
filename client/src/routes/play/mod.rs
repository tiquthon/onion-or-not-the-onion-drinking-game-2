use std::cell::RefCell;
use std::rc::Rc;

use fluent_templates::LanguageIdentifier;

use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};

use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::ClientMessage;

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
                        count_of_questions,
                        minimum_score_of_questions,
                        timer,
                    }) => ClientMessage::CreateLobby {
                        player_name: player_name.clone(),
                        // TODO: just_watch
                        just_watch: false,
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
            PlayComponentMsg::WebSocketMessageReceived(msg) => {
                log::info!("Web Socket Message Received {msg:?}");
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.state {
            PlayState::Connecting { .. } => {
                html! { <ConnectingComponent /> }
            }
            PlayState::Lobby => {
                html! { <LobbyComponent /> }
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
    Lobby,
    // TODO
    #[allow(dead_code)]
    Game,
    // TODO
    #[allow(dead_code)]
    Aftermath,
}

#[allow(clippy::await_holding_refcell_ref)]
async fn send(sink_cloned: &Rc<RefCell<SplitSink<WebSocket, Message>>>, message: ClientMessage) {
    RefCell::borrow_mut(sink_cloned)
        .send(Message::Bytes(message.try_into().unwrap()))
        .await
        .unwrap();
}
