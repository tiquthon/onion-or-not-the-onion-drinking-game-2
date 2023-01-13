use fluent_templates::LanguageIdentifier;

use yew::{html, Component, Context, ContextHandle, Html};

use super::locale::{locale_args, Locale};

pub struct Header {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,
}

impl Component for Header {
    type Message = HeaderMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(ctx.link().callback(HeaderMsg::MessageContextUpdated))
            .expect("Missing LanguageIdentifier context.");
        Self {
            langid,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HeaderMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header>
                <div><Locale keyid="game-title"/></div>
                <div><Locale keyid="game-subtitle"/></div>
                <div><Locale keyid="hello" args={locale_args([("name", "Thimo".into())])}/></div>
            </header>
        }
    }
}

pub enum HeaderMsg {
    MessageContextUpdated(LanguageIdentifier),
}
