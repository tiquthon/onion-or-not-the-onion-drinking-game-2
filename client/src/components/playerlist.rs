use std::rc::Rc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    Game, GameState, PlayType, Player, PlayingState,
};

use yew::{classes, html, Component, Context, ContextHandle, Html};

use crate::components::locale::{locale_args, LocaleComponent};

pub struct PlayerListComponent {
    game: Rc<Game>,
    _context_listener: ContextHandle<Rc<Game>>,
}

impl PlayerListComponent {
    fn view_player_list(&self) -> Html {
        if self.game.players.is_empty() {
            html! {
                <p class={classes!("playerlist-listing-or-paragraph")}>
                    <LocaleComponent keyid="play-view-players-no-one-here"/>
                </p>
            }
        } else {
            html! {
                <ul class={classes!("playerlist-listing-or-paragraph")}>
                    {
                        self.game
                            .players
                            .iter()
                            .map(|player: &Player| self.view_player(player))
                            .collect::<Html>()
                    }
                </ul>
            }
        }
    }

    fn view_player(&self, player: &Player) -> Html {
        let is_this_player = player.id == self.game.this_player_id;
        html! {
            <li>
                <span class={classes!(
                    "playerlist-username",
                    is_this_player.then_some("playerlist-actual-user-is-username")
                )}>
                    {player.name.to_string()}
                </span>
                { self.view_player_state(player) }
                <span class={classes!("playerlist-points-or-watching")}>
                    {
                        match &player.play_type {
                            PlayType::Player { points } => {
                                html! {
                                    <LocaleComponent
                                        keyid="play-view-players-points"
                                        args={locale_args([("points", points.into())])}/>
                                }
                            }
                            PlayType::Watcher => {
                                html! {
                                    <>
                                        {" ("}
                                        <LocaleComponent keyid="play-view-players-is-watching"/>
                                        {")"}
                                    </>
                                }
                            }
                        }
                    }
                </span>
            </li>
        }
    }

    fn view_player_state(&self, player: &Player) -> Html {
        match &self.game.game_state {
            GameState::InLobby => html! {},
            GameState::Playing {
                playing_state: PlayingState::Question { answers, .. },
                ..
            } => {
                let user_has_answered: bool = answers.contains(&player.id);
                if user_has_answered {
                    html! {
                        <span class={classes!("playerlist-user-has-answered")}>
                            <img src="pencil_icon.png"/>
                        </span>
                    }
                } else {
                    html! {}
                }
            }
            GameState::Playing {
                playing_state:
                    PlayingState::Solution {
                        correct_answer,
                        answers,
                        skip_request,
                        ..
                    },
                ..
            } => {
                let user_has_answered: bool = answers.contains_key(&player.id);
                let user_has_correct_answer: Option<bool> = answers
                    .get(&player.id)
                    .map(|player_answer| player_answer == correct_answer);
                let user_wants_skip: bool = skip_request.contains(&player.id);
                html! {
                    <>
                        {
                            if user_wants_skip {
                                html! {
                                    <span class={classes!("playerlist-user-wants-to-skip")}>
                                        <img src="fastforward.png"/>
                                    </span>
                                }
                            } else {
                                html! {}
                            }
                        }
                        {
                            if user_has_answered {
                                html! {
                                    <span class={classes!("playerlist-user-has-answered")}>
                                        <img src="pencil_icon.png"/>
                                    </span>
                                }
                            } else {
                                html! {}
                            }
                        }
                        {
                            match user_has_correct_answer {
                                Some(true) => {
                                    html! {
                                        <span class={classes!("playerlist-users-answer")}>
                                            <img src="correct.png"/>
                                        </span>
                                    }
                                }
                                Some(false) => {
                                    html! {
                                        <span class={classes!("playerlist-users-answer")}>
                                            <img src="incorrect.png"/>
                                        </span>
                                    }
                                }
                                None => {
                                    html! {}
                                }
                            }
                        }
                    </>
                }
            }
            GameState::Aftermath { .. } => {
                html! {}
            }
        }
    }
}

impl Component for PlayerListComponent {
    type Message = PlayerListComponentMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (game, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(PlayerListComponentMsg::MessageContextUpdated),
            )
            .expect("Missing Game context.");

        Self {
            game,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlayerListComponentMsg::MessageContextUpdated(game) => {
                self.game = game;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h3 class={classes!("playerlist-headline")}>
                    <LocaleComponent keyid="play-view-players-headline"/>
                </h3>
                { self.view_player_list() }
            </>
        }
    }
}

pub enum PlayerListComponentMsg {
    MessageContextUpdated(Rc<Game>),
}
