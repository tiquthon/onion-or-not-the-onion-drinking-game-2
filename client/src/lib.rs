use fluent_templates::LanguageIdentifier;

use unic_langid::langid;

use yew::{html, Component, Context, ContextProvider, Html};

use yew_router::{BrowserRouter, Switch};

use crate::components::footer::FooterComponent;
use crate::components::header::HeaderComponent;
use crate::components::locale::{
    load_or_else_browser_select_language_identifier_and_log_warnings,
    store_language_identifier_to_persistent_storage_and_log_warnings,
};

use crate::routes::{route_switch, Route};

pub mod components;
pub mod routes;

pub struct AppComponent {
    langid: LanguageIdentifier,
}

impl Component for AppComponent {
    type Message = AppComponentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            langid: load_or_else_browser_select_language_identifier_and_log_warnings()
                .unwrap_or(langid!("en-US")),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppComponentMsg::ChangeLanguageIdentifier(lid) => {
                store_language_identifier_to_persistent_storage_and_log_warnings(&lid);
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
                <HeaderComponent/>
                <BrowserRouter>
                    <Switch<Route> render={route_switch}/>
                </BrowserRouter>
                <FooterComponent {on_change_language_identifier}/>
            </ContextProvider<LanguageIdentifier>>
        }
    }
}

pub enum AppComponentMsg {
    ChangeLanguageIdentifier(LanguageIdentifier),
}
