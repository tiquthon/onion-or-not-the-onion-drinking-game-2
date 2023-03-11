pub extern crate yew;

use fluent_templates::LanguageIdentifier;

use unic_langid::langid;

use yew::{
    function_component, html, use_effect_with_deps, use_state_eq, Callback, ContextProvider, Html,
};

use crate::components::footer::FooterComponent;
use crate::components::header::HeaderComponent;
use crate::components::locale::{
    load_or_else_browser_select_language_identifier_and_log_warnings,
    store_language_identifier_to_persistent_storage_and_log_warnings,
};
use crate::routes::index::IndexComponent;
use crate::routes::play::{CreateJoinLobby, PlayComponent};

pub mod components;
pub mod routes;
pub mod utils;

#[function_component(AppComponent)]
pub fn app_component() -> Html {
    let langid = use_state_eq::<LanguageIdentifier, _>(|| langid!("en-US"));
    let state = use_state_eq(|| AppState::Index);

    let cloned_langid = langid.clone();
    use_effect_with_deps(
        move |_| {
            let optional_loaded_langid =
                load_or_else_browser_select_language_identifier_and_log_warnings();
            if let Some(loaded_langid) = optional_loaded_langid {
                cloned_langid.set(loaded_langid);
            }
        },
        (),
    );

    let inner_components = match &*state {
        AppState::Index => {
            let cloned_state = state.clone();
            let on_join_lobby = Callback::from(move |join_lobby| {
                cloned_state.set(AppState::Play {
                    create_join_lobby: CreateJoinLobby::Join(join_lobby),
                });
            });

            let cloned_state = state.clone();
            let on_create_lobby = Callback::from(move |create_lobby| {
                cloned_state.set(AppState::Play {
                    create_join_lobby: CreateJoinLobby::Create(create_lobby),
                });
            });

            html! {
                <IndexComponent {on_join_lobby} {on_create_lobby} />
            }
        }
        AppState::Play { create_join_lobby } => {
            let cloned_state = state.clone();
            let on_go_back_to_index = Callback::from(move |_| cloned_state.set(AppState::Index));

            html! {
                <PlayComponent create_join_lobby={create_join_lobby.clone()} {on_go_back_to_index} />
            }
        }
    };

    let cloned_langid = langid.clone();
    let on_change_language_identifier = Callback::from(move |lid: LanguageIdentifier| {
        store_language_identifier_to_persistent_storage_and_log_warnings(&lid);
        cloned_langid.set(lid);
    });

    html! {
        <ContextProvider<LanguageIdentifier> context={(*langid).clone()}>
            <HeaderComponent />
            { inner_components }
            <FooterComponent {on_change_language_identifier} />
        </ContextProvider<LanguageIdentifier>>
    }
}

#[derive(PartialEq)]
enum AppState {
    Index,
    Play { create_join_lobby: CreateJoinLobby },
}
