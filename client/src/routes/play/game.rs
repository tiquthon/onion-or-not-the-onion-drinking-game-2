use chrono::Utc;
use std::rc::Rc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    Answer, Game, GameState, PlayingState,
};

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::{locale_args, LocaleComponent};
use crate::components::player_name_type_exit_headline::PlayerNameTypeExitHeadlineComponent;
use crate::components::playerlist::PlayerListComponent;

pub struct GameComponent {
    game: Rc<Game>,
    _context_listener: ContextHandle<Rc<Game>>,

    _update_timer: gloo_timers::callback::Interval,
}

impl GameComponent {
    fn view_remaining_time(&self, ctx: &Context<Self>) -> Html {
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
                                        <>
                                            {" - "}
                                            <LocaleComponent keyid="game-view-solution-playing-state-continuing" />
                                        </>
                                    }
                                } else {
                                    let onclick_skip_button = ctx.link().callback(|_| GameComponentMsg::RequestSkip);
                                    html! {
                                        <button type="button" class={classes!("skip-button")} onclick={onclick_skip_button}>
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

    fn view_question_or_solution(&self, ctx: &Context<Self>) -> Html {
        let this_player_is_watcher = self.game.get_this_player().unwrap().is_watcher();
        if let GameState::Playing { playing_state } = &self.game.game_state {
            match playing_state {
                PlayingState::Question {
                    current_question,
                    own_answer,
                    ..
                } => {
                    let this_player_answer_is_the_onion = own_answer
                        .map(|answer| answer == Answer::TheOnion)
                        .unwrap_or(false)
                        .then_some("button-chosen-outline");
                    let this_player_answer_is_not_the_onion = own_answer
                        .map(|answer| answer == Answer::NotTheOnion)
                        .unwrap_or(false)
                        .then_some("button-chosen-outline");

                    html! {
                        <>
                            <h1 class={classes!("question-headline-at-question-state", "question-headline")}>
                                {current_question.title.clone()}
                            </h1>
                            {
                                if !this_player_is_watcher {
                                    let onclick_the_onion = ctx.link().callback(|_| GameComponentMsg::ChooseAnswer(Answer::TheOnion));
                                    let onclick_not_the_onion = ctx.link().callback(|_| GameComponentMsg::ChooseAnswer(Answer::NotTheOnion));
                                    html! {
                                        <section class={classes!("question-the-onion-not-the-onion-forms-section")}>
                                            <div class={classes!("question-the-onion-form")}>
                                                <button type="button" class={classes!("question-the-onion-form-submit-button", this_player_answer_is_the_onion)} onclick={onclick_the_onion}>
                                                    <LocaleComponent keyid="game-view-question-playing-state-selection-button-the-onion" />
                                                </button>
                                            </div>
                                            <div class={classes!("question-not-the-onion-form")}>
                                                <button type="button" class={classes!("question-not-the-onion-form-submit-button", this_player_answer_is_not_the_onion)} onclick={onclick_not_the_onion}>
                                                    <LocaleComponent keyid="game-view-question-playing-state-selection-button-not-the-onion" />
                                                </button>
                                            </div>
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
                    let this_player_answer = answers.get(&self.game.this_player_id);
                    let this_player_answer_correct = this_player_answer
                        .map(|player_answer| *player_answer == current_question.answer);

                    let question_result_css_class =
                        match (this_player_is_watcher, this_player_answer_correct) {
                            (false, Some(true)) => "question-result-correct",
                            (false, Some(false)) => "question-result-wrong",
                            (true, _) | (false, None) => match current_question.answer {
                                Answer::TheOnion => "question-result-correct",
                                Answer::NotTheOnion => "question-result-wrong",
                            },
                        };

                    let sub_headline_locale = match current_question.answer {
                        Answer::TheOnion => {
                            "game-view-solution-playing-state-sub-headline-the-onion"
                        }
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
                            <h1 class={classes!("question-headline-at-aftermath-state", "question-headline")}>
                                {current_question.question.title.clone()}
                            </h1>
                            <section class={classes!("question-subline")}>
                                <a class={classes!("question-subline-link-to-post")} href={current_question.url.clone()} target="_blank">
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
            GameComponentMsg::ChooseAnswer(answer) => {
                ctx.props().on_choose_answer.emit(answer);
                false
            }
            GameComponentMsg::RequestSkip => {
                ctx.props().on_request_skip.emit(());
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
                    { self.view_remaining_time(ctx) }
                    { self.view_question_or_solution(ctx) }
                    <PlayerListComponent />
                </section>
            </main>
        }
    }
}

pub enum GameComponentMsg {
    MessageContextUpdated(Rc<Game>),
    UpdateByInterval,

    ExitGame,
    ChooseAnswer(Answer),
    RequestSkip,
}

#[derive(yew::Properties, PartialEq)]
pub struct GameComponentProps {
    pub on_exit_game_wish: Callback<()>,
    pub on_choose_answer: Callback<Answer>,
    pub on_request_skip: Callback<()>,
}
