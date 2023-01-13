use fluent_templates::LanguageIdentifier;

use unic_langid::langid;

use yew::prelude::*;

use components::footer::Footer;
use components::header::Header;
use components::locale::{locale, Locale};

pub mod components;

pub struct App {
    langid: LanguageIdentifier,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            langid: langid!("en-US"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ChangeLanguageIdentifier(lid) => {
                self.langid = lid;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_language_identifier = ctx.link().callback(AppMsg::ChangeLanguageIdentifier);
        html! {
            <ContextProvider<LanguageIdentifier> context={self.langid.clone()}>
                <Header/>
                <main>
                    <p><span style="font-weight: bold;"><Locale keyid="game-name"/></span>{" "}<Locale keyid="game-title-description"/></p>
                    <form>
                        <label>
                            <Locale keyid="game-creation-form-username-label"/>
                            {": "}
                            <input type="text" placeholder={locale("game-creation-form-username-placeholder", &self.langid)}/>
                        </label>
                        <label>
                            <Locale keyid="game-creation-form-invite-code-label"/>
                            {": "}
                            <input type="text" placeholder={locale("game-creation-form-invite-code-placeholder", &self.langid)}/>
                        </label>
                        <p><Locale keyid="game-creation-form-starting-game-explanation"/></p>
                        <label><input type="checkbox"/>{" "}<Locale keyid="game-creation-form-just-watch-label"/></label>
                        <label><Locale keyid="game-creation-form-max-questions-label"/>{": "}<input type="text" value="10" placeholder={locale("game-creation-form-max-questions-placeholder", &self.langid)}/></label>
                        <p><Locale keyid="game-creation-form-max-questions-explanation"/></p>
                        <label><Locale keyid="game-creation-form-minimum-score-label"/>{": "}<input type="text" placeholder={locale("game-creation-form-minimum-score-placeholder", &self.langid)}/></label>
                        <p><Locale keyid="game-creation-form-minimum-score-explanation"/></p>
                        <label><Locale keyid="game-creation-form-timer-wanted-label"/>{": "}<input type="text" placeholder={locale("game-creation-form-timer-wanted-placeholder", &self.langid)}/></label>
                        <p><Locale keyid="game-creation-form-timer-wanted-explanation"/></p>
                        <input type="submit" value={locale("game-creation-form-submit-value-create", &self.langid)}/>
                    </form>
                </main>
                <Footer {on_change_language_identifier}/>
            </ContextProvider<LanguageIdentifier>>
        }
    }
}

pub enum AppMsg {
    ChangeLanguageIdentifier(LanguageIdentifier),
}
