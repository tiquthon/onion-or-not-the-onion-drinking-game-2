use yew::{classes, html, AttrValue, Callback, Classes, Component, Context, Html};

pub struct MessagesComponent;

impl Component for MessagesComponent {
    type Message = MessagesComponentMsg;
    type Properties = MessagesComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MessagesComponentMsg::OnClickClose(message) => {
                ctx.props().on_message_closed.emit(message);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let message_sections: Vec<Html> = ctx.props()
            .messages
            .iter()
            .map(|message| {
                let class = match message.level {
                    MessageLevel::Success => "msg-box-success",
                    MessageLevel::Error => "msg-box-error",
                    MessageLevel::Warn => "msg-box-warn",
                    MessageLevel::Info => "msg-box-info",
                };
                let cloned_message = message.clone();
                let onclick_close = ctx
                    .link()
                    .callback(move |_| MessagesComponentMsg::OnClickClose(cloned_message.clone()));
                html! {
                    <section class={classes!(class, ctx.props().class.clone())}>
                        <span>{message.text.clone()}</span>
                        {
                            match message.closable {
                                ClosingCapability::Closable => html! {
                                    <span class="msg-box-close" onclick={onclick_close}>{"\u{1F5D9}"}</span>
                                },
                                ClosingCapability::NonClosable => html! {},
                            }
                        }
                    </section>
                }
            })
            .collect();
        html! {
            <>{message_sections}</>
        }
    }
}

pub enum MessagesComponentMsg {
    OnClickClose(Message),
}

#[derive(yew::Properties, PartialEq)]
pub struct MessagesComponentProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub messages: Vec<Message>,
    #[prop_or_default]
    pub on_message_closed: Callback<Message>,
}

#[derive(yew::Properties, Clone, PartialEq, Eq, Hash)]
pub struct Message {
    pub text: AttrValue,
    #[prop_or(MessageLevel::Info)]
    pub level: MessageLevel,
    #[prop_or(ClosingCapability::Closable)]
    pub closable: ClosingCapability,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum MessageLevel {
    Success,
    Error,
    Warn,
    Info,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ClosingCapability {
    Closable,
    NonClosable,
}
