use fluent_templates::LanguageIdentifier;

use unic_langid::langid;

use yew::prelude::*;

pub mod components;

use components::footer::Footer;
use components::header::Header;
use components::locale::Locale;

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
            <>
                <Header langid={self.langid.clone()}/>
                <main>
                    <p><Locale keyid="test-it-works" langid={self.langid.clone()}/></p>
                </main>
                <Footer langid={self.langid.clone()} {on_change_language_identifier}/>
            </>
        }
    }
}

pub enum AppMsg {
    ChangeLanguageIdentifier(LanguageIdentifier),
}
