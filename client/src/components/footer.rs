use fluent_templates::{LanguageIdentifier, Loader};

use yew::{classes, function_component, html, use_context, Callback, Html};

use crate::components::locale::{locale, LOCALES};

#[function_component(FooterComponent)]
pub fn footer_component(props: &FooterProps) -> Html {
    let langid = use_context::<LanguageIdentifier>().expect("Missing LanguageIdentifier context.");

    let locale_change_buttons: Vec<Html> = LOCALES
        .locales()
        .map(|language_identifier| {
            let on_change_language_identifier = props.on_change_language_identifier.clone();
            let onclick = Callback::from(move |_| {
                on_change_language_identifier.emit(language_identifier.clone());
            });
            let is_selected = langid == *language_identifier;
            html! {
                <button type="button" class={classes!("button", "locale-selection__button")} disabled={is_selected} {onclick}>
                    {locale("language-name", language_identifier)}
                </button>
            }
        })
        .collect();

    html! {
        <footer class={classes!("footer")}>
            <nav class={classes!("locales-selection", "footer__locales-selection")}>
                {locale_change_buttons}
            </nav>
            <span class={classes!("footer__copyright")}>{"\u{00a9} 2023 Thimo \"Tiquthon\" Neumann"}</span>
        </footer>
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub on_change_language_identifier: Callback<LanguageIdentifier>,
}
