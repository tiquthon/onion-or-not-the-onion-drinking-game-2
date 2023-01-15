use yew::{html, Component, Context, Html};

use crate::components::join_game::JoinGameComponent;
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
                <JoinGameComponent />
                <h2>{"PLAYER NAME"}</h2>
                <p>
                    <span>{
                        if true {
                            html!{ <LocaleComponent keyid="game-view-type-of-player-watcher"/> }
                        } else {
                            html!{ <LocaleComponent keyid="game-view-type-of-player-player"/> }
                        }
                    }</span>
                    {" | "}
                    {"3"}{" / "}{"12"}
                    {" | "}
                    <a href=""><LocaleComponent keyid="game-view-exit-the-game"/></a>
                </p>
            </main>
        }
    }
}
