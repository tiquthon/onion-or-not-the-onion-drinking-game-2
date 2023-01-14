use fluent_templates::LanguageIdentifier;

use yew::{html, Component, Context, ContextHandle, Html};

pub struct GameComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,
}

impl Component for GameComponent {
    type Message = GameComponentMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(ctx.link().callback(GameComponentMsg::MessageContextUpdated))
            .expect("Missing LanguageIdentifier context.");
        Self {
            langid,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                {"Game"}
            </main>
        }
    }
}

pub enum GameComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
}
