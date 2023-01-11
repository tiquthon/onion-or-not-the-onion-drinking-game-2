//! These structures exist, because the previous attempt in using the crate `roux:2.2.5` did not work as needed.
//! Within `roux:2.2.5` the #R4::preview was missing. When someone finds the preview for the image url in `roux` this can be made obsolete.

#[derive(serde::Deserialize)]
pub struct R1 {
    pub data: R2,
}

#[derive(serde::Deserialize)]
pub struct R2 {
    pub after: Option<String>,
    pub children: Vec<R3>,
}

#[derive(serde::Deserialize)]
pub struct R3 {
    pub data: R4,
}

#[derive(serde::Deserialize)]
pub struct R4 {
    pub subreddit: String,
    pub subreddit_id: String,
    pub id: String,
    pub permalink: String,
    pub created: f64,
    pub created_utc: f64,
    pub url: String,
    pub title: String,
    pub score: f64,
    pub downs: f64,
    pub ups: f64,
    pub over_18: bool,
    pub thumbnail: String,
    pub preview: Option<R5>,
}

#[derive(serde::Deserialize)]
pub struct R5 {
    pub images: Vec<R6>,
}

#[derive(serde::Deserialize)]
pub struct R6 {
    pub source: R7,
}

#[derive(serde::Deserialize)]
pub struct R7 {
    pub url: String,
}
