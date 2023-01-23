use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, PlayType};

use yew::{classes, html, Component, Context, Html};

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
        let this_player = ctx
            .props()
            .game
            .players
            .iter()
            .find(|player| player.id == ctx.props().game.this_player_id)
            .unwrap();
        let player_name = this_player.name.to_string();
        let is_watcher = matches!(this_player.play_type, PlayType::Watcher);

        let invite_code = ctx.props().game.invite_code.to_string();

        let count_of_questions = ctx
            .props()
            .game
            .configuration
            .count_of_questions
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "\u{221E}".to_string());

        html! {
            <main class={classes!("play-main")}>
                <JoinGameComponent {invite_code} />
                <h2>{player_name}</h2>
                <p>
                    <span>{
                        if is_watcher {
                            html!{ <LocaleComponent keyid="game-view-type-of-player-watcher"/> }
                        } else {
                            html!{ <LocaleComponent keyid="game-view-type-of-player-player"/> }
                        }
                    }</span>
                    {" | 0 / "}
                    {count_of_questions}
                    {" | "}
                    <button type="button"><LocaleComponent keyid="game-view-exit-the-game"/></button>
                </p>
            </main>
        }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct LobbyComponentProps {
    pub game: Game,
}
