use std::rc::Rc;

use fluent_templates::LanguageIdentifier;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, GameState};

use yew::{classes, function_component, html, use_context, Callback, Html};

use crate::components::locale::LocaleComponent;

#[function_component(PlayerNameTypeExitHeadlineComponent)]
pub fn player_name_type_exit_headline_component(
    props: &PlayerNameTypeExitHeadlineComponentProps,
) -> Html {
    let _langid: LanguageIdentifier = use_context().expect("Missing LanguageIdentifier context.");
    let game: Rc<Game> = use_context().expect("Missing Game context.");

    let this_player = game.get_this_player().unwrap();
    let player_name = this_player.name.to_string();
    let is_watcher = this_player.is_watcher();

    let count_of_questions = game.configuration.count_of_questions.to_string();

    let number_of_current_question = match &game.game_state {
        GameState::InLobby => "0".to_string(),
        GameState::Playing {
            index_of_current_question,
            ..
        } => (index_of_current_question + 1).to_string(),
        GameState::Aftermath { .. } => count_of_questions.to_string(),
    };

    let cloned_on_exit_game_wished = props.on_exit_game_wished.clone();
    let onclick_exit_game = Callback::from(move |_| cloned_on_exit_game_wished.emit(()));

    html! {
        <>
            <h3 class={classes!("player-name-headline")}>{player_name}</h3>
            <p class={classes!("player-type-and-exit")}>
                <span class={classes!("player-type-and-exit__player-type")}>{
                    if is_watcher {
                        html!{ <LocaleComponent keyid="play-view-type-of-player-watcher"/> }
                    } else {
                        html!{ <LocaleComponent keyid="play-view-type-of-player-player"/> }
                    }
                }</span>
                {" | "}
                {number_of_current_question}
                {" / "}
                {count_of_questions}
                {" | "}
                <button type="button" class={classes!("button", "player-type-and-exit__exit-game-link")} onclick={onclick_exit_game}>
                    <LocaleComponent keyid="play-view-exit-the-game"/>
                </button>
            </p>
        </>
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct PlayerNameTypeExitHeadlineComponentProps {
    pub on_exit_game_wished: Callback<()>,
}
