use std::rc::Rc;

use fluent_templates::LanguageIdentifier;

use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Answer, Game, GameState};
use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::ServerMessage;

use yew::{html, Callback, Component, Context, ContextHandle, ContextProvider, Html};

use aftermath::AftermathComponent;
use connecting::ConnectingComponent;
use game::GameComponent;
use lobby::LobbyComponent;
use play_state::PlayState;

use crate::routes::index::{CreateLobby, JoinLobby};
use crate::routes::play::connecting::ConnectingComponentState;

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
                option_env!("BUILD_WEBSOCKET_ADDRESS").unwrap_or("ws://localhost:8000/api"),
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
                Ok(Message::Text(text)) => {
                    log::warn!("Received text from WebSocket \"{text}\"; ignoring it...");
                    false
                }
                Err(error) => {
                    log::warn!("An error occurred: {error} ({error:?})");
                    self.state = PlayState::ConnectingError {
                        error: error.into(),
                    };
                    true
                }
            },
            PlayComponentMsg::GoBackToIndex => {
                ctx.props().on_go_back_to_index.emit(());
                false
            }
            PlayComponentMsg::ExitGame => {
                self.state.exit(ctx.props().on_go_back_to_index.clone());
                false
            }
            PlayComponentMsg::StartGame => {
                self.state.wish_for_game_start();
                false
            }
            PlayComponentMsg::ChooseAnswer(answer) => {
                self.state.choose_answer(answer);
                false
            }
            PlayComponentMsg::RequestSkip => {
                self.state.request_skip();
                false
            }
            PlayComponentMsg::RequestPlayAgain => {
                self.state.request_play_again();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            PlayState::ConnectingError { error } => {
                let on_go_back = ctx.link().callback(|_| PlayComponentMsg::GoBackToIndex);

                html! { <ConnectingComponent state={ConnectingComponentState::Failed { error: error.to_string() }} {on_go_back} /> }
            }
            PlayState::Connecting { .. } => {
                let on_cancel = ctx.link().callback(|_| PlayComponentMsg::ExitGame);

                html! { <ConnectingComponent state={ConnectingComponentState::Connecting} {on_cancel} /> }
            }
            PlayState::Playing { game, .. } => {
                let on_exit_game_wish = ctx.link().callback(|_| PlayComponentMsg::ExitGame);

                match game.game_state {
                    GameState::InLobby => {
                        let on_start_game = ctx.link().callback(|_| PlayComponentMsg::StartGame);

                        let game_rc = Rc::new(AsRef::as_ref(game).clone());
                        html! {
                            <ContextProvider<Rc<Game>> context={game_rc}>
                                <LobbyComponent {on_exit_game_wish} {on_start_game} />
                            </ContextProvider<Rc<Game>>>
                        }
                    }
                    GameState::Playing { .. } => {
                        let game_rc = Rc::new(AsRef::as_ref(game).clone());
                        let on_choose_answer = ctx.link().callback(PlayComponentMsg::ChooseAnswer);
                        let on_request_skip =
                            ctx.link().callback(|_| PlayComponentMsg::RequestSkip);
                        html! {
                            <ContextProvider<Rc<Game>> context={game_rc}>
                                <GameComponent {on_exit_game_wish} {on_choose_answer} {on_request_skip} />
                            </ContextProvider<Rc<Game>>>
                        }
                    }
                    GameState::Aftermath { .. } => {
                        let game_rc = Rc::new(AsRef::as_ref(game).clone());
                        let on_play_again_wish =
                            ctx.link().callback(|_| PlayComponentMsg::RequestPlayAgain);
                        html! {
                            <ContextProvider<Rc<Game>> context={game_rc}>
                                <AftermathComponent {on_exit_game_wish} {on_play_again_wish}/>
                            </ContextProvider<Rc<Game>>>
                        }
                    }
                }
            }
        }
    }
}

pub enum PlayComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
    WebSocketClosed,
    WebSocketMessageReceived(Result<Message, WebSocketError>),

    GoBackToIndex,
    ExitGame,
    StartGame,

    ChooseAnswer(Answer),
    RequestSkip,
    RequestPlayAgain,
}

#[derive(yew::Properties, PartialEq)]
pub struct PlayComponentProps {
    pub create_join_lobby: CreateJoinLobby,
    pub on_go_back_to_index: Callback<()>,
}

#[derive(PartialEq, Clone)]
pub enum CreateJoinLobby {
    Create(CreateLobby),
    Join(JoinLobby),
}
