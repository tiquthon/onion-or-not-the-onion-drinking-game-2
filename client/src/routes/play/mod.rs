use fluent_templates::LanguageIdentifier;

use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::ServerMessage;

use yew::{html, Callback, Component, Context, ContextHandle, Html};

use aftermath::AftermathComponent;
use connecting::ConnectingComponent;
use game::GameComponent;
use lobby::LobbyComponent;
use play_state::PlayState;

use crate::routes::index::{CreateLobby, JoinLobby};

pub mod aftermath;
pub mod connecting;
pub mod game;
pub mod lobby;
pub mod play_state;

pub struct PlayComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,

    state: PlayState,
}

impl Component for PlayComponent {
    type Message = PlayComponentMsg;
    type Properties = PlayComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(ctx.link().callback(PlayComponentMsg::MessageContextUpdated))
            .expect("Missing LanguageIdentifier context.");

        Self {
            langid,
            _context_listener: context_listener,

            state: PlayState::connect(
                option_env!("BUILD_WEBSOCKET_ADDRESS").unwrap_or("ws://localhost:8000/api/ws"),
                ctx.link()
                    .callback(PlayComponentMsg::WebSocketMessageReceived),
                ctx.link().callback(|_| PlayComponentMsg::WebSocketClosed),
                &ctx.props().create_join_lobby,
            ),
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
                    self.state.handle_server_message(server_message)
                }
                Ok(Message::Text(_)) | Err(_) => panic!(),
            },
            PlayComponentMsg::ExitGame => {
                self.state.exit(ctx.props().on_exit_game.clone());
                false
            }
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
