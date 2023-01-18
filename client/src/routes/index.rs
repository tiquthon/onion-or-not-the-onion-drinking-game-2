use fluent_templates::LanguageIdentifier;

use web_sys::{HtmlInputElement, SubmitEvent};

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html, NodeRef};

use crate::components::locale::{locale, LocaleComponent};

pub struct IndexComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,

    player_name_error_message_langkeyid: Option<String>,
    invite_code_error_message_langkeyid: Option<String>,

    player_name_node_ref: NodeRef,
    invite_code_node_ref: NodeRef,

    create_lobby_settings_visibility: CreateLobbySettingsVisibility,
}

impl Component for IndexComponent {
    type Message = IndexComponentMsg;
    type Properties = IndexComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(IndexComponentMsg::MessageContextUpdated),
            )
            .expect("Missing LanguageIdentifier context.");
        Self {
            langid,
            _context_listener: context_listener,

            player_name_error_message_langkeyid: None,
            invite_code_error_message_langkeyid: None,

            player_name_node_ref: NodeRef::default(),
            invite_code_node_ref: NodeRef::default(),

            create_lobby_settings_visibility: CreateLobbySettingsVisibility::visible_default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
            IndexComponentMsg::FormSubmitted => {
                let mut error_found = false;

                let player_name: String = self
                    .player_name_node_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .trim()
                    .to_string();
                if player_name.is_empty() {
                    log::error!("Player name is empty.");
                    self.player_name_error_message_langkeyid =
                        Some("game-creation-form-error-message-player-name-empty".to_string());
                    error_found = true;
                }

                match &mut self.create_lobby_settings_visibility {
                    CreateLobbySettingsVisibility::Visible {
                        max_questions_error_message_langkeyid,
                        minimum_score_error_message_langkeyid,
                        timer_wanted_error_message_langkeyid,

                        max_questions_input_node_ref,
                        minimum_score_input_node_ref,
                        timer_wanted_input_node_ref,
                    } => {
                        fn special_string_parse(input: &NodeRef) -> anyhow::Result<Option<u64>> {
                            let value: String = input.cast::<HtmlInputElement>().unwrap().value();
                            let value_str = value.trim();
                            if value_str.is_empty() {
                                Ok(None)
                            } else {
                                value_str.parse::<u64>().map(Some).map_err(Into::into)
                            }
                        }

                        *max_questions_error_message_langkeyid = None;
                        *minimum_score_error_message_langkeyid = None;
                        *timer_wanted_error_message_langkeyid = None;

                        let count_of_questions_result =
                            special_string_parse(max_questions_input_node_ref);
                        let minimum_score_of_questions_result =
                            special_string_parse(minimum_score_input_node_ref);
                        let timer_result = special_string_parse(timer_wanted_input_node_ref);

                        if let Err(error) = &count_of_questions_result {
                            log::error!("Could not parse count of questions result ({error}).");
                            *max_questions_error_message_langkeyid = Some(
                                "game-creation-form-error-message-max-questions-invalid"
                                    .to_string(),
                            );
                            error_found = true;
                        }

                        if let Err(error) = &minimum_score_of_questions_result {
                            log::error!(
                                "Could not parse minimum score of questions result ({error})."
                            );
                            *minimum_score_error_message_langkeyid = Some(
                                "game-creation-form-error-message-minimum-score-invalid"
                                    .to_string(),
                            );
                            error_found = true;
                        }

                        if let Err(error) = &timer_result {
                            log::error!("Could not parse timer wanted result ({error}).");
                            *timer_wanted_error_message_langkeyid = Some(
                                "game-creation-form-error-message-timer-wanted-invalid".to_string(),
                            );
                            error_found = true;
                        }

                        if let (
                            false,
                            Ok(count_of_questions),
                            Ok(minimum_score_of_questions),
                            Ok(timer),
                        ) = (
                            error_found,
                            count_of_questions_result,
                            minimum_score_of_questions_result,
                            timer_result,
                        ) {
                            ctx.props().on_create_lobby.emit(CreateLobby {
                                player_name,
                                count_of_questions,
                                minimum_score_of_questions,
                                timer,
                            });
                        }
                    }
                    CreateLobbySettingsVisibility::Hidden {
                        just_watch_checkbox_node_ref,
                    } => {
                        let invite_code: String = self
                            .invite_code_node_ref
                            .cast::<HtmlInputElement>()
                            .unwrap()
                            .value()
                            .trim()
                            .to_string();
                        if invite_code.is_empty() {
                            log::error!("Invite code is empty.");
                            self.invite_code_error_message_langkeyid = Some(
                                "game-creation-form-error-message-invite-code-empty".to_string(),
                            );
                            error_found = true;
                        }

                        let just_watch: bool = just_watch_checkbox_node_ref
                            .cast::<HtmlInputElement>()
                            .unwrap()
                            .checked();
                        if !error_found {
                            ctx.props().on_join_lobby.emit(JoinLobby {
                                player_name,
                                invite_code,
                                just_watch,
                            });
                        }
                    }
                }
                error_found
            }
            IndexComponentMsg::InviteCodeValueChanged => {
                let empty_invite_code = self
                    .invite_code_node_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .is_empty();
                self.create_lobby_settings_visibility = if empty_invite_code {
                    CreateLobbySettingsVisibility::visible_default()
                } else {
                    CreateLobbySettingsVisibility::hidden_default()
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let form_onsubmit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            event.stop_propagation();
            IndexComponentMsg::FormSubmitted
        });
        let invite_code_onkeyup = ctx
            .link()
            .callback(|_| IndexComponentMsg::InviteCodeValueChanged);
        html! {
            <main>
                <p class={classes!("index-game-explanation")}><span style="font-weight: bold;"><LocaleComponent keyid="game-name"/></span>{" "}<LocaleComponent keyid="game-title-description"/></p>

                <form class={classes!("index-form")} onsubmit={form_onsubmit}>

                    <label for="username_new_game">
                        <span class={classes!("index-form-description-label")}>
                            <LocaleComponent keyid="game-creation-form-username-label"/>
                            {":"}
                        </span>
                        {" "}
                    </label>
                    <input type="text" id="username_new_game" placeholder={locale("game-creation-form-username-placeholder", &self.langid)} ref={self.player_name_node_ref.clone()}/>
                    if let Some(player_name_error_message_langkeyid) = &self.player_name_error_message_langkeyid {
                        <p class={classes!("index-form-error-paragraph")}>
                            <LocaleComponent keyid={player_name_error_message_langkeyid.clone()}/>
                        </p>
                    }

                    <label for="invite_code">
                        <span class={classes!("index-form-description-label")}>
                            <LocaleComponent keyid="game-creation-form-invite-code-label"/>
                            {":"}
                        </span>
                        {" "}
                    </label>
                    <input type="text" id="invite_code" placeholder={locale("game-creation-form-invite-code-placeholder", &self.langid)} onkeyup={invite_code_onkeyup} ref={self.invite_code_node_ref.clone()}/>
                    if let Some(invite_code_error_message_langkeyid) = &self.invite_code_error_message_langkeyid {
                        <p class={classes!("index-form-error-paragraph")}>
                            <LocaleComponent keyid={invite_code_error_message_langkeyid.clone()}/>
                        </p>
                    }

                    <p class={classes!("index-form-description-paragraph")}><LocaleComponent keyid="game-creation-form-starting-game-explanation"/></p>

                    {
                        match &self.create_lobby_settings_visibility {
                            CreateLobbySettingsVisibility::Visible {
                                max_questions_error_message_langkeyid,
                                minimum_score_error_message_langkeyid,
                                timer_wanted_error_message_langkeyid,

                                max_questions_input_node_ref,
                                minimum_score_input_node_ref,
                                timer_wanted_input_node_ref,
                            } => html! {
                                <>
                                    <label for="max-questions">
                                        <LocaleComponent keyid="game-creation-form-max-questions-label"/>
                                        {": "}
                                    </label>
                                    <input type="text" id="max-questions" value="10" placeholder={locale("game-creation-form-max-questions-placeholder", &self.langid)} ref={max_questions_input_node_ref.clone()}/>
                                    if let Some(max_questions_error_message_langkeyid) = &max_questions_error_message_langkeyid {
                                        <p class={classes!("index-form-error-paragraph")}>
                                            <LocaleComponent keyid={max_questions_error_message_langkeyid.clone()}/>
                                        </p>
                                    }
                                    <p class={classes!("index-form-description-paragraph")}><LocaleComponent keyid="game-creation-form-max-questions-explanation"/></p>

                                    <label for="minimum-score">
                                        <LocaleComponent keyid="game-creation-form-minimum-score-label"/>
                                        {": "}
                                    </label>
                                    <input type="text" id="minimum-score" placeholder={locale("game-creation-form-minimum-score-placeholder", &self.langid)} ref={minimum_score_input_node_ref.clone()}/>
                                    if let Some(minimum_score_error_message_langkeyid) = &minimum_score_error_message_langkeyid {
                                        <p class={classes!("index-form-error-paragraph")}>
                                            <LocaleComponent keyid={minimum_score_error_message_langkeyid.clone()}/>
                                        </p>
                                    }
                                    <p class={classes!("index-form-description-paragraph")}><LocaleComponent keyid="game-creation-form-minimum-score-explanation"/></p>

                                    <label for="timer-wanted">
                                        <LocaleComponent keyid="game-creation-form-timer-wanted-label"/>
                                        {": "}
                                    </label>
                                    <input type="text" id="timer-wanted" placeholder={locale("game-creation-form-timer-wanted-placeholder", &self.langid)} ref={timer_wanted_input_node_ref.clone()}/>
                                    if let Some(timer_wanted_error_message_langkeyid) = &timer_wanted_error_message_langkeyid {
                                        <p class={classes!("index-form-error-paragraph")}>
                                            <LocaleComponent keyid={timer_wanted_error_message_langkeyid.clone()}/>
                                        </p>
                                    }
                                    <p class={classes!("index-form-description-paragraph")}><LocaleComponent keyid="game-creation-form-timer-wanted-explanation"/></p>
                                </>
                            },
                            CreateLobbySettingsVisibility::Hidden { just_watch_checkbox_node_ref } => html! {
                                <label class={classes!("just-watch-new-game-label")}>
                                    <input type="checkbox" ref={just_watch_checkbox_node_ref.clone()}/>
                                    {" "}
                                    <LocaleComponent keyid="game-creation-form-just-watch-label"/>
                                </label>
                            },
                        }
                    }

                    <input type="submit" class={classes!("start-or-join-game-button")} value={
                        match &self.create_lobby_settings_visibility {
                            CreateLobbySettingsVisibility::Visible { .. } => locale("game-creation-form-submit-value-create", &self.langid),
                            CreateLobbySettingsVisibility::Hidden { .. } => locale("game-creation-form-submit-value-join", &self.langid),
                        }
                    }/>

                </form>
            </main>
        }
    }
}

pub enum IndexComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
    FormSubmitted,
    InviteCodeValueChanged,
}

#[derive(yew::Properties, PartialEq)]
pub struct IndexComponentProps {
    #[prop_or_default]
    pub on_join_lobby: Callback<JoinLobby>,
    #[prop_or_default]
    pub on_create_lobby: Callback<CreateLobby>,
}

#[derive(Debug)]
pub struct JoinLobby {
    pub player_name: String,
    pub invite_code: String,
    pub just_watch: bool,
}

#[derive(Debug)]
pub struct CreateLobby {
    pub player_name: String,
    pub count_of_questions: Option<u64>,
    pub minimum_score_of_questions: Option<u64>,
    pub timer: Option<u64>,
}

enum CreateLobbySettingsVisibility {
    Visible {
        max_questions_error_message_langkeyid: Option<String>,
        minimum_score_error_message_langkeyid: Option<String>,
        timer_wanted_error_message_langkeyid: Option<String>,

        max_questions_input_node_ref: NodeRef,
        minimum_score_input_node_ref: NodeRef,
        timer_wanted_input_node_ref: NodeRef,
    },
    Hidden {
        just_watch_checkbox_node_ref: NodeRef,
    },
}

impl CreateLobbySettingsVisibility {
    fn visible_default() -> Self {
        Self::Visible {
            max_questions_error_message_langkeyid: None,
            minimum_score_error_message_langkeyid: None,
            timer_wanted_error_message_langkeyid: None,

            max_questions_input_node_ref: Default::default(),
            minimum_score_input_node_ref: Default::default(),
            timer_wanted_input_node_ref: Default::default(),
        }
    }

    fn hidden_default() -> Self {
        Self::Hidden {
            just_watch_checkbox_node_ref: Default::default(),
        }
    }
}
