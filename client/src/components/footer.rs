use std::sync::LazyLock;

use chrono::{DateTime, Local, Utc};

use fluent_templates::{LanguageIdentifier, Loader};

use itertools::Itertools;

use yew::{classes, function_component, html, use_context, Callback, Html};

use crate::components::locale::{locale, LOCALES};

#[function_component(FooterComponent)]
pub fn footer_component(props: &FooterProps) -> Html {
    let langid = use_context::<LanguageIdentifier>().expect("Missing LanguageIdentifier context.");

    let locale_change_buttons: Vec<Html> = LOCALES
        .locales()
        .sorted_by_key(|language_identifier| locale("language-name", language_identifier).unwrap_or_default())
        .map(|language_identifier| {
            let on_change_language_identifier = props.on_change_language_identifier.clone();
            let onclick = Callback::from(move |_| {
                on_change_language_identifier.emit(language_identifier.clone());
            });
            let is_selected = langid == *language_identifier;
            html! {
                <button type="button" class={classes!("button", "locale-selection__button")} disabled={is_selected} {onclick}>
                    {locale("language-name", language_identifier)}
                </button>
            }
        })
        .collect();

    let timestamp = BUILD_TIMESTAMP
        .map(|datetime| datetime.with_timezone(&Local))
        .map(|datetime| datetime.format("%Y-%m-%d %H:%M:%S %Z").to_string())
        .unwrap_or_else(|| "NO BUILD TIME".to_string());

    let build_information = OPTIONAL_BUILD_INFORMATION
        .as_ref()
        .map(|build_information| {
            let BuildInformation {
                git_commit_id_short,
                git_commit_timestamp,
                git_commit_branch,
                git_dirty,
                ..
            } = build_information;
            format!(
                "built on {} from commit {git_commit_branch}@{git_commit_id_short}{} from {}",
                timestamp,
                if *git_dirty { "*" } else { "" },
                git_commit_timestamp.with_timezone(&Local).format("%F %T")
            )
        });

    html! {
        <footer class={classes!("footer")}>
            <nav class={classes!("footer__locales-selection")}>
                {locale_change_buttons}
            </nav>
            <div class={classes!("footer__copyright")}>{"\u{00a9} 2023 Thimo \"Tiquthon\" Neumann"}</div>
            if let Some(build_information) = build_information {
                <div class={classes!("footer__build-stamp")}>
                    {build_information}
                </div>
            }
        </footer>
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub on_change_language_identifier: Callback<LanguageIdentifier>,
}

static BUILD_TIMESTAMP: LazyLock<Option<DateTime<Utc>>> = LazyLock::new(|| {
    Some(
        DateTime::parse_from_rfc3339(option_env!("BUILD_TIMESTAMP")?)
            .inspect_err(|error| log::warn!("Could not parse BUILD_TIMESTAMP ({error})"))
            .ok()?
            .with_timezone(&Utc),
    )
});

static OPTIONAL_BUILD_INFORMATION: LazyLock<Option<BuildInformation>> = LazyLock::new(|| {
    let git_commit_timestamp_seconds = option_env!("BUILD_GIT_COMMIT_TIMESTAMP")?
        .parse::<i64>()
        .inspect_err(|error| {
            log::warn!("Could not parse BUILD_GIT_COMMIT_TIMESTAMP as number ({error}).")
        })
        .ok()?;
    let git_commit_timestamp = DateTime::from_timestamp(git_commit_timestamp_seconds, 0)
        .or_else(|| {
            log::warn!("Could not create DateTime from BUILD_GIT_COMMIT_TIMESTAMP {git_commit_timestamp_seconds}");
            None
        })?;

    Some(BuildInformation {
        git_commit_id_short: option_env!("BUILD_GIT_COMMIT_ID_SHORT")?,
        _git_commit_id_long: option_env!("BUILD_GIT_COMMIT_ID_LONG")?,
        git_commit_timestamp,
        git_commit_branch: option_env!("BUILD_GIT_COMMIT_BRANCH")?,
        git_dirty: option_env!("BUILD_GIT_DIRTY")? == "true",
    })
});

struct BuildInformation {
    git_commit_id_short: &'static str,
    _git_commit_id_long: &'static str,
    git_commit_timestamp: DateTime<Utc>,
    git_commit_branch: &'static str,
    git_dirty: bool,
}
