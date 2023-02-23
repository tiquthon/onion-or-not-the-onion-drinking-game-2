use fluent_templates::{LanguageIdentifier, Loader};

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html};

use super::locale::{locale, LOCALES};

pub struct FooterComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,
}

impl Component for FooterComponent {
    type Message = FooterComponentMsg;
    type Properties = FooterProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(FooterComponentMsg::MessageContextUpdated),
            )
            .expect("Missing LanguageIdentifier context.");
        Self {
            langid,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FooterComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
            FooterComponentMsg::ChangeLanguageIdentifier(lid) => {
                ctx.props().on_change_language_identifier.emit(lid);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let locale_change_buttons: Vec<Html> = LOCALES
            .locales()
            .map(|language_identifier| {
                let onclick = ctx.link().callback(move |_| {
                    FooterComponentMsg::ChangeLanguageIdentifier(language_identifier.clone())
                });
                let is_selected = self.langid == *language_identifier;
                html! {
                    <button type="button" disabled={is_selected} {onclick}>
                        {locale("language-name", language_identifier)}
                    </button>
                }
            })
            .collect();
        html! {
            <footer>
                <nav class={classes!("footer-links")}>
                    {locale_change_buttons}
                </nav>
                <p class={classes!("footer-copyright")}>{"\u{00a9} 2023 Thimo \"Tiquthon\" Neumann"}</p>
            </footer>
        }
    }
}

pub enum FooterComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
    ChangeLanguageIdentifier(LanguageIdentifier),
}

#[derive(yew::Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub on_change_language_identifier: Callback<LanguageIdentifier>,
}
