use std::collections::{HashMap, HashSet};

use crate::model::{QuestionId, RedditSubmissionData};

const RAW_NOT_THE_ONION_BEST: &str = include_str!("../assets/nottheonion.best.max2000.ron");
const RAW_NOT_THE_ONION_TOP: &str = include_str!("../assets/nottheonion.top.max2000.ron");
const RAW_THE_ONION_BEST: &str = include_str!("../assets/theonion.best.max2000.ron");
const RAW_THE_ONION_TOP: &str = include_str!("../assets/theonion.top.max2000.ron");

lazy_static::lazy_static! {
    static ref NOT_THE_ONION_BEST: HashMap<QuestionId, RedditSubmissionData> = {
        parse(RAW_NOT_THE_ONION_BEST)
    };

    static ref NOT_THE_ONION_BEST__KEYS: Vec<QuestionId> = {
        NOT_THE_ONION_BEST.keys().copied().collect()
    };

    static ref NOT_THE_ONION_TOP: HashMap<QuestionId, RedditSubmissionData> = {
        parse(RAW_NOT_THE_ONION_TOP)
    };

    static ref NOT_THE_ONION_TOP__KEYS: Vec<QuestionId> = {
        NOT_THE_ONION_TOP.keys().copied().collect()
    };

    static ref THE_ONION_BEST: HashMap<QuestionId, RedditSubmissionData> = {
        parse(RAW_THE_ONION_BEST)
    };

    static ref THE_ONION_BEST__KEYS: Vec<QuestionId> = {
        THE_ONION_BEST.keys().copied().collect()
    };

    static ref THE_ONION_TOP: HashMap<QuestionId, RedditSubmissionData> = {
        parse(RAW_THE_ONION_TOP)
    };

    static ref THE_ONION_TOP__KEYS: Vec<QuestionId> = {
        THE_ONION_TOP.keys().copied().collect()
    };

    static ref ALL__KEYS: Vec<QuestionId> = {
        NOT_THE_ONION_BEST.keys().copied()
            .chain(NOT_THE_ONION_TOP.keys().copied())
            .chain(THE_ONION_BEST.keys().copied())
            .chain(THE_ONION_BEST.keys().copied())
            .collect()
    };
}

fn parse(data: &str) -> HashMap<QuestionId, RedditSubmissionData> {
    ron::de::from_str::<Vec<RedditSubmissionData>>(data)
        .unwrap()
        .into_iter()
        .map(|submission_data| (QuestionId::generate(), submission_data))
        .collect()
}

pub fn get(question_id: &QuestionId) -> Option<&RedditSubmissionData> {
    NOT_THE_ONION_BEST
        .get(question_id)
        .or_else(|| NOT_THE_ONION_TOP.get(question_id))
        .or_else(|| THE_ONION_BEST.get(question_id))
        .or_else(|| THE_ONION_TOP.get(question_id))
}

pub fn get_random_question_id(
    minimum_score_per_question: Option<i64>,
    blacklist: Option<&HashSet<QuestionId>>,
    timeout_retries: Option<u32>,
) -> Result<QuestionId, GetRandomQuestionIdError> {
    use rand::distributions::Distribution;

    let left_over_count = ALL__KEYS.len() - blacklist.map(|blacklist| blacklist.len()).unwrap_or(0);

    for _ in 0..=timeout_retries.unwrap_or(100) {
        let selected_optional_question_id = ALL__KEYS
            .iter()
            .filter(|question_id| match blacklist {
                Some(blacklist) => !blacklist.contains(*question_id),
                None => true,
            })
            .nth(
                rand::distributions::uniform::Uniform::new(0, left_over_count - 1)
                    .sample(&mut rand::thread_rng()),
            )
            .copied();

        match selected_optional_question_id {
            None => return Err(GetRandomQuestionIdError::NoneFound),
            Some(selected_question_id) => {
                let has_at_least_minimum_score = minimum_score_per_question
                    .map(|min_score| {
                        i64::try_from(get(&selected_question_id).unwrap().score).unwrap()
                            >= min_score
                    })
                    .unwrap_or(true);
                if has_at_least_minimum_score {
                    return Ok(selected_question_id);
                }
            }
        }
    }

    Err(GetRandomQuestionIdError::Timeout)
}

#[derive(thiserror::Error, Debug)]
pub enum GetRandomQuestionIdError {
    #[error("Could not select a question")]
    NoneFound,
    #[error("Reached timeout while sampling questions")]
    Timeout,
}

pub fn calculate_count_of_questions(minimum_score_per_question: Option<i64>) -> usize {
    match minimum_score_per_question {
        None => ALL__KEYS.len(),
        Some(min_score) => ALL__KEYS
            .iter()
            .filter(|question_id| {
                i64::try_from(get(question_id).unwrap().score).unwrap() >= min_score
            })
            .count(),
    }
}
