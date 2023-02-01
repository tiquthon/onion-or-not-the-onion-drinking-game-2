use std::rc::Rc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, GameState};

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html};

use crate::components::locale::LocaleComponent;

pub struct PlayerNameTypeExitHeadlineComponent {
    game: Rc<Game>,
    _context_listener: ContextHandle<Rc<Game>>,
}

impl Component for PlayerNameTypeExitHeadlineComponent {
    type Message = PlayerNameTypeExitHeadlineComponentMsg;
    type Properties = PlayerNameTypeExitHeadlineComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (game, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(PlayerNameTypeExitHeadlineComponentMsg::MessageContextUpdated),
            )
            .expect("Missing Game context.");

        Self {
            game,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlayerNameTypeExitHeadlineComponentMsg::MessageContextUpdated(game) => {
                self.game = game;
                true
            }
            PlayerNameTypeExitHeadlineComponentMsg::ExitGame => {
                ctx.props().on_exit_game_wished.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let this_player = self.game.get_this_player().unwrap();
        let player_name = this_player.name.to_string();
        let is_watcher = this_player.is_watcher();

        let count_of_questions = self.game.configuration.count_of_questions.to_string();

        let number_of_current_question = match &self.game.game_state {
            GameState::InLobby => "0".to_string(),
            GameState::Playing {
                index_of_current_question,
                ..
            } => (index_of_current_question + 1).to_string(),
            GameState::Aftermath { .. } => count_of_questions.to_string(),
        };

        let onclick_exit_game = ctx
            .link()
            .callback(|_| PlayerNameTypeExitHeadlineComponentMsg::ExitGame);

        html! {
            <>
                <h2 class={classes!("player-name-headline")}>{player_name}</h2>
                <p class={classes!("player-type-and-exit")}>
                    <span class={classes!("player-type")}>{
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
                    <button type="button" class={classes!("exit-game-link")} onclick={onclick_exit_game}>
                        <LocaleComponent keyid="play-view-exit-the-game"/>
                    </button>
                </p>
            </>
        }
    }
}

pub enum PlayerNameTypeExitHeadlineComponentMsg {
    MessageContextUpdated(Rc<Game>),
    ExitGame,
}

#[derive(yew::Properties, PartialEq)]
pub struct PlayerNameTypeExitHeadlineComponentProps {
    pub on_exit_game_wished: Callback<()>,
}
