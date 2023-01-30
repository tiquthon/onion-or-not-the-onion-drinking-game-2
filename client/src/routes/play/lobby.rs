use std::rc::Rc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, PlayType};

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::LocaleComponent;
use crate::components::player_name_type_exit_headline::PlayerNameTypeExitHeadlineComponent;
use crate::components::playerlist::PlayerListComponent;

pub struct LobbyComponent {
    game: Rc<Game>,
    _context_listener: ContextHandle<Rc<Game>>,
}

impl Component for LobbyComponent {
    type Message = LobbyComponentMsg;
    type Properties = LobbyComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (game, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(LobbyComponentMsg::MessageContextUpdated),
            )
            .expect("Missing Game context.");

        Self {
            game,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LobbyComponentMsg::MessageContextUpdated(game) => {
                self.game = game;
                true
            }
            LobbyComponentMsg::ExitGame => {
                ctx.props().on_exit_game_wish.emit(());
                false
            }
            LobbyComponentMsg::StartGame => {
                ctx.props().on_start_game.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let this_player = self
            .game
            .players
            .iter()
            .find(|player| player.id == self.game.this_player_id)
            .unwrap();
        let is_watcher = matches!(this_player.play_type, PlayType::Watcher);

        let invite_code = self.game.invite_code.to_string();

        let on_exit_game_wished = ctx.link().callback(|_| LobbyComponentMsg::ExitGame);

        let onclick_start_game = ctx.link().callback(|_| LobbyComponentMsg::StartGame);

        html! {
            <main class={classes!("play-main")}>
                <JoinGameComponent {invite_code} />
                <section class={classes!("main-wrapper")}>
                    <PlayerNameTypeExitHeadlineComponent {on_exit_game_wished} />
                    <h1 class={classes!("welcome-headline")}>
                        <LocaleComponent keyid="lobby-view-welcome-headline"/>
                    </h1>
                    {
                        if is_watcher {
                            html! {}
                        } else {
                            html! {
                                <div class={classes!("start-form")}>
                                    <button type="button" id="start-form-submit-button" class={classes!("start-form-submit-button")} onclick={onclick_start_game}>
                                        <LocaleComponent keyid="lobby-view-start-game-button"/>
                                    </button>
                                </div>
                            }
                        }
                    }
                    <PlayerListComponent />
                </section>
            </main>
        }
    }
}

pub enum LobbyComponentMsg {
    MessageContextUpdated(Rc<Game>),
    ExitGame,
    StartGame,
}

#[derive(yew::Properties, PartialEq)]
pub struct LobbyComponentProps {
    pub on_exit_game_wish: Callback<()>,
    pub on_start_game: Callback<()>,
}
