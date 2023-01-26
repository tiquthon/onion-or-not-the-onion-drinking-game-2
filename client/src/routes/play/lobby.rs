use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, PlayType};

use yew::{classes, html, Callback, Component, Context, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::LocaleComponent;
use crate::components::playerlist::PlayerListComponent;

pub struct LobbyComponent;

impl Component for LobbyComponent {
    type Message = LobbyComponentMsg;
    type Properties = LobbyComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LobbyComponentMsg::ExitGame => {
                ctx.props().on_exit_game_wish.emit(());
                false
            }
        }
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

        let onclick_exit_game = ctx.link().callback(|_| LobbyComponentMsg::ExitGame);

        html! {
            <main class={classes!("play-main")}>
                <JoinGameComponent {invite_code} />
                <section class={classes!("main-wrapper")}>
                    <h2 class={classes!("player-name-headline")}>{player_name}</h2>
                    <p class={classes!("player-type-and-exit")}>
                        <span class={classes!("player-type")}>{
                            if is_watcher {
                                html!{ <LocaleComponent keyid="play-view-type-of-player-watcher"/> }
                            } else {
                                html!{ <LocaleComponent keyid="play-view-type-of-player-player"/> }
                            }
                        }</span>
                        {" | 0 / "}
                        {count_of_questions}
                        {" | "}
                        <button type="button" class={classes!("exit-game-link")} onclick={onclick_exit_game}>
                            <LocaleComponent keyid="play-view-exit-the-game"/>
                        </button>
                    </p>
                    <h1 class={classes!("welcome-headline")}>
                        <LocaleComponent keyid="lobby-view-welcome-headline"/>
                    </h1>
                    {
                        if !is_watcher {
                            html! {
                                <div class={classes!("start-form")}>
                                    <button type="button" id="start-form-submit-button" class={classes!("start-form-submit-button")}>
                                        <LocaleComponent keyid="lobby-view-start-game-button"/>
                                    </button>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <PlayerListComponent
                        players={ctx.props().game.players.clone()}
                        this_player_id={ctx.props().game.this_player_id}
                        game_state={ctx.props().game.game_state.clone()}/>
                </section>
            </main>
        }
    }
}

pub enum LobbyComponentMsg {
    ExitGame,
}

#[derive(yew::Properties, PartialEq)]
pub struct LobbyComponentProps {
    pub game: Game,
    pub on_exit_game_wish: Callback<()>,
}
