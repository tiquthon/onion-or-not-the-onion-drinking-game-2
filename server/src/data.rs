use std::collections::HashMap;

use crate::model::{QuestionId, RedditSubmissionData};

const RAW_NOT_THE_ONION_BEST: &str = include_str!("../assets/nottheonion.best.max2000.ron");
const RAW_NOT_THE_ONION_TOP: &str = include_str!("../assets/nottheonion.top.max2000.ron");
const RAW_THE_ONION_BEST: &str = include_str!("../assets/theonion.best.max2000.ron");
const RAW_THE_ONION_TOP: &str = include_str!("../assets/theonion.top.max2000.ron");

lazy_static::lazy_static! {
    static ref NOT_THE_ONION_BEST: HashMap<QuestionId, RedditSubmissionData> = {
        parse(RAW_NOT_THE_ONION_BEST)
    };

    static ref NOT_THE_ONION_TOP: HashMap<QuestionId, RedditSubmissionData> = {
        parse(RAW_NOT_THE_ONION_TOP)
    };

    static ref THE_ONION_BEST: HashMap<QuestionId, RedditSubmissionData> = {
        parse(RAW_THE_ONION_BEST)
    };

    static ref THE_ONION_TOP: HashMap<QuestionId, RedditSubmissionData> = {
        parse(RAW_THE_ONION_TOP)
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