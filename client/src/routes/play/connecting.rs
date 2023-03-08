use fluent_templates::LanguageIdentifier;

use yew::{classes, function_component, html, use_context, Callback, Html};

use crate::components::locale::LocaleComponent;

#[function_component(ConnectingComponent)]
pub fn connecting_component(props: &ConnectingComponentProps) -> Html {
    let _langid = use_context::<LanguageIdentifier>().expect("Missing LanguageIdentifier context.");

    match &props.state {
        ConnectingComponentState::Connecting => {
            let cloned_on_cancel = props.on_cancel.clone();
            let cancel_button_onclick = Callback::from(move |_| cloned_on_cancel.emit(()));
            html! {
                <main class={classes!("main", "main--padding-normal", "connecting-view")}>
                    <p class={classes!("connecting-view__main-text")}>
                        <LocaleComponent keyid="connecting-view-connecting-string" />
                    </p>
                    <button class={classes!("button", "connecting-view__button")}
                        onclick={cancel_button_onclick} type="button">
                        <LocaleComponent keyid="cancel-button-text" />
                    </button>
                </main>
            }
        }
        ConnectingComponentState::Failed {
            locale_key_id,
            error,
        } => {
            let cloned_on_go_back = props.on_go_back.clone();
            let go_back_button_onclick = Callback::from(move |_| cloned_on_go_back.emit(()));
            html! {
                <main class={classes!("main", "connecting-view")}>
                    <p class={classes!("connecting-view__main-text")}>
                        <LocaleComponent keyid={locale_key_id.clone()} />
                    </p>
                    if let Some(error) = error.clone() {
                        <p class={classes!("connecting-view__sub-text")}>
                            {"("}{error}{")"}
                        </p>
                    }
                    <button class={classes!("button", "connecting-view__button")}
                        onclick={go_back_button_onclick} type="button">
                        <LocaleComponent keyid="go-back-button-text" />
                    </button>
                </main>
            }
        }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct ConnectingComponentProps {
    pub state: ConnectingComponentState,
    #[prop_or_default]
    pub on_go_back: Callback<()>,
    #[prop_or_default]
    pub on_cancel: Callback<()>,
}

#[derive(PartialEq)]
pub enum ConnectingComponentState {
    Connecting,
    Failed {
        locale_key_id: String,
        error: Option<String>,
    },
}
