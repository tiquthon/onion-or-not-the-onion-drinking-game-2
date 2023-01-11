use std::fs::File;
use std::path::PathBuf;

use anyhow::Context;

use clap::Parser;

use reqwest::StatusCode;

use onion_or_not_the_onion_drinking_game_2_reddit_gatherer::response_model::{R1, R3, R4};

fn main() {
    let ClapArgs {
        subreddit_name,
        feed_type,
        count,
        fetch_amount_per_request,
        output_file_path,
        overwrite_output_file,
    } = ClapArgs::parse();

    if output_file_path.exists() && !overwrite_output_file {
        eprintln!("ERROR: Output file \"{output_file_path:?}\" does already exist.");
        eprintln!("ERROR: Supply `--overwrite-output-file` to force overwriting it.");
        return;
    }

    if count < 1 {
        eprintln!("ERROR: Given number of posts ({count}) to retrieve is too low.");
        return;
    }

    let subreddit_information: anyhow::Result<Vec<RedditSubmissionData>> = SubredditIterator::new(
        &subreddit_name,
        feed_type,
        fetch_amount_per_request.min(count),
    )
    .take(usize::try_from(count).unwrap())
    .collect::<Result<_, _>>()
    .context("Failed collecting reddit submissions.");

    match subreddit_information {
        Ok(subreddit_information) => {
            println!(
                "STATUS: Collected {} submissions.",
                subreddit_information.len()
            );
            match File::create(output_file_path) {
                Ok(output_file) => match ron::ser::to_writer(output_file, &subreddit_information) {
                    Ok(_) => println!("Successfully saved subreddit information."),
                    Err(error) => eprintln!("ERROR: Could not write to output file ({error})"),
                },
                Err(error) => eprintln!("ERROR: Could not open output file ({error})"),
            }
        }
        Err(error) => {
            eprintln!("ERROR: {error:?}");
        }
    }
}

#[derive(clap::Parser, Debug)]
#[command(name = "ONTO 2 Reddit Gatherer", author, version, about, long_about = None)]
struct ClapArgs {
    #[arg(short, long, required = true)]
    /// The subreddit name from which headlines and images should be retrieved (TheOnion, NotTheOnion).
    subreddit_name: String,

    #[arg(short = 't', long, default_value = "hot")]
    /// The type of feed to retrieve of the subreddit.
    feed_type: FeedType,

    #[arg(short, long, default_value_t = 25)]
    /// The number of posts to retrieve.
    count: u32,

    #[arg(short = 'a', long, default_value_t = 100)]
    fetch_amount_per_request: u32,

    #[arg(short, long = "output", default_value = "data.ron")]
    /// The destination file to save the retrieved posts in.
    output_file_path: PathBuf,

    #[arg(short = 'f', long, default_value_t = false)]
    /// If the destination file should be overwritten.
    overwrite_output_file: bool,
}

#[derive(clap::ValueEnum, Copy, Clone, Debug, strum::Display)]
enum FeedType {
    Best,
    Hot,
    New,
    Rising,
    Top,
    Controversial,
}

struct SubredditIterator {
    client: reqwest::blocking::Client,
    subreddit_name: String,
    feed_type: FeedType,
    fetch_amount_per_request: u32,
    previous_anchor: Option<String>,
    cache: Vec<RedditSubmissionData>,
    reached_end: bool,
}

impl SubredditIterator {
    fn new(subreddit_name: &str, feed_type: FeedType, fetch_amount_per_request: u32) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            subreddit_name: subreddit_name.to_string(),
            feed_type,
            fetch_amount_per_request,
            previous_anchor: None,
            cache: Vec::new(),
            reached_end: false,
        }
    }
}

impl Iterator for SubredditIterator {
    type Item = anyhow::Result<RedditSubmissionData>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_empty() && !self.reached_end {
            let additional_parameters: &str = match self.feed_type {
                FeedType::Best | FeedType::New | FeedType::Rising => "",
                FeedType::Hot => "&g=GLOBAL",
                FeedType::Top | FeedType::Controversial => "&t=all",
            };
            let mut fetch_url = format!(
                "https://www.reddit.com/r/{}/{}.json?limit={}&raw_json=1{additional_parameters}",
                self.subreddit_name,
                self.feed_type.to_string().to_lowercase(),
                self.fetch_amount_per_request
            );

            if let Some(after) = &self.previous_anchor {
                fetch_url.push_str(&format!("&after={after}"));
            }

            println!("STATUS: Fetching {fetch_url}");

            let response = match self.client.get(fetch_url.clone()).send() {
                Ok(response) => {
                    if matches!(response.status(), StatusCode::OK) {
                        response
                    } else {
                        return Some(Err(anyhow::anyhow!(
                            "Fetching \"{fetch_url}\" returned status {}.",
                            response.status()
                        )));
                    }
                }
                Err(error) => {
                    return Some(
                        Err(error).with_context(|| format!("Failed fetching \"{fetch_url}\".")),
                    )
                }
            };

            let response_body: R1 = match response.json() {
                Ok(response_body) => response_body,
                Err(error) => {
                    return Some(Err(error).with_context(|| {
                        format!("Failed parsing response body for \"{fetch_url}\".")
                    }))
                }
            };

            let after = response_body.data.after;
            if after.is_none() {
                self.reached_end = true;
            }

            let children: anyhow::Result<Vec<RedditSubmissionData>> = response_body
                .data
                .children
                .into_iter()
                .map(|child| {
                    let R3 {
                        data:
                            R4 {
                                subreddit,
                                subreddit_id,
                                id,
                                permalink,
                                created,
                                created_utc,
                                url,
                                title,
                                score,
                                downs,
                                ups,
                                over_18,
                                thumbnail,
                                preview,
                            },
                    } = child;
                    Ok(RedditSubmissionData {
                        subreddit,
                        subreddit_id,
                        id,
                        permalink: permalink.clone(),
                        created: created as u64,
                        created_utc: created_utc as u64,
                        url,
                        title,
                        score: score as u64,
                        downs: downs as u64,
                        ups: ups as u64,
                        over_18,
                        thumbnail,
                        preview_image_url: preview
                            .map(|preview| {
                                preview
                                    .images
                                    .first()
                                    .ok_or_else(|| {
                                        anyhow::anyhow!(
                                    "Missing image in preview of permalink=\"{permalink}\"."
                                )
                                    })
                                    .map(|first| first.source.url.clone())
                            })
                            .transpose()?,
                    })
                })
                .collect::<Result<_, _>>();
            let children = match children {
                Ok(children) => children,
                Err(error) => return Some(Err(error)),
            };

            self.previous_anchor = after;
            self.cache.extend(children);

            if self.cache.is_empty() {
                self.reached_end = true;
            }
        }

        if self.cache.is_empty() {
            None
        } else {
            Some(Ok(self.cache.remove(0)))
        }
    }
}

#[derive(serde::Serialize)]
struct RedditSubmissionData {
    subreddit: String,
    subreddit_id: String,
    id: String,
    permalink: String,
    created: u64,
    created_utc: u64,
    url: String,
    title: String,
    score: u64,
    downs: u64,
    ups: u64,
    over_18: bool,
    thumbnail: String,
    preview_image_url: Option<String>,
}
