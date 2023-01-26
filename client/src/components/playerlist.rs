use onion_or_not_the_onion_drinking_game_2_shared_library::model::game::{
    PlayType, Player, PlayerId,
};

use yew::{classes, html, Component, Context, Html};

use crate::components::locale::{locale_args, LocaleComponent};

pub struct PlayerListComponent;

impl Component for PlayerListComponent {
    type Message = ();
    type Properties = PlayerListComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h3 class={classes!("playerlist-headline")}>
                    <LocaleComponent keyid="play-view-players-headline"/>
                </h3>
                {
                    if ctx.props().players.is_empty() {
                        html! {
                            <p class={classes!("playerlist-listing-or-paragraph")}>
                                <LocaleComponent keyid="play-view-players-no-one-here"/>
                            </p>
                        }
                    } else {
                        html! {
                            <ul class={classes!("playerlist-listing-or-paragraph")}>
                                {
                                    ctx.props()
                                        .players
                                        .iter()
                                        .map(|player: &Player| {
                                            let is_this_player = player.id == ctx.props().this_player_id;
                                            html! {
                                                <li>
                                                    <span class={classes!(
                                                        "playerlist-span-username",
                                                        is_this_player.then_some("span-actual-user-is-username")
                                                    )}>
                                                        {player.name.to_string()}
                                                    </span>
                                                    {"Wants to skip Aftermath"}
                                                    {"Has Answered"}
                                                    {"-> AFTERMATH & Answered correct incorrect"}
                                                    <span>
                                                        {
                                                            match &player.play_type {
                                                                PlayType::Player { points } => {
                                                                    html! {
                                                                        <LocaleComponent
                                                                            keyid="play-view-players-points"
                                                                            args={locale_args([("points", points.into())])}/>
                                                                    }
                                                                }
                                                                PlayType::Watcher => {
                                                                    html! {
                                                                        <>
                                                                            {" ("}
                                                                            <LocaleComponent keyid="play-view-players-is-watching"/>
                                                                            {")"}
                                                                        </>
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    </span>
                                                </li>
                                            }
                                        })
                                        .collect::<Html>()
                                }
                            </ul>
                        }
                    }
                }
            </>
        }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct PlayerListComponentProps {
    pub players: Vec<Player>,
    pub this_player_id: PlayerId,
}
