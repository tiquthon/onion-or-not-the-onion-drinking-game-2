use std::rc::Rc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::Game;

use yew::{classes, html, Callback, Component, Context, ContextHandle, Html};

use crate::components::join_game::JoinGameComponent;
use crate::components::player_name_type_exit_headline::PlayerNameTypeExitHeadlineComponent;

pub struct GameComponent {
    game: Rc<Game>,
    _context_listener: ContextHandle<Rc<Game>>,
}

impl Component for GameComponent {
    type Message = GameComponentMsg;
    type Properties = GameComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (game, context_listener) = ctx
            .link()
            .context(ctx.link().callback(GameComponentMsg::MessageContextUpdated))
            .expect("Missing Game context.");

        Self {
            game,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameComponentMsg::MessageContextUpdated(game) => {
                self.game = game;
                true
            }
            GameComponentMsg::ExitGame => {
                ctx.props().on_exit_game_wish.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let invite_code = self.game.invite_code.to_string();

        let on_exit_game_wished = ctx.link().callback(|_| GameComponentMsg::ExitGame);

        html! {
            <main class={classes!("play-main")}>
                <JoinGameComponent {invite_code} />
                <section class={classes!("main-wrapper")}>
                    <PlayerNameTypeExitHeadlineComponent {on_exit_game_wished} />
                </section>
            </main>
        }
    }
}

pub enum GameComponentMsg {
    MessageContextUpdated(Rc<Game>),
    ExitGame,
}

#[derive(yew::Properties, PartialEq)]
pub struct GameComponentProps {
    pub on_exit_game_wish: Callback<()>,
}
