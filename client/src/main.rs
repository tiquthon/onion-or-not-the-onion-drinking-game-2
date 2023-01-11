use std::collections::HashMap;

use fluent_templates::{LanguageIdentifier, Loader};

use unic_langid::langid;

use yew::prelude::*;

const US_ENGLISH: LanguageIdentifier = langid!("en-US");
#[allow(dead_code)]
const GERMAN: LanguageIdentifier = langid!("de");

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        core_locales: "./locales/core.ftl",
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}

struct App {
    locale: LanguageIdentifier,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { locale: US_ENGLISH }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ChangeLocale(lid) => self.locale = lid,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let m = LOCALES.lookup(&self.locale, "hello-world");
        let n = LOCALES.lookup_with_args(
            &self.locale,
            "greeting",
            &HashMap::from([("name".to_string(), "Alice".into())]),
        );
        let locales: Vec<Html> = LOCALES.locales()
            .cloned()
            .map(|lid: LanguageIdentifier| (
                format!(
                    "{}{}",
                    lid.language,
                    lid.region
                        .as_ref()
                        .map(Into::into)
                        .map(|region: &str| format!("-{region}"))
                        .unwrap_or_else(String::new)
                ),
                lid
            ))
            .map(|(locale_string, lid): (String, LanguageIdentifier)| {
                html! {
                    <li>
                        <button type="button" onclick={ctx.link().callback(move |_| AppMsg::ChangeLocale(lid.clone()))}>{locale_string}</button>
                    </li>
                }
            })
            .collect();
        html! {
            <main>
                <ul>
                    {locales}
                </ul>
                <p>{m}</p>
                <p>{n}</p>
            </main>
        }
    }
}

enum AppMsg {
    ChangeLocale(LanguageIdentifier),
}
