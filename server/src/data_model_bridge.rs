use std::collections::HashSet;

pub fn get_random_answered_question(
    minimum_score_per_question: Option<i64>,
    blacklist: Option<&HashSet<crate::model::QuestionId>>,
    timeout_retries: Option<u32>,
) -> anyhow::Result<Option<crate::model::AnsweredQuestion>> {
    Ok(get_answered_question(crate::data::get_random_question_id(
        minimum_score_per_question,
        blacklist,
        timeout_retries,
    )?))
}

pub fn get_answered_question(
    question_id: crate::model::QuestionId,
) -> Option<crate::model::AnsweredQuestion> {
    crate::data::get(&question_id)
        .map(|reddit_submission_data| {
            let answer = match reddit_submission_data.subreddit.to_lowercase().as_str() {
                "nottheonion" => crate::model::Answer::NotTheOnion,
                "theonion" => crate::model::Answer::TheOnion,
                other => unreachable!("the dataset should only contain subreddit = \"nottheonion\" or \"theonion\" and not \"{other}\""),
            };
            crate::model::AnsweredQuestion {
                question_id,
                answer,
            }
        })
}
