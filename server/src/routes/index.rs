use std::io::ErrorKind;
use std::path::PathBuf;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};

use futures::stream::StreamExt;

use once_cell::sync::Lazy;

const INDEX_HTML: &str = include_str!("../../../client/dist/index.html");

static INDEX_HTML_BEFORE: Lazy<String> = Lazy::new(|| {
    let (html_before, _) = INDEX_HTML.split_once("<body>").unwrap();
    format!("{html_before}<body>")
});

static INDEX_HTML_AFTER: Lazy<String> =
    Lazy::new(|| INDEX_HTML.split_once("<body>").unwrap().1.to_owned());

static TRUNK_DIST_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
    std::fs::canonicalize(option_env!("ONION2_RUN_CLIENT_DIST_DIR").unwrap_or("../client/dist"))
        .unwrap()
});

#[tracing::instrument(name = "Index")]
pub async fn index() -> HttpResponse {
    let renderer = onion_or_not_the_onion_drinking_game_2_client::yew::ServerRenderer::<
        onion_or_not_the_onion_drinking_game_2_client::AppComponent,
    >::new();
    let body = futures::stream::once(async move { INDEX_HTML_BEFORE.clone() })
        .chain(renderer.render_stream())
        .chain(futures::stream::once(
            async move { INDEX_HTML_AFTER.clone() },
        ))
        .map(|m| Result::<web::Bytes, actix_web::Error>::Ok(m.into()));
    HttpResponse::Ok().content_type("text/html").streaming(body)
}

#[tracing::instrument(name = "Static File")]
pub async fn static_file(
    static_file: web::Path<String>,
) -> Result<actix_files::NamedFile, StaticFileError> {
    let static_file: String = static_file.into_inner();
    let canonical_file_path = std::fs::canonicalize(TRUNK_DIST_DIRECTORY.join(static_file))?;
    if canonical_file_path.starts_with(&*TRUNK_DIST_DIRECTORY) {
        Ok(actix_files::NamedFile::open(canonical_file_path)?)
    } else {
        Err(StaticFileError::PermissionDenied)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StaticFileError {
    #[error("File not found")]
    FileNotFound,
    #[error("Access not granted")]
    PermissionDenied,
    #[error("Internal Server Error")]
    Unknown(std::io::Error),
}

impl ResponseError for StaticFileError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::FileNotFound | Self::PermissionDenied => StatusCode::NOT_FOUND,
            Self::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<std::io::Error> for StaticFileError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => Self::FileNotFound,
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            _ => Self::Unknown(value),
        }
    }
}
