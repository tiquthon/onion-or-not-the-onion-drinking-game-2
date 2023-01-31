use yew::{classes, html, Callback, Component, Context, Html};

use crate::components::locale::LocaleComponent;

pub struct ConnectingComponent;

impl Component for ConnectingComponent {
    type Message = ConnectingComponentMsg;
    type Properties = ConnectingComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ConnectingComponentMsg::GoBack => {
                ctx.props().on_go_back.emit(());
                false
            }
            ConnectingComponentMsg::Cancel => {
                ctx.props().on_cancel.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().state {
            ConnectingComponentState::Connecting => {
                let cancel_button_onclick = ctx.link().callback(|_| ConnectingComponentMsg::Cancel);
                html! {
                    <main class={classes!("connecting-main")}>
                        <span class={classes!("connecting-text")}>
                            <LocaleComponent keyid="connecting-view-connecting-string" />
                        </span>
                        <button type="button" onclick={cancel_button_onclick}>
                            <LocaleComponent keyid="cancel-button-text" />
                        </button>
                    </main>
                }
            }
            ConnectingComponentState::Failed { error } => {
                let go_back_button_onclick =
                    ctx.link().callback(|_| ConnectingComponentMsg::GoBack);
                html! {
                    <main class={classes!("connecting-main")}>
                        <span class={classes!("connecting-text")}>
                            <LocaleComponent keyid="connecting-view-error-occurred-string" />
                        </span>
                        <span class={classes!("connecting-sub-text")}>
                            {"("}{ error.clone() }{")"}
                        </span>
                        <button type="button" onclick={go_back_button_onclick}>
                            <LocaleComponent keyid="go-back-button-text" />
                        </button>
                    </main>
                }
            }
        }
    }
}

pub enum ConnectingComponentMsg {
    GoBack,
    Cancel,
}

#[derive(yew::Properties, PartialEq)]
pub struct ConnectingComponentProps {
    pub state: ConnectingComponentState,
    #[prop_or_default]
    pub on_go_back: Callback<()>,
    #[prop_or_default]
    pub on_cancel: Callback<()>,
}

#[derive(PartialEq)]
pub enum ConnectingComponentState {
    Connecting,
    Failed { error: String },
}
