use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    Game, GameState, PlayType,
};

use yew::{classes, function_component, html, use_context, Callback, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::{locale_args, LocaleComponent};
use crate::components::player_name_type_exit_headline::PlayerNameTypeExitHeadlineComponent;
use crate::components::playerlist::PlayerListComponent;

#[function_component(AftermathComponent)]
pub fn aftermath_component(props: &AftermathComponentProps) -> Html {
    let game: Rc<Game> = use_context().expect("Missing Game context.");

    let invite_code = game.invite_code.to_string();

    let cloned_on_exit_game_wish = props.on_exit_game_wish.clone();
    let on_exit_game_wished = Callback::from(move |_| cloned_on_exit_game_wish.emit(()));

    let optional_ranking_html: Option<Html> = match &game.game_state {
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
                                1 => "player-ranking__placement--first_place",
                                2 => "player-ranking__placement--second_place",
                                3 => "player-ranking__placement--third_place",
                                _ => "player-ranking__placement--other_place",
                            };
                            html! {
                                    <li class={classes!("player-ranking__placement", ranking_css_class)}>
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
        <main class={classes!("main")}>
            <JoinGameComponent {invite_code} />
            <section class={classes!("centered-primary-content", "play-primary-content")}>
                <PlayerNameTypeExitHeadlineComponent {on_exit_game_wished} />
                <h1>
                    <LocaleComponent keyid="aftermath-view-headline" />
                </h1>
                {
                    match &game.get_this_player().unwrap().play_type {
                        PlayType::Player { .. } => {
                            let has_skipped: bool = match &game.game_state {
                                GameState::Aftermath { restart_requests, .. } => {
                                    restart_requests.contains(&game.get_this_player().unwrap().id)
                                }
                                GameState::InLobby | GameState::Playing { .. } => unreachable!(),
                            };
                            if has_skipped {
                                html! {
                                    <p class={classes!("aftermath-play-again")}>
                                        <LocaleComponent keyid="aftermath-view-next-round-clicked" />
                                    </p>
                                }
                            } else {
                                let cloned_on_play_again_wish = props.on_play_again_wish.clone();
                                let onclick_play_again_button = Callback::from(move |_| cloned_on_play_again_wish.emit(()));
                                html! {
                                    <button type="button" class={classes!("button", "button--width-full")} onclick={onclick_play_again_button}>
                                        <LocaleComponent keyid="aftermath-view-next-round" />
                                    </button>
                                }
                            }
                        },
                        PlayType::Watcher => html! {},
                    }
                }
                <h2>
                    <LocaleComponent keyid="aftermath-view-ranking-headline" />
                </h2>
                {
                    if let Some(ranking_html) = optional_ranking_html {
                        html! {
                            <ol class={classes!("player-ranking")}>
                                {ranking_html}
                            </ol>
                        }
                    } else {
                        html! {
                            <p>
                                <LocaleComponent keyid="aftermath-view-ranking-no-one" />
                            </p>
                        }
                    }
                }
                <PlayerListComponent class={classes!("play-primary-content__player-list")} />
            </section>
        </main>
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct AftermathComponentProps {
    pub on_exit_game_wish: Callback<()>,
    pub on_play_again_wish: Callback<()>,
}
