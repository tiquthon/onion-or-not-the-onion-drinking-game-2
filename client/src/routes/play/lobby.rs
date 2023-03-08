use std::rc::Rc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, PlayType};

use yew::{classes, function_component, html, use_context, Callback, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::locale::LocaleComponent;
use crate::components::player_name_type_exit_headline::PlayerNameTypeExitHeadlineComponent;
use crate::components::playerlist::PlayerListComponent;

#[function_component(LobbyComponent)]
pub fn lobby_component(props: &LobbyComponentProps) -> Html {
    let game: Rc<Game> = use_context().expect("Missing Game context.");

    let this_player = game
        .players
        .iter()
        .find(|player| player.id == game.this_player_id)
        .unwrap();
    let is_watcher = matches!(this_player.play_type, PlayType::Watcher);

    let invite_code = game.invite_code.to_string();

    let cloned_on_exit_game_wish = props.on_exit_game_wish.clone();
    let on_exit_game_wished = Callback::from(move |_| cloned_on_exit_game_wish.emit(()));

    let cloned_on_start_game = props.on_start_game.clone();
    let onclick_start_game = Callback::from(move |_| cloned_on_start_game.emit(()));

    html! {
        <main class={classes!("main")}>
            <JoinGameComponent {invite_code} />
            <section class={classes!("centered-primary-content", "play-primary-content")}>
                <PlayerNameTypeExitHeadlineComponent {on_exit_game_wished} />
                <h1 class={classes!("welcome-headline")}>
                    <LocaleComponent keyid="lobby-view-welcome-headline"/>
                </h1>
                if !is_watcher {
                    <button class={classes!("button", "button--width-full")} onclick={onclick_start_game} type="button">
                        <LocaleComponent keyid="lobby-view-start-game-button"/>
                    </button>
                }
                <PlayerListComponent class={classes!("play-primary-content__player-list")} />
            </section>
        </main>
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct LobbyComponentProps {
    pub on_exit_game_wish: Callback<()>,
    pub on_start_game: Callback<()>,
}
