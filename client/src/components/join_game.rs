use yew::{html, Component, Context, Html};

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
            <aside>
                <LocaleComponent keyid="join-game-header-string-1"/>
                {" "}
                <span style="font-weight: bold;">{"tkprog.de/onto"}</span>
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
