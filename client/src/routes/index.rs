use fluent_templates::LanguageIdentifier;

use web_sys::{HtmlInputElement, SubmitEvent};

use yew::{classes, html, Component, Context, ContextHandle, Html, NodeRef};

use crate::components::locale::{locale, LocaleComponent};
use crate::components::messages::{ClosingCapability, Message, MessageLevel, MessagesComponent};

pub struct IndexComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,

    messages: Vec<Message>,

    state: IndexComponentState,
}

impl Component for IndexComponent {
    type Message = IndexComponentMsg;
    type Properties = ();

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

            messages: vec![Message {
                text: "This is a test message.".into(),
                level: MessageLevel::Warn,
                closable: ClosingCapability::Closable,
            }],

            state: IndexComponentState::Index {
                player_name_node_ref: NodeRef::default(),
                invite_code_node_ref: NodeRef::default(),
                create_lobby_settings_visibility: CreateLobbySettingsVisibility::Visible {
                    max_questions_input_node_ref: NodeRef::default(),
                    minimum_score_input_node_ref: NodeRef::default(),
                    timer_wanted_input_node_ref: NodeRef::default(),
                },
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                log::info!("Form Submitted");
                false
            }
            IndexComponentMsg::InviteCodeValueChanged => {
                // TODO
                #[allow(irrefutable_let_patterns)]
                if let IndexComponentState::Index {
                    invite_code_node_ref,
                    create_lobby_settings_visibility,
                    ..
                } = &mut self.state
                {
                    let empty_invite_code = invite_code_node_ref
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value()
                        .is_empty();
                    *create_lobby_settings_visibility = if empty_invite_code {
                        CreateLobbySettingsVisibility::Visible {
                            max_questions_input_node_ref: NodeRef::default(),
                            minimum_score_input_node_ref: NodeRef::default(),
                            timer_wanted_input_node_ref: NodeRef::default(),
                        }
                    } else {
                        CreateLobbySettingsVisibility::Hidden
                    };
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_message_closed = ctx
            .link()
            .callback(IndexComponentMsg::MessagesComponentOnMessageClosed);
        html! {
            <main>
                <p class={classes!("index-game-explanation")}><span style="font-weight: bold;"><LocaleComponent keyid="game-name"/></span>{" "}<LocaleComponent keyid="game-title-description"/></p>
                <MessagesComponent class={classes!("index-messages")} messages={self.messages.clone()} {on_message_closed} />
                {
                    match &self.state {
                        IndexComponentState::Index { player_name_node_ref, invite_code_node_ref, create_lobby_settings_visibility } => self.view_index(ctx, player_name_node_ref, invite_code_node_ref, create_lobby_settings_visibility),
                        // TODO: IndexComponentState::Connecting => self.view_connecting(ctx),
                    }
                }
            </main>
        }
    }
}

impl IndexComponent {
    fn view_index(
        &self,
        ctx: &Context<Self>,
        player_name_node_ref: &NodeRef,
        invite_code_node_ref: &NodeRef,
        create_lobby_settings_visibility: &CreateLobbySettingsVisibility,
    ) -> Html {
        let form_onsubmit = ctx.link().callback(|event: SubmitEvent| {
            event.prevent_default();
            event.stop_propagation();
            IndexComponentMsg::FormSubmitted
        });
        let invite_code_onkeyup = ctx
            .link()
            .callback(|_| IndexComponentMsg::InviteCodeValueChanged);
        html! {
            <form class={classes!("index-form")} onsubmit={form_onsubmit}>
                <label for="username_new_game">
                    <span class={classes!("index-form-description-label")}>
                        <LocaleComponent keyid="game-creation-form-username-label"/>
                        {":"}
                    </span>
                    {" "}
                </label>
                <input type="text" id="username_new_game" placeholder={locale("game-creation-form-username-placeholder", &self.langid)} ref={player_name_node_ref.clone()}/>

                <label for="invite_code">
                    <span class={classes!("index-form-description-label")}>
                        <LocaleComponent keyid="game-creation-form-invite-code-label"/>
                        {":"}
                    </span>
                    {" "}
                </label>
                <input type="text" id="invite_code" placeholder={locale("game-creation-form-invite-code-placeholder", &self.langid)} onkeyup={invite_code_onkeyup} ref={invite_code_node_ref.clone()}/>

                <p class={classes!("index-form-description-paragraph")}><LocaleComponent keyid="game-creation-form-starting-game-explanation"/></p>

                <label class={classes!("just-watch-new-game-label")}>
                    <input type="checkbox"/>
                    {" "}
                    <LocaleComponent keyid="game-creation-form-just-watch-label"/>
                </label>

                {
                    match create_lobby_settings_visibility {
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
                        CreateLobbySettingsVisibility::Hidden => html! {},
                    }
                }

                <input type="submit" class={classes!("start-or-join-game-button")} value={
                    match create_lobby_settings_visibility {
                        CreateLobbySettingsVisibility::Visible { .. } => locale("game-creation-form-submit-value-create", &self.langid),
                        CreateLobbySettingsVisibility::Hidden => locale("game-creation-form-submit-value-join", &self.langid),
                    }
                }/>
            </form>
        }
    }

    /* TODO: fn view_connecting(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>{"Connecting"}</p>
        }
    }*/
}

pub enum IndexComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
    MessagesComponentOnMessageClosed(Message),
    FormSubmitted,
    InviteCodeValueChanged,
}

enum IndexComponentState {
    Index {
        player_name_node_ref: NodeRef,
        invite_code_node_ref: NodeRef,
        create_lobby_settings_visibility: CreateLobbySettingsVisibility,
    },
    // TODO: Connecting,
}

enum CreateLobbySettingsVisibility {
    Visible {
        max_questions_input_node_ref: NodeRef,
        minimum_score_input_node_ref: NodeRef,
        timer_wanted_input_node_ref: NodeRef,
    },
    Hidden,
}
