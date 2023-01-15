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

pub struct LocaleComponent {
    langid: LanguageIdentifier,
    _context_listener: ContextHandle<LanguageIdentifier>,
}

impl Component for LocaleComponent {
    type Message = LocaleComponentMsg;
    type Properties = LocaleProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (langid, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(LocaleComponentMsg::MessageContextUpdated),
            )
            .expect("Missing LanguageIdentifier context.");
        Self {
            langid,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LocaleComponentMsg::MessageContextUpdated(langid) => {
                self.langid = langid;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let locale: Option<String> =
            LOCALES.lookup_with_args(&self.langid, &ctx.props().keyid, &ctx.props().args);
        match locale {
            None => {
                log::warn!("Could not find {} in language files.", ctx.props().keyid);
                Default::default()
            }
            Some(locale) => html! { <>{locale}</> },
        }
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct LocaleProps {
    pub keyid: AttrValue,
    #[prop_or_default]
    pub args: HashMap<String, FluentValue<'static>>,
}

pub enum LocaleComponentMsg {
    MessageContextUpdated(LanguageIdentifier),
}

pub fn locale_args<const N: usize, KEY>(
    args: [(KEY, FluentValue); N],
) -> HashMap<String, FluentValue>
where
    KEY: Into<String>,
{
    args.into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect()
}

pub fn locale_args_vec<KEY>(args: Vec<(KEY, FluentValue)>) -> HashMap<String, FluentValue>
where
    KEY: Into<String>,
{
    args.into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect()
}

pub fn locale(key: &str, langid: &LanguageIdentifier) -> Option<String> {
    LOCALES.lookup(langid, key)
}

pub fn locale_with_args(
    key: &str,
    langid: &LanguageIdentifier,
    args: &HashMap<String, FluentValue<'static>>,
) -> Option<String> {
    LOCALES.lookup_with_args(langid, key, args)
}
