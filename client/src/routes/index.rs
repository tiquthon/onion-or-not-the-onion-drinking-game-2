use std::collections::HashMap;

use fluent_templates::LanguageIdentifier;

use web_sys::{HtmlInputElement, SubmitEvent};

use yew::platform::spawn_local;
use yew::{
    classes, function_component, html, use_context, use_effect_with_deps, use_state_eq, Callback,
    Html, NodeRef, UseStateHandle,
};

use crate::components::locale::{locale, locale_args, LocaleComponent};
use crate::utils::retrieve_browser_location;

#[function_component(IndexComponent)]
pub fn index_component(props: &IndexComponentProps) -> Html {
    let langid = use_context::<LanguageIdentifier>().expect("Missing LanguageIdentifier context.");

    let node_refs = use_state_eq(NodeRefs::default);
    let form_mode = use_state_eq(FormMode::default);
    let error_messages = use_state_eq(ErrorMessages::default);
    let question_scores_distribution = use_state_eq::<Option<HashMap<u64, usize>>, _>(|| None);

    let cloned_question_scores_distribution = question_scores_distribution.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let api_root_url = retrieve_browser_location(None, Some("/api"));
                log::debug!("Retrieved api_root_url as {api_root_url}");

                let response_result =
                    gloo_net::http::Request::get(&format!("{api_root_url}/distribution"))
                        .send()
                        .await;
                let response = match response_result {
                    Ok(response) => Ok(response.json::<HashMap<u64, usize>>().await),
                    Err(error) => Err(error),
                };

                match response {
                    Ok(Ok(distribution)) => {
                        cloned_question_scores_distribution.set(Some(distribution))
                    }
                    Ok(Err(error)) => {
                        log::error!("Failed parsing fetched question scores distribution ({error})")
                    }
                    Err(error) => {
                        log::error!("Failed fetching question scores distribution ({error})")
                    }
                }
            });
        },
        (),
    );

    html! {
        <main class={classes!("main", "centered-primary-content")}>
            <p class={classes!("game-explanation")}>
                <span class={classes!("game-explanation__game-name")}>
                    <LocaleComponent keyid="game-name"/>
                </span>
                {" "}
                <LocaleComponent keyid="game-title-description"/>
            </p>

            { view_form(props, &langid, &node_refs, &form_mode, &error_messages, &question_scores_distribution) }
        </main>
    }
}

fn view_form(
    props: &IndexComponentProps,
    langid: &LanguageIdentifier,
    node_refs: &UseStateHandle<NodeRefs>,
    form_mode: &UseStateHandle<FormMode>,
    error_messages: &UseStateHandle<ErrorMessages>,
    question_scores_distribution: &UseStateHandle<Option<HashMap<u64, usize>>>,
) -> Html {
    let cloned_on_join_lobby = props.on_join_lobby.clone();
    let cloned_on_create_lobby = props.on_create_lobby.clone();
    let cloned_node_refs = node_refs.clone();
    let cloned_form_mode = form_mode.clone();
    let cloned_error_messages = error_messages.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        event.stop_propagation();
        process_form_submission(
            &cloned_on_join_lobby,
            &cloned_on_create_lobby,
            &cloned_node_refs,
            &cloned_form_mode,
            &cloned_error_messages,
        );
    });

    let cloned_invite_code_node_ref = node_refs.invite_code_node_ref.clone();
    let cloned_form_mode = form_mode.clone();
    let invite_code_onkeyup = Callback::from(move |_| {
        let is_invite_code_empty: bool = cloned_invite_code_node_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value()
            .trim()
            .to_string()
            .is_empty();
        if is_invite_code_empty {
            cloned_form_mode.set(FormMode::new_create_game());
        } else {
            cloned_form_mode.set(FormMode::new_join_game());
        }
    });

    let question_count_value: Option<String> = match &**form_mode {
        FormMode::JoinGame => None,
        FormMode::CreateGame { node_refs, .. } => node_refs
            .question_count_node_ref
            .cast()
            .map(|question_count_element: HtmlInputElement| question_count_element.value()),
    };
    let question_count_value = question_count_value.unwrap_or_else(|| "10".to_string());

    let create_game_form_html = match &**form_mode {
        FormMode::JoinGame => Default::default(),
        FormMode::CreateGame {
            error_messages,
            node_refs,
        } => {
            let (minimum_available_score, maximum_available_score, optional_score_and_count) =
                if let Some(question_scores_distribution) = &**question_scores_distribution {
                    let (min_score, max_score) = question_scores_distribution.keys().fold(
                        (None, None),
                        |(min, max), value| {
                            let min =
                                Some(min.map(|other: u64| other.min(*value)).unwrap_or(*value));
                            let max =
                                Some(max.map(|other: u64| other.max(*value)).unwrap_or(*value));
                            (min, max)
                        },
                    );
                    let min_score = min_score.map(|n| n.to_string()).unwrap_or_default();
                    let max_score = max_score.map(|n| n.to_string()).unwrap_or_default();

                    let current_score = node_refs
                        .minimum_score_node_ref
                        .cast()
                        .and_then(|minimum_score_element: HtmlInputElement| {
                            minimum_score_element.value().parse::<u64>().ok()
                        })
                        .unwrap_or(0);
                    let question_count: usize = question_scores_distribution
                        .iter()
                        .filter(|(score, _)| **score >= current_score)
                        .map(|(_, count)| *count)
                        .sum();

                    (min_score, max_score, Some((current_score, question_count)))
                } else {
                    (String::new(), String::new(), None)
                };

            html! {
                <>
                    <label for="question_count">
                        <span class={classes!("form-input-label")}>
                            <LocaleComponent keyid="game-creation-form-max-questions-label"/>
                            {":"}
                        </span>
                        {" "}
                    </label>
                    <input autocomplete="off"
                        class={classes!("input-field")}
                        id="question_count"
                        placeholder={locale("game-creation-form-max-questions-placeholder", langid)}
                        ref={node_refs.question_count_node_ref.clone()}
                        type="number"
                        value={question_count_value} />
                    if let Some(lang_key_id) = error_messages.optional_question_count_error_message_lang_key_id {
                        <p class={classes!("form-error-paragraph", "game-create-join-form__error_paragraph")}>
                            <LocaleComponent keyid={lang_key_id}/>
                        </p>
                    }
                    <p class={classes!("form-description-paragraph", "game-create-join-form__description-paragraph")}>
                        <LocaleComponent keyid="game-creation-form-max-questions-explanation"/>
                    </p>

                    <label for="minimum_score">
                        <span class={classes!("form-input-label")}>
                            <LocaleComponent keyid="game-creation-form-minimum-score-label"/>
                            {":"}
                        </span>
                        {" "}
                    </label>
                    <input autocomplete="off"
                        class={classes!("input-field")}
                        id="minimum_score"
                        max={maximum_available_score}
                        min={minimum_available_score}
                        placeholder={locale("game-creation-form-minimum-score-placeholder", langid)}
                        ref={node_refs.minimum_score_node_ref.clone()}
                        type="number" />
                    if let Some(lang_key_id) = error_messages.optional_minimum_score_error_message_lang_key_id {
                        <p class={classes!("form-error-paragraph", "game-create-join-form__error_paragraph")}>
                            <LocaleComponent keyid={lang_key_id}/>
                        </p>
                    }
                    <p class={classes!("form-description-paragraph", "game-create-join-form__description-paragraph")}>
                        <LocaleComponent keyid="game-creation-form-minimum-score-explanation"/>
                        if let Some((current_score, question_count_with_current_score)) = optional_score_and_count {
                            <br/>
                            <LocaleComponent keyid="game-creation-form-minimum-score-count-of-available"
                                args={locale_args([("score", current_score.into()), ("count", question_count_with_current_score.into())])} />
                        }
                    </p>

                    <label for="timer">
                        <span class={classes!("form-input-label")}>
                            <LocaleComponent keyid="game-creation-form-timer-wanted-label"/>
                            {":"}
                        </span>
                        {" "}
                    </label>
                    <input autocomplete="off"
                        class={classes!("input-field")}
                        id="timer"
                        placeholder={locale("game-creation-form-timer-wanted-placeholder", langid)}
                        ref={node_refs.timer_node_ref.clone()}
                        type="number" />
                    if let Some(lang_key_id) = error_messages.optional_timer_error_message_lang_key_id {
                        <p class={classes!("form-error-paragraph", "game-create-join-form__error_paragraph")}>
                            <LocaleComponent keyid={lang_key_id}/>
                        </p>
                    }
                    <p class={classes!("form-description-paragraph", "game-create-join-form__description-paragraph")}>
                        <LocaleComponent keyid="game-creation-form-timer-wanted-explanation"/>
                    </p>
                </>
            }
        }
    };

    let form_submit_button_value = match &**form_mode {
        FormMode::JoinGame => locale("game-creation-form-submit-value-join", langid),
        FormMode::CreateGame { .. } => locale("game-creation-form-submit-value-create", langid),
    };

    html! {
        <form class={classes!("game-create-join-form")} {onsubmit}>
            <label for="username">
                <span class={classes!("form-input-label")}>
                    <LocaleComponent keyid="game-creation-form-username-label"/>
                    {":"}
                </span>
                {" "}
            </label>
            <input class={classes!("input-field")}
                id="username"
                placeholder={locale("game-creation-form-username-placeholder", langid)}
                required={true}
                ref={node_refs.player_name_node_ref.clone()}
                type="text" />
            if let Some(lang_key_id) = error_messages.optional_player_name_error_message_lang_key_id {
                <p class={classes!("form-error-paragraph", "game-create-join-form__error_paragraph")}>
                    <LocaleComponent keyid={lang_key_id}/>
                </p>
            }

            <label for="invite_code">
                <span class={classes!("form-input-label")}>
                    <LocaleComponent keyid="game-creation-form-invite-code-label"/>
                    {":"}
                </span>
                {" "}
            </label>
            <input autocomplete="off"
                class={classes!("input-field")}
                id="invite_code"
                maxlength="4"
                onkeyup={invite_code_onkeyup}
                placeholder={locale("game-creation-form-invite-code-placeholder", langid)}
                ref={node_refs.invite_code_node_ref.clone()}
                type="text" />
            if let Some(lang_key_id) = error_messages.optional_invite_code_error_message_lang_key_id {
                <p class={classes!("form-error-paragraph", "game-create-join-form__error_paragraph")}>
                    <LocaleComponent keyid={lang_key_id}/>
                </p>
            }

            <p class={classes!("form-description-paragraph", "game-create-join-form__description-paragraph")}>
                <LocaleComponent keyid="game-creation-form-starting-game-explanation"/>
            </p>

            <label class={classes!("form-just-watch-label", "game-create-join-form__just-watch-label")}>
                <input type="checkbox"
                    ref={node_refs.just_watch_node_ref.clone()} />
                {" "}
                <LocaleComponent keyid="game-creation-form-just-watch-label"/>
            </label>

            { create_game_form_html }

            <input class={classes!("button", "game-create-join-form__submit-button")} value={form_submit_button_value} type="submit" />
        </form>
    }
}

fn process_form_submission(
    on_join_lobby: &Callback<JoinLobby>,
    on_create_lobby: &Callback<CreateLobby>,
    node_refs: &NodeRefs,
    form_mode: &UseStateHandle<FormMode>,
    error_messages: &UseStateHandle<ErrorMessages>,
) {
    let mut new_error_messages = ErrorMessages::default();

    let player_name: String = node_refs
        .player_name_node_ref
        .cast::<HtmlInputElement>()
        .unwrap()
        .value()
        .trim()
        .to_string();
    if player_name.is_empty() {
        log::error!("Player name is empty.");
        new_error_messages.optional_player_name_error_message_lang_key_id =
            Some("game-creation-form-error-message-player-name-empty");
    }

    let just_watch: bool = node_refs
        .just_watch_node_ref
        .cast::<HtmlInputElement>()
        .unwrap()
        .checked();

    match &**form_mode {
        FormMode::JoinGame => {
            let invite_code: String = node_refs
                .invite_code_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .trim()
                .to_string();
            if invite_code.is_empty() {
                log::error!("Invite code is empty; this is an internal error.");
                new_error_messages.optional_invite_code_error_message_lang_key_id =
                    Some("game-creation-form-error-message-invite-code-empty");
            }

            if new_error_messages.no_error_set() {
                on_join_lobby.emit(JoinLobby {
                    player_name,
                    invite_code,
                    just_watch,
                });
            }
        }
        FormMode::CreateGame { node_refs, .. } => {
            let mut new_extended_error_messages = ExtendedErrorMessages::default();

            fn parse_trimmed_optional_input<T>(
                element_node_ref: &NodeRef,
                error_message: &mut Option<&'static str>,
                console_on_error_reference: &'static str,
                error_message_lang_id: &'static str,
            ) -> Option<T>
            where
                T: std::str::FromStr,
                <T as std::str::FromStr>::Err: std::fmt::Display,
            {
                let element_str: String =
                    element_node_ref.cast::<HtmlInputElement>().unwrap().value();
                let element_str = element_str.trim();
                if element_str.is_empty() {
                    None
                } else {
                    match element_str.parse() {
                        Ok(element_value) => Some(element_value),
                        Err(error) => {
                            log::error!("Could not parse {console_on_error_reference} ({error})");
                            *error_message = Some(error_message_lang_id);
                            None
                        }
                    }
                }
            }

            let question_count = parse_trimmed_optional_input(
                &node_refs.question_count_node_ref,
                &mut new_extended_error_messages.optional_question_count_error_message_lang_key_id,
                "question count",
                "game-creation-form-error-message-max-questions-invalid",
            );

            let minimum_score = parse_trimmed_optional_input(
                &node_refs.minimum_score_node_ref,
                &mut new_extended_error_messages.optional_minimum_score_error_message_lang_key_id,
                "minimum score",
                "game-creation-form-error-message-minimum-score-invalid",
            );

            let timer = parse_trimmed_optional_input(
                &node_refs.timer_node_ref,
                &mut new_extended_error_messages.optional_timer_error_message_lang_key_id,
                "timer",
                "game-creation-form-error-message-timer-wanted-invalid",
            );

            if new_error_messages.no_error_set() && new_extended_error_messages.no_error_set() {
                on_create_lobby.emit(CreateLobby {
                    player_name,
                    just_watch,
                    count_of_questions: question_count,
                    minimum_score_per_question: minimum_score,
                    maximum_answer_seconds_per_question: timer,
                });
            }

            form_mode.set(FormMode::CreateGame {
                error_messages: new_extended_error_messages,
                node_refs: (*node_refs).clone(),
            });
        }
    }

    error_messages.set(new_error_messages);
}

#[derive(yew::Properties, PartialEq)]
pub struct IndexComponentProps {
    #[prop_or_default]
    pub on_join_lobby: Callback<JoinLobby>,
    #[prop_or_default]
    pub on_create_lobby: Callback<CreateLobby>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JoinLobby {
    pub player_name: String,
    pub invite_code: String,
    pub just_watch: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CreateLobby {
    pub player_name: String,
    pub just_watch: bool,
    pub count_of_questions: Option<u64>,
    pub minimum_score_per_question: Option<i64>,
    pub maximum_answer_seconds_per_question: Option<u64>,
}

#[derive(Default, PartialEq)]
struct NodeRefs {
    player_name_node_ref: NodeRef,
    invite_code_node_ref: NodeRef,
    just_watch_node_ref: NodeRef,
}

#[derive(PartialEq)]
enum FormMode {
    JoinGame,
    CreateGame {
        error_messages: ExtendedErrorMessages,
        node_refs: ExtendedNodeRefs,
    },
}

impl FormMode {
    fn new_create_game() -> Self {
        Self::CreateGame {
            error_messages: Default::default(),
            node_refs: Default::default(),
        }
    }

    fn new_join_game() -> Self {
        Self::JoinGame
    }
}

impl Default for FormMode {
    fn default() -> Self {
        Self::new_create_game()
    }
}

#[derive(Default, PartialEq)]
struct ExtendedErrorMessages {
    optional_question_count_error_message_lang_key_id: Option<&'static str>,
    optional_minimum_score_error_message_lang_key_id: Option<&'static str>,
    optional_timer_error_message_lang_key_id: Option<&'static str>,
}

impl ExtendedErrorMessages {
    fn no_error_set(&self) -> bool {
        self.optional_question_count_error_message_lang_key_id
            .is_none()
            && self
                .optional_minimum_score_error_message_lang_key_id
                .is_none()
            && self.optional_timer_error_message_lang_key_id.is_none()
    }
}

#[derive(Default, PartialEq, Clone)]
struct ExtendedNodeRefs {
    question_count_node_ref: NodeRef,
    minimum_score_node_ref: NodeRef,
    timer_node_ref: NodeRef,
}

#[derive(Default, PartialEq)]
struct ErrorMessages {
    optional_player_name_error_message_lang_key_id: Option<&'static str>,
    optional_invite_code_error_message_lang_key_id: Option<&'static str>,
}

impl ErrorMessages {
    fn no_error_set(&self) -> bool {
        self.optional_player_name_error_message_lang_key_id
            .is_none()
            && self
                .optional_invite_code_error_message_lang_key_id
                .is_none()
    }
}
