use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::{LanguageIdentifier, Loader};

use gloo_storage::errors::StorageError;
use gloo_storage::{LocalStorage, Storage};

use yew::{function_component, html, use_context, AttrValue, Html};

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        core_locales: "./locales/core.ftl",
    };
}

const LOCAL_STORAGE_KEY_LANGUAGE_IDENTIFIER: &str = "language_identifier";

#[function_component(LocaleComponent)]
pub fn locale_component(props: &LocaleProps) -> Html {
    let langid = use_context::<LanguageIdentifier>().expect("Missing LanguageIdentifier context.");
    let optional_locale: Option<String> =
        LOCALES.lookup_with_args(&langid, &props.keyid, &props.args);
    match optional_locale {
        None => {
            log::warn!("Could not find {} in language files.", props.keyid);
            Default::default()
        }
        Some(locale) => html! { <>{locale}</> },
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct LocaleProps {
    pub keyid: AttrValue,
    #[prop_or_default]
    pub args: HashMap<String, FluentValue<'static>>,
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

pub fn load_or_else_browser_select_language_identifier_and_log_warnings(
) -> Option<LanguageIdentifier> {
    load_language_identifier_from_persistent_storage()
        .map_err(|inspect_error| {
            log::warn!("While loading language identifier: {inspect_error}");
            inspect_error
        })
        .ok()
        .flatten()
        .or_else(|| {
            select_language_identifier_by_browser_setting().unwrap_or_else(|error| {
                log::warn!("While browser selecting language identifier: {error}");
                None
            })
        })
}

pub fn load_language_identifier_from_persistent_storage(
) -> anyhow::Result<Option<LanguageIdentifier>> {
    use anyhow::Context;

    match LocalStorage::get::<String>(LOCAL_STORAGE_KEY_LANGUAGE_IDENTIFIER) {
        Ok(language_identifier_string) => LanguageIdentifier::from_str(&language_identifier_string)
            .map(Some)
            .with_context(|| format!("Could not parse LanguageIdentifier from value \"{language_identifier_string}\" from key \"{LOCAL_STORAGE_KEY_LANGUAGE_IDENTIFIER}\" from LocalStorage.")),
        Err(StorageError::KeyNotFound(_)) => Ok(None),
        Err(error) => Err(error).with_context(|| format!("Could not get language identifier from key \"{LOCAL_STORAGE_KEY_LANGUAGE_IDENTIFIER}\" from LocalStorage.")),
    }
}

pub fn select_language_identifier_by_browser_setting() -> anyhow::Result<Option<LanguageIdentifier>>
{
    Ok(web_sys::window()
        .ok_or_else(|| anyhow::anyhow!("Failed accessing the browser window."))?
        .navigator()
        .languages()
        .iter()
        .filter_map(|language: wasm_bindgen::JsValue| {
            language
                .as_string()
                /* Silently ignoring parsed error of browser's languages with `.ok()`,
                because we may encounter ill-formed language identifiers by some browsers. */
                .and_then(|language_string| LanguageIdentifier::from_str(&language_string).ok())
        })
        .find(|language_identifier: &LanguageIdentifier| {
            LOCALES
                .locales()
                .any(|langid: &LanguageIdentifier| *langid == *language_identifier)
        }))
}

pub fn store_language_identifier_to_persistent_storage_and_log_warnings(
    language_identifier: &LanguageIdentifier,
) {
    if let Err(error) = store_language_identifier_to_persistent_storage(language_identifier) {
        log::warn!(
            "While storing language identifier \"{}\" ({error}).",
            language_identifier.to_string()
        );
    }
}

pub fn store_language_identifier_to_persistent_storage(
    language_identifier: &LanguageIdentifier,
) -> anyhow::Result<()> {
    use anyhow::Context;

    LocalStorage::set(LOCAL_STORAGE_KEY_LANGUAGE_IDENTIFIER, language_identifier.to_string())
        .with_context(|| format!("Could not store selected LanguageIdentifier to LocalStorage \"{LOCAL_STORAGE_KEY_LANGUAGE_IDENTIFIER}\"."))
}
