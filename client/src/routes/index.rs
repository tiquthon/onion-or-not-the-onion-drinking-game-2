use fluent_templates::LanguageIdentifier;

use web_sys::{HtmlInputElement, SubmitEvent};

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html, NodeRef};

use crate::components::locale::{locale, LocaleComponent};
use crate::components::messages::{ClosingCapability, Message, MessageLevel, MessagesComponent};

pub struct IndexComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,

    messages: Vec<Message>,

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

            messages: Vec::new(),

            player_name_node_ref: NodeRef::default(),
            invite_code_node_ref: NodeRef::default(),
            create_lobby_settings_visibility: CreateLobbySettingsVisibility::Visible {
                max_questions_input_node_ref: NodeRef::default(),
                minimum_score_input_node_ref: NodeRef::default(),
                timer_wanted_input_node_ref: NodeRef::default(),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
            IndexComponentMsg::MessagesComponentOnMessageClosed(message) => {
                self.messages.retain(|other| *other != message);
                true
            }
            IndexComponentMsg::FormSubmitted => {
                let mut error_found = false;
                let player_name: String = self
                    .player_name_node_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                if player_name.trim().is_empty() {
                    log::error!("Missing player name.");
                    self.messages.push(Message {
                        // TODO: LOCALIZE
                        text: "PLAYER NAME EMPTY".into(),
                        level: MessageLevel::Error,
                        closable: ClosingCapability::Closable,
                    });
                    error_found = true;
                }
                match &self.create_lobby_settings_visibility {
                    CreateLobbySettingsVisibility::Visible {
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
                                use anyhow::Context;
                                value_str.parse::<u64>().map(Some).context("")
                            }
                        }

                        let count_of_questions_result =
                            special_string_parse(max_questions_input_node_ref);
                        let minimum_score_of_questions_result =
                            special_string_parse(minimum_score_input_node_ref);
                        let timer_result = special_string_parse(timer_wanted_input_node_ref);

                        if let Err(error) = &count_of_questions_result {
                            log::error!("Failed parsing count of questions: {error}");
                            self.messages.push(Message {
                                // TODO: LOCALIZE
                                text: "FAILED PARSING COUNT OF QUESTIONS".into(),
                                level: MessageLevel::Error,
                                closable: ClosingCapability::Closable,
                            });
                            error_found = true;
                        }

                        if let Err(error) = &minimum_score_of_questions_result {
                            log::error!("Failed parsing minimum score of questions: {error}");
                            self.messages.push(Message {
                                // TODO: LOCALIZE
                                text: "FAILED PARSING MINIMUM SCORE OF QUESTIONS".into(),
                                level: MessageLevel::Error,
                                closable: ClosingCapability::Closable,
                            });
                            error_found = true;
                        }

                        if let Err(error) = &timer_result {
                            log::error!("Failed parsing timer: {error}");
                            self.messages.push(Message {
                                // TODO: LOCALIZE
                                text: "FAILED PARSING TIMER".into(),
                                level: MessageLevel::Error,
                                closable: ClosingCapability::Closable,
                            });
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
                            .value();
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
                    CreateLobbySettingsVisibility::Visible {
                        max_questions_input_node_ref: NodeRef::default(),
                        minimum_score_input_node_ref: NodeRef::default(),
                        timer_wanted_input_node_ref: NodeRef::default(),
                    }
                } else {
                    CreateLobbySettingsVisibility::Hidden {
                        just_watch_checkbox_node_ref: NodeRef::default(),
                    }
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_message_closed = ctx
            .link()
            .callback(IndexComponentMsg::MessagesComponentOnMessageClosed);
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
                <MessagesComponent class={classes!("index-messages")} messages={self.messages.clone()} {on_message_closed} />
                <form class={classes!("index-form")} onsubmit={form_onsubmit}>
                    <label for="username_new_game">
                        <span class={classes!("index-form-description-label")}>
                            <LocaleComponent keyid="game-creation-form-username-label"/>
                            {":"}
                        </span>
                        {" "}
                    </label>
                    <input type="text" id="username_new_game" placeholder={locale("game-creation-form-username-placeholder", &self.langid)} ref={self.player_name_node_ref.clone()}/>

                    <label for="invite_code">
                        <span class={classes!("index-form-description-label")}>
                            <LocaleComponent keyid="game-creation-form-invite-code-label"/>
                            {":"}
                        </span>
                        {" "}
                    </label>
                    <input type="text" id="invite_code" placeholder={locale("game-creation-form-invite-code-placeholder", &self.langid)} onkeyup={invite_code_onkeyup} ref={self.invite_code_node_ref.clone()}/>

                    <p class={classes!("index-form-description-paragraph")}><LocaleComponent keyid="game-creation-form-starting-game-explanation"/></p>

                    {
                        match &self.create_lobby_settings_visibility {
                            CreateLobbySettingsVisibility::Visible {
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
                                    <p class={classes!("index-form-description-paragraph")}><LocaleComponent keyid="game-creation-form-max-questions-explanation"/></p>

                                    <label for="minimum-score">
                                        <LocaleComponent keyid="game-creation-form-minimum-score-label"/>
                                        {": "}
                                    </label>
                                    <input type="text" id="minimum-score" placeholder={locale("game-creation-form-minimum-score-placeholder", &self.langid)} ref={minimum_score_input_node_ref.clone()}/>
                                    <p class={classes!("index-form-description-paragraph")}><LocaleComponent keyid="game-creation-form-minimum-score-explanation"/></p>

                                    <label for="timer-wanted">
                                        <LocaleComponent keyid="game-creation-form-timer-wanted-label"/>
                                        {": "}
                                    </label>
                                    <input type="text" id="timer-wanted" placeholder={locale("game-creation-form-timer-wanted-placeholder", &self.langid)} ref={timer_wanted_input_node_ref.clone()}/>
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
    MessagesComponentOnMessageClosed(Message),
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
        max_questions_input_node_ref: NodeRef,
        minimum_score_input_node_ref: NodeRef,
        timer_wanted_input_node_ref: NodeRef,
    },
    Hidden {
        just_watch_checkbox_node_ref: NodeRef,
    },
}
