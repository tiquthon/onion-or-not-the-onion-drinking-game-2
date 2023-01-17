use std::str::FromStr;

use fluent_templates::{LanguageIdentifier, Loader};

use unic_langid::langid;

use yew::{html, Component, Context, ContextProvider, Html};

use yew_router::{BrowserRouter, Switch};

use crate::components::footer::FooterComponent;
use crate::components::header::HeaderComponent;
use crate::components::locale::LOCALES;

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
        let langid: LanguageIdentifier = web_sys::window()
            .unwrap()
            .navigator()
            .languages()
            .iter()
            .filter_map(|language: wasm_bindgen::JsValue| {
                language
                    .as_string()
                    .and_then(|language_string| LanguageIdentifier::from_str(&language_string).ok())
            })
            .find(|language_identifier: &LanguageIdentifier| {
                LOCALES
                    .locales()
                    .any(|langid: &LanguageIdentifier| *langid == *language_identifier)
            })
            .unwrap_or(langid!("en-US"));
        Self { langid }
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
