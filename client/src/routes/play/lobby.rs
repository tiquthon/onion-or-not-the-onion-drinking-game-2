use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::Game;

use yew::{html, Component, Context, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::LocaleComponent;

pub struct LobbyComponent;

impl Component for LobbyComponent {
    type Message = ();
    type Properties = LobbyComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let player_name = ctx
            .props()
            .game
            .players
            .iter()
            .find(|player| player.id == ctx.props().game.this_player_id)
            .unwrap()
            .name
            .to_string();
        let invite_code = ctx.props().game.invite_code.to_string();
        html! {
            <main>
                <JoinGameComponent {invite_code} />
                <h2>{player_name}</h2>
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

#[derive(yew::Properties, PartialEq)]
pub struct LobbyComponentProps {
    pub game: Game,
}
