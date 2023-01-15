use yew::{html, Html};

use super::routes::index::IndexComponent;
use super::routes::play::PlayComponent;

pub mod index;
pub mod play;

#[derive(Clone, yew_router::Routable, PartialEq)]
pub enum Route {
    #[at("/play")]
    Play,
    #[not_found]
    #[at("/")]
    Index,
}

pub fn route_switch(route: Route) -> Html {
    match route {
        Route::Play => html! { <PlayComponent/> },
        Route::Index => html! { <IndexComponent/> },
    }
}
