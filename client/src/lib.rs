use fluent_templates::LanguageIdentifier;

use unic_langid::langid;

use yew::{html, Component, Context, ContextProvider, Html};

use crate::components::footer::FooterComponent;
use crate::components::header::HeaderComponent;
use crate::components::locale::{
    load_or_else_browser_select_language_identifier_and_log_warnings,
    store_language_identifier_to_persistent_storage_and_log_warnings,
};

use crate::routes::index::IndexComponent;
use crate::routes::play::CreateJoinLobby;
use crate::routes::play::PlayComponent;

pub mod components;
pub mod routes;

pub struct AppComponent {
    langid: LanguageIdentifier,
    state: AppState,
}

impl Component for AppComponent {
    type Message = AppComponentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            langid: load_or_else_browser_select_language_identifier_and_log_warnings()
                .unwrap_or(langid!("en-US")),
            state: AppState::Index,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppComponentMsg::ChangeLanguageIdentifier(lid) => {
                store_language_identifier_to_persistent_storage_and_log_warnings(&lid);
                self.langid = lid;
                true
            }
            AppComponentMsg::JoinLobby(join_lobby) => {
                self.state = AppState::Play {
                    create_join_lobby: CreateJoinLobby::Join(join_lobby),
                };
                true
            }
            AppComponentMsg::CreateLobby(create_lobby) => {
                self.state = AppState::Play {
                    create_join_lobby: CreateJoinLobby::Create(create_lobby),
                };
                true
            }
            AppComponentMsg::NavigateToIndex => {
                self.state = AppState::Index;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_language_identifier = ctx
            .link()
            .callback(AppComponentMsg::ChangeLanguageIdentifier);
        html! {
            <ContextProvider<LanguageIdentifier> context={self.langid.clone()}>
                <HeaderComponent/>
                {
                    match &self.state {
                        AppState::Index => {
                            let on_join_lobby = ctx.link().callback(AppComponentMsg::JoinLobby);
                            let on_create_lobby = ctx.link().callback(AppComponentMsg::CreateLobby);
                            html! {
                                <IndexComponent {on_join_lobby} {on_create_lobby}/>
                            }
                        },
                        AppState::Play { create_join_lobby } => {
                            let on_exit_game = ctx.link().callback(|_| AppComponentMsg::NavigateToIndex);
                            html! {
                                <PlayComponent create_join_lobby={create_join_lobby.clone()} {on_exit_game}/>
                            }
                        },
                    }
                }
                <FooterComponent {on_change_language_identifier}/>
            </ContextProvider<LanguageIdentifier>>
        }
    }
}

pub enum AppComponentMsg {
    ChangeLanguageIdentifier(LanguageIdentifier),

    JoinLobby(routes::index::JoinLobby),
    CreateLobby(routes::index::CreateLobby),

    NavigateToIndex,
}

enum AppState {
    Index,
    Play { create_join_lobby: CreateJoinLobby },
}
