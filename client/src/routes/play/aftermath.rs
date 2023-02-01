use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    Game, GameState, PlayType,
};

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::{locale_args, LocaleComponent};
use crate::components::player_name_type_exit_headline::PlayerNameTypeExitHeadlineComponent;
use crate::components::playerlist::PlayerListComponent;

pub struct AftermathComponent {
    game: Rc<Game>,
    _context_listener: ContextHandle<Rc<Game>>,
}

impl Component for AftermathComponent {
    type Message = AftermathComponentMsg;
    type Properties = AftermathComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (game, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(AftermathComponentMsg::MessageContextUpdated),
            )
            .expect("Missing Game context.");

        Self {
            game,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AftermathComponentMsg::MessageContextUpdated(game) => {
                self.game = game;
                true
            }
            AftermathComponentMsg::ExitGame => {
                ctx.props().on_exit_game_wish.emit(());
                false
            }
            AftermathComponentMsg::PlayAgain => {
                ctx.props().on_play_again_wish.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let invite_code = self.game.invite_code.to_string();
        let on_exit_game_wished = ctx.link().callback(|_| AftermathComponentMsg::ExitGame);

        let optional_ranking_html: Option<Html> = match &self.game.game_state {
            GameState::Aftermath { ranked_players, .. } => {
                let ranked_players = ranked_players.clone();
                if ranked_players.is_empty() {
                    None
                } else {
                    let possible_points: HashMap<u16, usize> = ranked_players
                        .iter()
                        .map(|(_, _, c)| *c)
                        .unique()
                        .sorted()
                        .rev()
                        .enumerate()
                        .map(|(index, points)| (points, index))
                        .collect();
                    Some(
                        ranked_players
                            .into_iter()
                            .sorted_by_key(|(_, _, points)| *points)
                            .rev()
                            .map(|(_, player_name, points)| {
                                let rank = *possible_points.get(&points).unwrap() + 1;
                                let ranking_css_class = match rank {
                                    1 => "list-or-paragraph-ranking-place-one",
                                    2 => "list-or-paragraph-ranking-place-two",
                                    3 => "list-or-paragraph-ranking-place-three",
                                    _ => "list-or-paragraph-ranking-place-other",
                                };
                                html! {
                                    <li class={classes!(ranking_css_class)}>
                                        {format!("{rank}. {player_name} - ")}
                                        <LocaleComponent
                                            keyid="aftermath-view-ranking-players-points"
                                            args={locale_args([("points", points.into())])} />
                                    </li>
                                }
                            })
                            .collect::<Html>(),
                    )
                }
            }
            GameState::InLobby | GameState::Playing { .. } => unreachable!(),
        };

        html! {
            <main class={classes!("play-main")}>
                <JoinGameComponent {invite_code} />
                <section class={classes!("main-wrapper")}>
                    <PlayerNameTypeExitHeadlineComponent {on_exit_game_wished} />
                    <h1 class={classes!("question-headline-at-end-state")}>
                        <LocaleComponent keyid="aftermath-view-headline" />
                    </h1>
                    {
                        match &self.game.get_this_player().unwrap().play_type {
                            PlayType::Player { .. } => {
                                let has_skipped: bool = match &self.game.game_state {
                                    GameState::Aftermath { restart_requests, .. } => {
                                        restart_requests.contains(&self.game.get_this_player().unwrap().id)
                                    }
                                    GameState::InLobby | GameState::Playing { .. } => unreachable!(),
                                };
                                if has_skipped {
                                    html! {
                                        <p class={classes!("play-again-submit-button")}>
                                            <LocaleComponent keyid="aftermath-view-next-round-clicked" />
                                        </p>
                                    }
                                } else {
                                    let onclick_play_again_button = ctx.link().callback(|_| AftermathComponentMsg::PlayAgain);
                                    html! {
                                        <button type="button" class={classes!("play-again-submit-button")} onclick={onclick_play_again_button}>
                                            <LocaleComponent keyid="aftermath-view-next-round" />
                                        </button>
                                    }
                                }
                            },
                            PlayType::Watcher => html! {},
                        }
                    }
                    <h2 class={classes!("ranking-headline-at-end-state")}>
                        <LocaleComponent keyid="aftermath-view-ranking-headline" />
                    </h2>
                    {
                        if let Some(ranking_html) = optional_ranking_html {
                            html! {
                                <ol class={classes!("list-or-paragraph-ranking")}>
                                    {ranking_html}
                                </ol>
                            }
                        } else {
                            html! {
                                <p class={classes!("list-or-paragraph-ranking")}>
                                    <LocaleComponent keyid="aftermath-view-ranking-no-one" />
                                </p>
                            }
                        }
                    }
                    <PlayerListComponent />
                </section>
            </main>
        }
    }
}

pub enum AftermathComponentMsg {
    MessageContextUpdated(Rc<Game>),

    ExitGame,
    PlayAgain,
}

#[derive(yew::Properties, PartialEq)]
pub struct AftermathComponentProps {
    pub on_exit_game_wish: Callback<()>,
    pub on_play_again_wish: Callback<()>,
}
