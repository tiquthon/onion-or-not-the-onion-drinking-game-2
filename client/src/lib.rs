use fluent_templates::LanguageIdentifier;

use onion_or_not_the_onion_drinking_game_2_shared_library::Game;

use unic_langid::langid;

use yew::prelude::*;

use yew_router::prelude::*;

use crate::components::footer::FooterComponent;
use crate::components::header::HeaderComponent;

use crate::routes::{route_switch, Route};

pub mod components;
pub mod routes;

pub struct AppComponent {
    langid: LanguageIdentifier,
    game: Option<Game>,
}

impl Component for AppComponent {
    type Message = AppComponentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            langid: langid!("en-US"),
            game: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppComponentMsg::ChangeLanguageIdentifier(lid) => {
                self.langid = lid;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_language_identifier = ctx
            .link()
            .callback(AppComponentMsg::ChangeLanguageIdentifier);
        html! {
            <ContextProvider<LanguageIdentifier> context={self.langid.clone()}>
                <ContextProvider<Option<Game>> context={self.game}>
                    <HeaderComponent/>
                    <BrowserRouter>
                        <Switch<Route> render={route_switch}/>
                    </BrowserRouter>
                    <FooterComponent {on_change_language_identifier}/>
                </ContextProvider<Option<Game>>>
            </ContextProvider<LanguageIdentifier>>
        }
    }
}

pub enum AppComponentMsg {
    ChangeLanguageIdentifier(LanguageIdentifier),
}
