use std::rc::Rc;

use chrono::Utc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    Answer, Game, GameState, PlayingState,
};

use yew::{
    classes, function_component, html, use_context, use_effect_with_deps, use_force_update,
    use_state, Callback, Html,
};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::{locale_args, LocaleComponent};
use crate::components::player_name_type_exit_headline::PlayerNameTypeExitHeadlineComponent;
use crate::components::playerlist::PlayerListComponent;

#[function_component(GameComponent)]
pub fn game_component(props: &GameComponentProps) -> Html {
    let game: Rc<Game> = use_context().expect("Missing Game context.");

    let update_timer = use_state::<Option<gloo_timers::callback::Interval>, _>(|| None);
    let force_update = use_force_update();
    let update_by_interval_callback = Callback::from(move |_| force_update.force_update());
    use_effect_with_deps(
        move |_| {
            update_timer.set(Some(gloo_timers::callback::Interval::new(500, move || {
                update_by_interval_callback.emit(());
            })))
        },
        (),
    );

    let invite_code = game.invite_code.to_string();

    let cloned_on_exit_game_wish = props.on_exit_game_wish.clone();
    let on_exit_game_wished = Callback::from(move |_| cloned_on_exit_game_wish.emit(()));

    html! {
        <main class={classes!("main")}>
            <JoinGameComponent {invite_code} />
            <section class={classes!("centered-primary-content", "play-primary-content")}>
                <PlayerNameTypeExitHeadlineComponent {on_exit_game_wished} />
                { view_remaining_time(props, &game) }
                { view_question_or_solution(props, &game) }
                <PlayerListComponent class={classes!("play-primary-content__player-list")} />
            </section>
        </main>
    }
}
fn view_remaining_time(props: &GameComponentProps, game: &Rc<Game>) -> Html {
    let this_player_is_watcher = game.get_this_player().unwrap().is_watcher();

    if let GameState::Playing { playing_state, .. } = &game.game_state {
        match playing_state {
            PlayingState::Question { time_until, .. } => match time_until {
                Some(some_time_until) => {
                    let duration = *some_time_until - Utc::now();
                    html! {
                        <section class={classes!("remaining-question-time")}>
                            <LocaleComponent keyid="game-view-question-playing-state-remaining-seconds"
                                args={locale_args([("seconds", duration.num_seconds().into())])} />
                        </section>
                    }
                }
                None => {
                    html! {
                        <section class={classes!("remaining-question-time")}>
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
                    <section class={classes!("remaining-solution-time")}>
                        <LocaleComponent keyid="game-view-solution-playing-state-remaining-seconds"
                            args={locale_args([("seconds", duration.num_seconds().into())])} />
                        {
                            if this_player_is_watcher {
                                html! {}
                            } else if skip_request.contains(&game.this_player_id) {
                                html! {
                                    <>
                                        {" - "}
                                        <LocaleComponent keyid="game-view-solution-playing-state-continuing" />
                                    </>
                                }
                            } else {
                                let cloned_on_request_skip = props.on_request_skip.clone();
                                let onclick_skip_button = Callback::from(move |_| cloned_on_request_skip.emit(()));
                                html! {
                                    <button type="button" class={classes!("button", "solution-skip-button")} onclick={onclick_skip_button}>
                                        <LocaleComponent keyid="game-view-solution-playing-state-continue" />
                                    </button>
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

fn view_question_or_solution(props: &GameComponentProps, game: &Rc<Game>) -> Html {
    let this_player_is_watcher = game.get_this_player().unwrap().is_watcher();
    if let GameState::Playing { playing_state, .. } = &game.game_state {
        match playing_state {
            PlayingState::Question {
                current_question,
                own_answer,
                ..
            } => {
                let this_player_answer_is_the_onion = own_answer
                    .map(|answer| answer == Answer::TheOnion)
                    .unwrap_or(false)
                    .then_some("question-button--chosen");
                let this_player_answer_is_not_the_onion = own_answer
                    .map(|answer| answer == Answer::NotTheOnion)
                    .unwrap_or(false)
                    .then_some("question-button--chosen");

                html! {
                    <>
                        <h1 class={classes!("question-headline")}>
                            {current_question.title.clone()}
                        </h1>
                        {
                            if !this_player_is_watcher {
                                let cloned_on_choose_answer = props.on_choose_answer.clone();
                                let onclick_the_onion = Callback::from(move |_| cloned_on_choose_answer.emit(Answer::TheOnion));
                                let cloned_on_choose_answer = props.on_choose_answer.clone();
                                let onclick_not_the_onion = Callback::from(move |_| cloned_on_choose_answer.emit(Answer::NotTheOnion));
                                html! {
                                    <section class={classes!("question-buttons-container")}>
                                        <button type="button" class={classes!("button", "question-button--the-onion", this_player_answer_is_the_onion)} onclick={onclick_the_onion}>
                                            <LocaleComponent keyid="game-view-question-playing-state-selection-button-the-onion" />
                                        </button>
                                        <button type="button" class={classes!("button", "question-button--not-the-onion", this_player_answer_is_not_the_onion)} onclick={onclick_not_the_onion}>
                                            <LocaleComponent keyid="game-view-question-playing-state-selection-button-not-the-onion" />
                                        </button>
                                    </section>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </>
                }
            }
            PlayingState::Solution {
                current_question,
                answers,
                ..
            } => {
                let this_player_answer = answers.get(&game.this_player_id);
                let this_player_answer_correct = this_player_answer
                    .map(|player_answer| *player_answer == current_question.answer);

                let question_result_css_class = if this_player_is_watcher {
                    match current_question.answer {
                        Answer::TheOnion => "question-result--correct",
                        Answer::NotTheOnion => "question-result--wrong",
                    }
                } else {
                    match this_player_answer_correct {
                        Some(true) => "question-result--correct",
                        Some(false) => "question-result--wrong",
                        None => "question-result--missing",
                    }
                };

                let sub_headline_locale = match current_question.answer {
                    Answer::TheOnion => "game-view-solution-playing-state-sub-headline-the-onion",
                    Answer::NotTheOnion => {
                        "game-view-solution-playing-state-sub-headline-not-the-onion"
                    }
                };

                html! {
                    <>
                        <p class={classes!("question-result", question_result_css_class)}>
                            <LocaleComponent keyid={sub_headline_locale} />
                            {
                                if this_player_is_watcher {
                                    html! {}
                                } else {
                                    let this_player_answer_locale = match this_player_answer_correct {
                                        Some(true) => "game-view-solution-playing-state-sub-headline-player-answer-correct",
                                        Some(false) => "game-view-solution-playing-state-sub-headline-player-answer-wrong",
                                        None => "game-view-solution-playing-state-sub-headline-player-answer-missing",
                                    };
                                    html! {
                                        <>
                                            <br/>
                                            <LocaleComponent keyid={this_player_answer_locale} />
                                        </>
                                    }
                                }
                            }
                        </p>
                        <h1 class={classes!("question-headline")}>
                            {current_question.question.title.clone()}
                        </h1>
                        <section>
                            <a class={classes!("link-to-original-news-article")} href={current_question.url.clone()} target="_blank">
                                <LocaleComponent keyid="game-view-solution-playing-state-link-to-newspaper-posting-anchor-text"/>
                            </a>
                        </section>
                        <img class={classes!("question-picture")} src={current_question.preview_image_url.clone().unwrap_or_default()}/>
                    </>
                }
            }
        }
    } else {
        unreachable!()
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct GameComponentProps {
    pub on_exit_game_wish: Callback<()>,
    pub on_choose_answer: Callback<Answer>,
    pub on_request_skip: Callback<()>,
}
