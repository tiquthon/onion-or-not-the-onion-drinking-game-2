use std::collections::HashMap;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::{LanguageIdentifier, Loader};

use yew::{html, AttrValue, Component, Context, Html};

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        core_locales: "./locales/core.ftl",
    };
}

pub struct Locale;

impl Component for Locale {
    type Message = ();
    type Properties = LocaleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let locale: Option<String> =
            LOCALES.lookup_with_args(&ctx.props().langid, &ctx.props().keyid, &ctx.props().args);
        html! { <>{locale}</> }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct LocaleProps {
    pub keyid: AttrValue,
    pub langid: LanguageIdentifier,
    #[prop_or_default]
    pub args: HashMap<String, FluentValue<'static>>,
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
