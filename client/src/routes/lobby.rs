use yew::{html, Component, Context, Html};

use crate::components::locale::LocaleComponent;

pub struct LobbyComponent;

impl Component for LobbyComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <aside>
                    <LocaleComponent keyid="join-game-header-string-1"/>
                    {" "}
                    <span style="font-weight: bold;">{"tkprog.de/onto"}</span>
                    {" "}
                    <LocaleComponent keyid="join-game-header-string-2"/>
                    {" "}
                    <span style="font-weight: bold;">{"YWQC"}</span>
                    {" "}
                    <LocaleComponent keyid="join-game-header-string-3"/>
                </aside>
            </main>
        }
    }
}
