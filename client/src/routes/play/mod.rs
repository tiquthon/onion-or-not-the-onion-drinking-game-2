// TODO: use std::cell::RefCell;
// TODO: use std::rc::Rc;

use fluent_templates::LanguageIdentifier;

// TODO: use futures_util::stream::SplitSink;
// TODO: use futures_util::StreamExt;

// TODO: use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};

// TODO: use wasm_bindgen_futures::spawn_local;

use yew::{html, Component, Context, ContextHandle, Html};

// TODO: use aftermath::AftermathComponent;
use connecting::ConnectingComponent;
// TODO: use game::GameComponent;
// TODO: use lobby::LobbyComponent;

pub mod aftermath;
pub mod connecting;
pub mod game;
pub mod lobby;

pub struct PlayComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,

    // TODO: messages: Vec<crate::components::messages::Message>,
    state: PlayState,
}

impl Component for PlayComponent {
    type Message = PlayComponentMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // TODO: use anyhow::Context;

        let (langid, context_listener) = ctx
            .link()
            .context(ctx.link().callback(PlayComponentMsg::MessageContextUpdated))
            .expect("Missing LanguageIdentifier context.");

        /* TODO: let web_socket_sink = WebSocket::open("ws://localhost:8080/ws")
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
            Rc::new(RefCell::new(sink))
        })
        .context("Failed opening WebSocket connection.");*/

        Self {
            langid,
            _context_listener: context_listener,

            // TODO: messages: Vec::new(),
            state: PlayState::Connecting {
                // TODO: web_socket_sink
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlayComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
            _ => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.state {
            PlayState::Connecting { .. } => {
                html! { <ConnectingComponent /> }
            } /* TODO: PlayState::Lobby => {
                  html! { <LobbyComponent /> }
              }
              PlayState::Game => {
                  html! { <GameComponent /> }
              }
              PlayState::Aftermath => {
                  html! { <AftermathComponent /> }
              } */
        }
    }
}

pub enum PlayComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
    WebSocketClosed,
    WebSocketMessageReceived(Result<Message, WebSocketError>),
}

enum PlayState {
    Connecting {
        // TODO: web_socket_sink: anyhow::Result<Rc<RefCell<SplitSink<WebSocket, Message>>>>,
    },
    // TODO: Lobby,
    // TODO: Game,
    // TODO: Aftermath,
}
