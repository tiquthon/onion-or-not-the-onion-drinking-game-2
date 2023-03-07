use std::cell::RefCell;
use std::rc::Rc;

use fluent_templates::LanguageIdentifier;

use gloo_net::websocket::{Message, WebSocketError};

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, GameState};
use onion_or_not_the_onion_drinking_game_2_shared_library::model::network::ServerMessage;

use yew::{
    function_component, html, use_context, use_effect_with_deps, use_force_update, use_state,
    Callback, ContextProvider, Html,
};

use crate::routes::index::{CreateLobby, JoinLobby};
use crate::routes::play::aftermath::AftermathComponent;
use crate::routes::play::connecting::{ConnectingComponent, ConnectingComponentState};
use crate::routes::play::game::GameComponent;
use crate::routes::play::lobby::LobbyComponent;
use crate::routes::play::play_state::{ConnectingErrorLocaleKeyId, PlayState};
use crate::utils::{retrieve_browser_location, REPLACE_PROTOCOL_WEBSOCKET};

pub mod aftermath;
pub mod connecting;
pub mod game;
pub mod lobby;
pub mod play_state;

#[function_component(PlayComponent)]
pub fn play_component(props: &PlayComponentProps) -> Html {
    let _langid = use_context::<LanguageIdentifier>().expect("Missing LanguageIdentifier context.");

    let force_update = use_force_update();

    let play_state = Rc::clone(&*use_state(|| Rc::new(RefCell::new(PlayState::None))));

    let cloned_create_join_lobby = props.create_join_lobby.clone();
    let cloned_play_state = Rc::clone(&play_state);
    use_effect_with_deps(
        move |_| {
            let web_socket_address_root =
                retrieve_browser_location(Some(REPLACE_PROTOCOL_WEBSOCKET), Some("/api"));
            log::debug!("Retrieved web_socket_address_root as {web_socket_address_root}");

            let cloned_force_update = force_update.clone();
            let cloned_cloned_play_state = Rc::clone(&cloned_play_state);
            let on_message_received = Callback::from(move |msg| match msg {
                Ok(Message::Bytes(bytes)) => {
                    let server_message = ServerMessage::try_from(&bytes[..]).unwrap();
                    let optional_new_play_state = {
                        RefCell::borrow(&cloned_cloned_play_state)
                            .handle_server_message(server_message)
                    };
                    if let Some(new_play_state) = optional_new_play_state {
                        *RefCell::borrow_mut(&cloned_cloned_play_state) = new_play_state;
                        cloned_force_update.force_update();
                    }
                }
                Ok(Message::Text(text)) => {
                    log::warn!("Received text from WebSocket \"{text}\"; ignoring it...");
                }
                Err(error) => {
                    log::warn!("An error occurred: {error} ({error:?})");
                    let new_play_state = PlayState::ConnectingError {
                        locale_keyid: match &error {
                            WebSocketError::ConnectionError => {
                                ConnectingErrorLocaleKeyId::MessageReceiveConnectionError
                            }
                            WebSocketError::ConnectionClose(_) => {
                                ConnectingErrorLocaleKeyId::MessageReceiveConnectionClose
                            }
                            WebSocketError::MessageSendError(_) => {
                                ConnectingErrorLocaleKeyId::MessageReceiveMessageSendError
                            }
                            _ => unimplemented!(),
                        },
                        error: Some(error.into()),
                    };
                    *RefCell::borrow_mut(&cloned_cloned_play_state) = new_play_state;
                    cloned_force_update.force_update();
                }
            });

            let cloned_force_update = force_update.clone();
            let on_connection_closed = Callback::from(move |_| cloned_force_update.force_update());

            *RefCell::borrow_mut(&cloned_play_state) = PlayState::connect(
                &web_socket_address_root,
                on_message_received,
                on_connection_closed,
                &cloned_create_join_lobby,
            );
            force_update.force_update();
        },
        (),
    );

    let borrowed_play_state = RefCell::borrow(&play_state);
    match &*borrowed_play_state {
        PlayState::Connecting { .. } => {
            let cloned_play_state = Rc::clone(&play_state);
            let cloned_on_go_back_to_index = props.on_go_back_to_index.clone();
            let on_cancel = Callback::from(move |_| {
                RefCell::borrow(&cloned_play_state).exit(cloned_on_go_back_to_index.clone())
            });

            html! { <ConnectingComponent state={ConnectingComponentState::Connecting} {on_cancel} /> }
        }
        PlayState::Playing { game, .. } => {
            let cloned_play_state = Rc::clone(&play_state);
            let cloned_on_go_back_to_index = props.on_go_back_to_index.clone();
            let on_exit_game_wish = Callback::from(move |_| {
                RefCell::borrow(&cloned_play_state).exit(cloned_on_go_back_to_index.clone())
            });

            match game.game_state {
                GameState::InLobby => {
                    let cloned_play_state = Rc::clone(&play_state);
                    let on_start_game = Callback::from(move |_| {
                        RefCell::borrow(&cloned_play_state).wish_for_game_start()
                    });

                    let game_rc = Rc::new(AsRef::as_ref(game).clone());
                    html! {
                        <ContextProvider<Rc<Game>> context={game_rc}>
                            <LobbyComponent {on_exit_game_wish} {on_start_game} />
                        </ContextProvider<Rc<Game>>>
                    }
                }
                GameState::Playing { .. } => {
                    let game_rc = Rc::new(AsRef::as_ref(game).clone());

                    let cloned_play_state = Rc::clone(&play_state);
                    let on_choose_answer = Callback::from(move |answer| {
                        RefCell::borrow(&cloned_play_state).choose_answer(answer)
                    });

                    let cloned_play_state = Rc::clone(&play_state);
                    let on_request_skip =
                        Callback::from(move |_| RefCell::borrow(&cloned_play_state).request_skip());

                    html! {
                        <ContextProvider<Rc<Game>> context={game_rc}>
                            <GameComponent {on_exit_game_wish} {on_choose_answer} {on_request_skip} />
                        </ContextProvider<Rc<Game>>>
                    }
                }
                GameState::Aftermath { .. } => {
                    let game_rc = Rc::new(AsRef::as_ref(game).clone());

                    let cloned_play_state = Rc::clone(&play_state);
                    let on_play_again_wish = Callback::from(move |_| {
                        RefCell::borrow(&cloned_play_state).request_play_again()
                    });

                    html! {
                        <ContextProvider<Rc<Game>> context={game_rc}>
                            <AftermathComponent {on_exit_game_wish} {on_play_again_wish}/>
                        </ContextProvider<Rc<Game>>>
                    }
                }
            }
        }
        PlayState::ConnectingError {
            locale_keyid,
            error,
        } => {
            let cloned_on_go_back_to_index = props.on_go_back_to_index.clone();
            let on_go_back = Callback::from(move |_| cloned_on_go_back_to_index.emit(()));

            let state = ConnectingComponentState::Failed {
                locale_key_id: locale_keyid.key_id().to_string(),
                error: error.as_ref().map(ToString::to_string),
            };

            html! { <ConnectingComponent {state} {on_go_back} /> }
        }
        PlayState::None => Default::default(),
    }
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
