use std::rc::Rc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{Game, PlayType};

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
        let this_player = self
            .game
            .players
            .iter()
            .find(|player| player.id == self.game.this_player_id)
            .unwrap();
        let player_name = this_player.name.to_string();
        let is_watcher = matches!(this_player.play_type, PlayType::Watcher);

        let count_of_questions = self
            .game
            .configuration
            .count_of_questions
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "\u{221E}".to_string());

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
                    {" | 0 / "}
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
