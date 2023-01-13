use fluent_templates::LanguageIdentifier;

use unic_langid::langid;

use yew::prelude::*;

use components::footer::Footer;
use components::header::Header;
use components::locale::Locale;

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
                    <p><Locale keyid="test-it-works"/></p>
                </main>
                <Footer {on_change_language_identifier}/>
            </ContextProvider<LanguageIdentifier>>
        }
    }
}

pub enum AppMsg {
    ChangeLanguageIdentifier(LanguageIdentifier),
}
