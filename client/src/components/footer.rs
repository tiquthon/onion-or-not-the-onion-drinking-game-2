use fluent_templates::{LanguageIdentifier, Loader};

use yew::{html, Callback, Component, Context, Html};

use super::locale::LOCALES;

pub struct Footer;

impl Component for Footer {
    type Message = FooterMsg;
    type Properties = FooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FooterMsg::ChangeLanguageIdentifier(lid) => {
                ctx.props().on_change_language_identifier.emit(lid);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let locale_change_buttons: Vec<Html> = locales_to_locale_string_tuple()
            .map(|(lid, lid_str)| {
                let onclick = ctx
                    .link()
                    .callback(move |_| FooterMsg::ChangeLanguageIdentifier(lid.clone()));
                html! {
                    <button type="button" {onclick}>{lid_str}</button>
                }
            })
            .collect();
        html! {
            <footer>
                <nav>
                    {locale_change_buttons}
                </nav>
                <p>{"\u{00a9} 2023 Thimo \"Tiquthon\" Neumann"}</p>
            </footer>
        }
    }
}

pub enum FooterMsg {
    ChangeLanguageIdentifier(LanguageIdentifier),
}

#[derive(yew::Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub on_change_language_identifier: Callback<LanguageIdentifier>,
}

fn locales_to_locale_string_tuple() -> impl Iterator<Item = (LanguageIdentifier, String)> {
    LOCALES.locales().map(|lid| (lid.clone(), lid.to_string()))
}
