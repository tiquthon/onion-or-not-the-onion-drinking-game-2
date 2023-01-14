use fluent_templates::LanguageIdentifier;

use yew::{html, Component, Context, ContextHandle, Html};

use crate::components::locale::{locale, LocaleComponent};

pub struct IndexComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <p><span style="font-weight: bold;"><LocaleComponent keyid="game-name"/></span>{" "}<LocaleComponent keyid="game-title-description"/></p>
                <form>
                    <label>
                        <LocaleComponent keyid="game-creation-form-username-label"/>
                        {": "}
                        <input type="text" placeholder={locale("game-creation-form-username-placeholder", &self.langid)}/>
                    </label>
                    <label>
                        <LocaleComponent keyid="game-creation-form-invite-code-label"/>
                        {": "}
                        <input type="text" placeholder={locale("game-creation-form-invite-code-placeholder", &self.langid)}/>
                    </label>
                    <p><LocaleComponent keyid="game-creation-form-starting-game-explanation"/></p>
                    <label>
                        <input type="checkbox"/>
                        {" "}
                        <LocaleComponent keyid="game-creation-form-just-watch-label"/>
                    </label>
                    <label>
                        <LocaleComponent keyid="game-creation-form-max-questions-label"/>
                        {": "}
                        <input type="text" value="10" placeholder={locale("game-creation-form-max-questions-placeholder", &self.langid)}/>
                    </label>
                    <p><LocaleComponent keyid="game-creation-form-max-questions-explanation"/></p>
                    <label>
                        <LocaleComponent keyid="game-creation-form-minimum-score-label"/>
                        {": "}
                        <input type="text" placeholder={locale("game-creation-form-minimum-score-placeholder", &self.langid)}/>
                    </label>
                    <p><LocaleComponent keyid="game-creation-form-minimum-score-explanation"/></p>
                    <label>
                        <LocaleComponent keyid="game-creation-form-timer-wanted-label"/>
                        {": "}
                        <input type="text" placeholder={locale("game-creation-form-timer-wanted-placeholder", &self.langid)}/>
                    </label>
                    <p><LocaleComponent keyid="game-creation-form-timer-wanted-explanation"/></p>
                    <input type="submit" value={locale("game-creation-form-submit-value-create", &self.langid)}/>
                </form>
            </main>
        }
    }
}

pub enum IndexComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
}
