use fluent_templates::LanguageIdentifier;

use yew::{html, Component, Context, Html};

use super::locale::{locale_args, Locale};

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = HeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header>
                <div><Locale keyid="game-title" langid={ctx.props().langid.clone()}/></div>
                <div><Locale keyid="game-subtitle" langid={ctx.props().langid.clone()}/></div>
                <div><Locale keyid="hello" langid={ctx.props().langid.clone()} args={locale_args([("name", "Thimo")])}/></div>
            </header>
        }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct HeaderProps {
    pub langid: LanguageIdentifier,
}
