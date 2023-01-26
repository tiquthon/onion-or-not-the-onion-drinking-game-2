use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    GameState, PlayType, Player, PlayerId, PlayingState,
};

use yew::{classes, html, Component, Context, Html};

use crate::components::locale::{locale_args, LocaleComponent};

pub struct PlayerListComponent;

impl PlayerListComponent {
    fn view_player_list(ctx: &Context<Self>) -> Html {
        if ctx.props().players.is_empty() {
            html! {
                <p class={classes!("playerlist-listing-or-paragraph")}>
                    <LocaleComponent keyid="play-view-players-no-one-here"/>
                </p>
            }
        } else {
            html! {
                <ul class={classes!("playerlist-listing-or-paragraph")}>
                    {
                        ctx.props()
                            .players
                            .iter()
                            .map(|player: &Player| Self::view_player(player, ctx))
                            .collect::<Html>()
                    }
                </ul>
            }
        }
    }

    fn view_player(player: &Player, ctx: &Context<Self>) -> Html {
        let is_this_player = player.id == ctx.props().this_player_id;
        html! {
            <li>
                <span class={classes!(
                    "playerlist-username",
                    is_this_player.then_some("playerlist-actual-user-is-username")
                )}>
                    {player.name.to_string()}
                </span>
                { Self::view_player_state(player, ctx) }
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

    fn view_player_state(player: &Player, ctx: &Context<Self>) -> Html {
        match &ctx.props().game_state {
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
    type Message = ();
    type Properties = PlayerListComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h3 class={classes!("playerlist-headline")}>
                    <LocaleComponent keyid="play-view-players-headline"/>
                </h3>
                { Self::view_player_list(ctx) }
            </>
        }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct PlayerListComponentProps {
    pub players: Vec<Player>,
    pub this_player_id: PlayerId,
    pub game_state: GameState,
}
