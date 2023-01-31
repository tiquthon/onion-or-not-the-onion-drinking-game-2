use chrono::Utc;
use std::rc::Rc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    Game, GameState, PlayingState,
};

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::{locale_args, LocaleComponent};
use crate::components::player_name_type_exit_headline::PlayerNameTypeExitHeadlineComponent;

pub struct GameComponent {
    game: Rc<Game>,
    _context_listener: ContextHandle<Rc<Game>>,

    _update_timer: gloo_timers::callback::Interval,
}

impl GameComponent {
    fn view_remaining_time(&self) -> Html {
        let this_player_is_watcher = self.game.get_this_player().unwrap().is_watcher();

        if let GameState::Playing { playing_state, .. } = &self.game.game_state {
            match playing_state {
                PlayingState::Question { time_until, .. } => match time_until {
                    Some(some_time_until) => {
                        let duration = *some_time_until - Utc::now();
                        html! {
                            <section class={classes!("remaining-time-question-state")}>
                                <LocaleComponent
                                    keyid="game-view-question-playing-state-remaining-seconds"
                                    args={locale_args([("seconds", duration.num_seconds().into())])} />
                            </section>
                        }
                    }
                    None => {
                        html! {
                            <section class={classes!("remaining-time-question-state")}>
                                <LocaleComponent keyid="game-view-question-playing-state-infinite-remaining-seconds" />
                            </section>
                        }
                    }
                },
                PlayingState::Solution {
                    time_until,
                    skip_request,
                    ..
                } => {
                    let duration = *time_until - Utc::now();
                    html! {
                        <section class={classes!("remaining-time-aftermath-state")}>
                            <LocaleComponent
                                keyid="game-view-solution-playing-state-remaining-seconds"
                                args={locale_args([("seconds", duration.num_seconds().into())])} />
                            {
                                if this_player_is_watcher {
                                    html! {}
                                } else if skip_request.contains(&self.game.this_player_id) {
                                    html! {
                                        {" - Skipping..."}
                                    }
                                } else {
                                    html! {
                                        <a href="">{"Skip"}</a>
                                    }
                                }
                            }
                        </section>
                    }
                }
            }
        } else {
            unreachable!()
        }
    }
}

impl Component for GameComponent {
    type Message = GameComponentMsg;
    type Properties = GameComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (game, context_listener) = ctx
            .link()
            .context(ctx.link().callback(GameComponentMsg::MessageContextUpdated))
            .expect("Missing Game context.");

        let update_by_interval_callback =
            ctx.link().callback(|_| GameComponentMsg::UpdateByInterval);

        Self {
            game,
            _context_listener: context_listener,

            _update_timer: gloo_timers::callback::Interval::new(500, move || {
                update_by_interval_callback.emit(());
            }),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameComponentMsg::MessageContextUpdated(game) => {
                self.game = game;
                true
            }
            GameComponentMsg::UpdateByInterval => true,
            GameComponentMsg::ExitGame => {
                ctx.props().on_exit_game_wish.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let invite_code = self.game.invite_code.to_string();

        let on_exit_game_wished = ctx.link().callback(|_| GameComponentMsg::ExitGame);

        html! {
            <main class={classes!("play-main")}>
                <JoinGameComponent {invite_code} />
                <section class={classes!("main-wrapper")}>
                    <PlayerNameTypeExitHeadlineComponent {on_exit_game_wished} />
                    { self.view_remaining_time() }
                </section>
            </main>
        }
    }
}

pub enum GameComponentMsg {
    MessageContextUpdated(Rc<Game>),
    UpdateByInterval,

    ExitGame,
}

#[derive(yew::Properties, PartialEq)]
pub struct GameComponentProps {
    pub on_exit_game_wish: Callback<()>,
}
