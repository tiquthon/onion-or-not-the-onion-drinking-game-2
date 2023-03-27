use std::rc::Rc;

use fluent_templates::LanguageIdentifier;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    Game, GameState, PlayType, Player, PlayingState,
};

use yew::{classes, function_component, html, use_context, Classes, Html};

use crate::components::locale::{locale_args, LocaleComponent};
use crate::components::svg::{CORRECT_SVG, FAST_FORWARD_SVG, INCORRECT_SVG, PENCIL_SVG};

#[function_component(PlayerListComponent)]
pub fn player_list_component(props: &PlayerListProps) -> Html {
    let _langid: LanguageIdentifier = use_context().expect("Missing LanguageIdentifier context.");
    let game: Rc<Game> = use_context().expect("Missing Game context.");

    html! {
        <aside class={classes!("player-list-container", props.class.clone())}>
            <h3 class={classes!("player-list-container__headline")}>
                <LocaleComponent keyid="play-view-players-headline"/>
            </h3>
            { view_player_list(&game) }
            <p class={classes!("player-list-container__points-explanation")}>
                <LocaleComponent keyid="play-view-players-points-explanation" />
            </p>
        </aside>
    }
}

fn view_player_list(game: &Rc<Game>) -> Html {
    if game.players.is_empty() {
        html! {
            <p class={classes!("player-list-container__empty")}>
                <LocaleComponent keyid="play-view-players-no-one-here"/>
            </p>
        }
    } else {
        let player_list_items = game
            .players
            .iter()
            .map(|player: &Player| view_player(game, player))
            .collect::<Html>();
        html! {
            <ul class={classes!("player-list-container__list", "player-list")}>
                { player_list_items }
            </ul>
        }
    }
}

fn view_player(game: &Rc<Game>, player: &Player) -> Html {
    let is_this_player = player.id == game.this_player_id;

    let points_or_watching_html = match &player.play_type {
        PlayType::Player { points } => {
            html! {
                <LocaleComponent keyid="play-view-players-points" args={locale_args([("points", points.into())])}/>
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
    };

    html! {
        <li class={classes!("player-list-element")}>
            <span class={classes!(is_this_player.then_some("player-list-element__player-name--is-current-user"))}>
                {player.name.to_string()}
            </span>
            { view_player_state(game, player) }
            <span class={classes!("player-list-element__points-or-watching")}>
                { points_or_watching_html }
            </span>
        </li>
    }
}

fn view_player_state(game: &Rc<Game>, player: &Player) -> Html {
    match &game.game_state {
        GameState::InLobby => html! {},
        GameState::Playing {
            playing_state: PlayingState::Question { answers, .. },
            ..
        } => {
            let user_has_answered: bool = answers.contains(&player.id);
            if user_has_answered {
                html! {
                    <span class={classes!("player-list-element__player-has-answered")}>{PENCIL_SVG}</span>
                }
            } else {
                Html::default()
            }
        }
        GameState::Playing {
            playing_state:
                PlayingState::Solution {
                    current_question,
                    answers,
                    skip_request,
                    ..
                },
            ..
        } => {
            let user_wants_to_skip: bool = skip_request.contains(&player.id);
            let user_wants_to_skip_html = if user_wants_to_skip {
                html! {
                    <span class={classes!("player-list-element__player-wants-to-skip")}>{FAST_FORWARD_SVG}</span>
                }
            } else {
                Html::default()
            };

            let user_has_answered: bool = answers.contains_key(&player.id);
            let user_has_answered_html = if user_has_answered {
                html! {
                    <span class={classes!("player-list-element__player-has-answered")}>{PENCIL_SVG}</span>
                }
            } else {
                Html::default()
            };

            let user_has_correct_answer: Option<bool> = answers
                .get(&player.id)
                .map(|player_answer| *player_answer == current_question.answer);
            let user_has_correct_answer_html = match user_has_correct_answer {
                Some(true) => {
                    html! {
                        <span class={classes!("player-list-element__players-answer", "player-list-element__player-has-answered--correct")}>{CORRECT_SVG}</span>
                    }
                }
                Some(false) => {
                    html! {
                        <span class={classes!("player-list-element__players-answer", "player-list-element__player-has-answered--incorrect")}>{INCORRECT_SVG}</span>
                    }
                }
                None => Html::default(),
            };

            html! {
                <>
                    { user_wants_to_skip_html }
                    { user_has_answered_html }
                    { user_has_correct_answer_html }
                </>
            }
        }
        GameState::Aftermath { .. } => Html::default(),
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct PlayerListProps {
    #[prop_or_default]
    pub class: Classes,
}
