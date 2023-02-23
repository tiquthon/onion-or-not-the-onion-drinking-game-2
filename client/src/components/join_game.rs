use crate::utils::{retrieve_browser_location, ReplaceProtocol};
use yew::{classes, html, Component, Context, Html};

use super::locale::LocaleComponent;

pub struct JoinGameComponent;

impl Component for JoinGameComponent {
    type Message = ();
    type Properties = JoinGameComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <aside class={classes!("join-panel")}>
                <LocaleComponent keyid="join-game-header-string-1"/>
                {" "}
                <span style="font-weight: bold;">{get_view_target_url_string()}</span>
                {" "}
                <LocaleComponent keyid="join-game-header-string-2"/>
                {" "}
                <span style="font-weight: bold;">{ctx.props().invite_code.clone()}</span>
                {" "}
                <LocaleComponent keyid="join-game-header-string-3"/>
            </aside>
        }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct JoinGameComponentProps {
    pub invite_code: String,
}

fn get_view_target_url_string() -> String {
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
