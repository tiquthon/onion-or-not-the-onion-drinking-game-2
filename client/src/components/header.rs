use fluent_templates::LanguageIdentifier;

use yew::{classes, function_component, html, use_context, Html};

use crate::components::locale::LocaleComponent;

#[function_component(HeaderComponent)]
pub fn header_component() -> Html {
    let _langid = use_context::<LanguageIdentifier>().expect("Missing LanguageIdentifier context.");

    html! {
        <header class={classes!("header")}>
            <h1 class={classes!("header__title")}><LocaleComponent keyid="game-title"/></h1>
            <h2 class={classes!("header__subtitle")}><LocaleComponent keyid="game-subtitle"/></h2>
        </header>
    }
}
