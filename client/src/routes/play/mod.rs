use fluent_templates::LanguageIdentifier;

use onion_or_not_the_onion_drinking_game_2_shared_library::Game;

use yew::{html, Component, Context, ContextHandle, ContextProvider, Html};

pub mod aftermath;
pub mod connecting;
pub mod game;
pub mod lobby;

pub struct PlayComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,
    game: Game,
}

impl Component for PlayComponent {
    type Message = PlayComponentMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(ctx.link().callback(PlayComponentMsg::MessageContextUpdated))
            .expect("Missing LanguageIdentifier context.");
        Self {
            langid,
            _context_listener: context_listener,
            game: Game::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlayComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <ContextProvider<Option<Game>> context={self.game}>
                    {"Play"}
                </ContextProvider<Option<Game>>>
            </main>
        }
    }
}

pub enum PlayComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
}
