use std::collections::HashMap;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::{LanguageIdentifier, Loader};

use yew::{html, AttrValue, Component, Context, ContextHandle, Html};

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        core_locales: "./locales/core.ftl",
    };
}

pub struct Locale {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,
}

impl Component for Locale {
    type Message = LocaleMsg;
    type Properties = LocaleProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(ctx.link().callback(LocaleMsg::MessageContextUpdated))
            .expect("Missing LanguageIdentifier context.");
        Self {
            langid,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LocaleMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let locale: Option<String> =
            LOCALES.lookup_with_args(&self.langid, &ctx.props().keyid, &ctx.props().args);
        html! { <>{locale}</> }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct LocaleProps {
    pub keyid: AttrValue,
    #[prop_or_default]
    pub args: HashMap<String, FluentValue<'static>>,
}

pub enum LocaleMsg {
    MessageContextUpdated(LanguageIdentifier),
}

pub fn locale_args<'a, const N: usize, KEY, VALUE>(
    args: [(KEY, VALUE); N],
) -> HashMap<String, FluentValue<'a>>
where
    KEY: Into<String>,
    VALUE: Into<FluentValue<'a>>,
{
    args.into_iter()
        .map(|(key, value)| (key.into(), value.into()))
        .collect()
}

pub fn locale_args_vec<'a, KEY, VALUE>(args: Vec<(KEY, VALUE)>) -> HashMap<String, FluentValue<'a>>
where
    KEY: Into<String>,
    VALUE: Into<FluentValue<'a>>,
{
    args.into_iter()
        .map(|(key, value)| (key.into(), value.into()))
        .collect()
}
