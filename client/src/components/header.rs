use fluent_templates::LanguageIdentifier;

use yew::{classes, html, Component, Context, ContextHandle, Html};

use super::locale::LocaleComponent;

pub struct HeaderComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,
}

impl Component for HeaderComponent {
    type Message = HeaderComponentMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(HeaderComponentMsg::MessageContextUpdated),
            )
            .expect("Missing LanguageIdentifier context.");
        Self {
            langid,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HeaderComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header>
                <div class={classes!("header-headline")}><LocaleComponent keyid="game-title"/></div>
                <div class={classes!("header-sub-headline")}><LocaleComponent keyid="game-subtitle"/></div>
            </header>
        }
    }
}

pub enum HeaderComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
}
