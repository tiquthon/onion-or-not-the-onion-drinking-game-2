use yew::{html, Html};

use super::routes::game::GameComponent;
use super::routes::index::IndexComponent;

pub mod game;
pub mod index;

#[derive(Clone, yew_router::Routable, PartialEq)]
pub enum Route {
    #[at("/game")]
    Game,
    #[not_found]
    #[at("/")]
    Index,
}

pub fn route_switch(route: Route) -> Html {
    match route {
        Route::Index => html! { <IndexComponent/> },
        Route::Game => html! { <GameComponent/> },
    }
}
