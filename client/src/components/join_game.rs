use yew::{classes, function_component, html, use_effect_with_deps, use_state_eq, Html};

use crate::components::locale::LocaleComponent;
use crate::utils::{retrieve_browser_location, ReplaceProtocol};

#[function_component(JoinGameComponent)]
pub fn join_game_component(props: &JoinGameComponentProps) -> Html {
    let optional_url_for_view = use_state_eq::<Option<String>, _>(|| None);

    let cloned_optional_url_for_view = optional_url_for_view.clone();
    use_effect_with_deps(
        move |_| {
            let url_for_view = retrieve_browser_location_without_protocol();
            cloned_optional_url_for_view.set(Some(url_for_view));
        },
        (),
    );

    let inner_html = match &*optional_url_for_view {
        Some(url_for_view) => {
            html! {
                <>
                    <LocaleComponent keyid="join-game-header-string-1"/>
                    {" "}
                    <span class={classes!("join-panel__invite-url")}>{url_for_view.clone()}</span>
                    {" "}
                    <LocaleComponent keyid="join-game-header-string-2"/>
                    {" "}
                    <span class={classes!("join-panel__invite-code")}>{props.invite_code.clone()}</span>
                    {" "}
                    <LocaleComponent keyid="join-game-header-string-3"/>
                </>
            }
        }
        None => {
            html! {
                <>
                    <LocaleComponent keyid="join-game-header-missing-url-string-1"/>
                    {" "}
                    <span class={classes!("join-panel__invite-code")}>{props.invite_code.clone()}</span>
                    {" "}
                    <LocaleComponent keyid="join-game-header-missing-url-string-2"/>
                </>
            }
        }
    };

    html! {
        <aside class={classes!("join-panel")}>
            { inner_html }
        </aside>
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct JoinGameComponentProps {
    pub invite_code: String,
}

fn retrieve_browser_location_without_protocol() -> String {
    retrieve_browser_location(
        Some(ReplaceProtocol {
            secure: "remove:",
            unsecure: "remove:",
        }),
        None,
    )
    .strip_prefix("remove://")
    .unwrap()
    .to_owned()
}
