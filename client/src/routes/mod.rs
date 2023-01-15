use yew::{html, Html};

use super::routes::game::GameComponent;
use super::routes::index::IndexComponent;
use super::routes::lobby::LobbyComponent;

pub mod game;
pub mod index;
pub mod lobby;

#[derive(Clone, yew_router::Routable, PartialEq)]
pub enum Route {
    #[at("/game")]
    Game,
    #[at("/lobby")]
    Lobby,
    #[not_found]
    #[at("/")]
    Index,
}

pub fn route_switch(route: Route) -> Html {
    match route {
        Route::Game => html! { <GameComponent/> },
        Route::Lobby => html! { <LobbyComponent/> },
        Route::Index => html! { <IndexComponent/> },
    }
}
