use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, Utc};

use uuid::Uuid;

use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

const INVITE_CODE_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub const POSSIBLE_INVITE_CODE_COMBINATIONS: usize = INVITE_CODE_CHARS.len()
    * (INVITE_CODE_CHARS.len() - 1)
    * (INVITE_CODE_CHARS.len() - 2)
    * (INVITE_CODE_CHARS.len() - 3);

/* INVITE CODE */

const INVITE_CODE_CHAR_COUNT: usize = 4;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct InviteCode(String);

impl InviteCode {
    pub fn generate() -> Self {
        use rand::seq::SliceRandom;
        Self(
            INVITE_CODE_CHARS
                .choose_multiple(&mut rand::thread_rng(), INVITE_CODE_CHAR_COUNT)
                .collect::<String>(),
        )
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for InviteCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InviteCode {
    type Err = InviteCodeFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.len() != INVITE_CODE_CHAR_COUNT {
            Err(InviteCodeFromStrError::IncorrectCountOfChars {
                is: trimmed.len(),
                expected: INVITE_CODE_CHAR_COUNT,
            })
        } else if trimmed
            .chars()
            .all(|char| INVITE_CODE_CHARS.contains(&char))
        {
            Ok(Self(trimmed.to_uppercase()))
        } else {
            Err(InviteCodeFromStrError::InvalidCharInInviteCode {
                allowed: &INVITE_CODE_CHARS,
            })
        }
    }
}

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::InviteCode> for InviteCode {
    fn into(self) -> shared_model::game::InviteCode {
        shared_model::game::InviteCode(self.0)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InviteCodeFromStrError {
    #[error("Invite code has incorrect count of chars (is {is}, expected {expected})")]
    IncorrectCountOfChars { is: usize, expected: usize },
    #[error("Invite code contains invalid character (allowed ones are {allowed:?})")]
    InvalidCharInInviteCode { allowed: &'static [char] },
}

/* GAME */

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Game {
    pub configuration: GameConfiguration,
    pub game_state: GameState,
    pub players: Vec<Player>,
}

impl Game {
    pub fn into_shared_model_game<F>(
        self,
        invite_code: InviteCode,
        this_player_id: PlayerId,
        f: F,
    ) -> shared_model::game::Game
    where
        F: Fn(&QuestionId) -> Option<&RedditSubmissionData>,
    {
        shared_model::game::Game {
            invite_code: invite_code.into(),
            configuration: self.configuration.into(),
            game_state: self
                .game_state
                .into_shared_model_game_state(&this_player_id, f),
            players: self.players.into_iter().map(Into::into).collect(),
            this_player_id: this_player_id.into(),
        }
    }
}

/* GAME CONFIGURATION */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GameConfiguration {
    pub count_of_questions: u64,
    pub minimum_score_per_question: Option<i64>,
    pub maximum_answer_time_per_question: Option<u64>,
}

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::GameConfiguration> for GameConfiguration {
    fn into(self) -> shared_model::game::GameConfiguration {
        shared_model::game::GameConfiguration {
            count_of_questions: self.count_of_questions,
            minimum_score_per_question: self.minimum_score_per_question,
            maximum_answer_time_per_question: self.maximum_answer_time_per_question,
        }
    }
}

/* GAME STATE */

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    InLobby,
    Playing {
        previous_questions: Vec<(AnsweredQuestion, HashMap<PlayerId, Answer>)>,
        current_question: AnsweredQuestion,
        playing_state: PlayingState,
    },
    Aftermath {
        ranked_players: Vec<(PlayerId, PlayerName, u16)>,
        restart_requests: Vec<PlayerId>,
    },
}

impl GameState {
    pub fn into_shared_model_game_state<F>(
        self,
        own_id: &PlayerId,
        f: F,
    ) -> shared_model::game::GameState
    where
        F: Fn(&QuestionId) -> Option<&RedditSubmissionData>,
    {
        match self {
            GameState::InLobby => shared_model::game::GameState::InLobby,
            GameState::Playing {
                previous_questions,
                current_question,
                playing_state,
            } => shared_model::game::GameState::Playing {
                index_of_current_question: previous_questions.len(),
                playing_state: playing_state.into_shared_model_playing_state(
                    own_id,
                    &current_question,
                    f,
                ),
            },
            GameState::Aftermath {
                ranked_players,
                restart_requests: restart_request,
            } => shared_model::game::GameState::Aftermath {
                ranked_players: ranked_players
                    .into_iter()
                    .map(|(player_id, player_name, points)| {
                        (player_id.into(), player_name.into(), points)
                    })
                    .collect(),
                restart_requests: restart_request.into_iter().map(Into::into).collect(),
            },
        }
    }
}

/* PLAYING STATE */

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PlayingState {
    Question {
        time_until: Option<DateTime<Utc>>,
        answers: HashMap<PlayerId, Answer>,
    },
    Solution {
        time_until: DateTime<Utc>,
        answers: HashMap<PlayerId, Answer>,
        skip_request: HashSet<PlayerId>,
    },
}

impl PlayingState {
    pub fn into_shared_model_playing_state<F>(
        self,
        own_id: &PlayerId,
        answered_question: &AnsweredQuestion,
        f: F,
    ) -> shared_model::game::PlayingState
    where
        F: Fn(&QuestionId) -> Option<&RedditSubmissionData>,
    {
        match self {
            PlayingState::Question {
                time_until,
                answers,
            } => {
                let reddit_submission_data = f(&answered_question.question_id).unwrap();
                let own_answer = answers.get(own_id).copied();
                shared_model::game::PlayingState::Question {
                    current_question: shared_model::game::Question {
                        title: reddit_submission_data.title.clone(),
                    },
                    time_until,
                    answers: answers.into_keys().map(Into::into).collect(),
                    own_answer: own_answer.map(Into::into),
                }
            }
            PlayingState::Solution {
                time_until,
                answers,
                skip_request,
            } => shared_model::game::PlayingState::Solution {
                current_question: answered_question.into_shared_model_answered_question(&f),
                time_until,
                answers: answers
                    .into_iter()
                    .map(|(id, answer)| (id.into(), answer.into()))
                    .collect(),
                skip_request: skip_request.into_iter().map(Into::into).collect(),
            },
        }
    }
}

/* QUESTION ID */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct QuestionId(pub Uuid);

impl QuestionId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for QuestionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/* ANSWERED QUESTION */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AnsweredQuestion {
    pub question_id: QuestionId,
    pub answer: Answer,
}

impl AnsweredQuestion {
    pub fn into_shared_model_answered_question<F>(
        self,
        f: &F,
    ) -> shared_model::game::AnsweredQuestion
    where
        F: Fn(&QuestionId) -> Option<&RedditSubmissionData>,
    {
        let reddit_submission_data = (*f)(&self.question_id).unwrap();
        shared_model::game::AnsweredQuestion {
            question: shared_model::game::Question {
                title: reddit_submission_data.title.clone(),
            },
            url: reddit_submission_data.url.clone(),
            preview_image_url: reddit_submission_data.preview_image_url.clone(),
            answer: self.answer.into(),
        }
    }
}

/* ANSWER */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Answer {
    TheOnion,
    NotTheOnion,
}

impl From<shared_model::game::Answer> for Answer {
    fn from(value: shared_model::game::Answer) -> Self {
        match value {
            shared_model::game::Answer::TheOnion => Self::TheOnion,
            shared_model::game::Answer::NotTheOnion => Self::NotTheOnion,
        }
    }
}

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::Answer> for Answer {
    fn into(self) -> shared_model::game::Answer {
        match self {
            Answer::TheOnion => shared_model::game::Answer::TheOnion,
            Answer::NotTheOnion => shared_model::game::Answer::NotTheOnion,
        }
    }
}

/* PLAYER */

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: PlayerName,
    pub play_type: PlayType,
}

impl Player {
    pub fn is_player(&self) -> bool {
        match self.play_type {
            PlayType::Player { .. } => true,
            PlayType::Watcher => false,
        }
    }
}

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::Player> for Player {
    fn into(self) -> shared_model::game::Player {
        shared_model::game::Player {
            id: self.id.into(),
            name: self.name.into(),
            play_type: self.play_type.into(),
        }
    }
}

/* PLAYER ID */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PlayerId(pub Uuid);

impl PlayerId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::PlayerId> for PlayerId {
    fn into(self) -> shared_model::game::PlayerId {
        shared_model::game::PlayerId(self.0)
    }
}

/* PLAYER NAME */

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PlayerName {
    type Err = PlayerNameFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(PlayerNameFromStrError::EmptyAfterBeingTrimmed)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PlayerNameFromStrError {
    #[error("Player name is empty after being trimmed")]
    EmptyAfterBeingTrimmed,
}

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::PlayerName> for PlayerName {
    fn into(self) -> shared_model::game::PlayerName {
        shared_model::game::PlayerName(self.0)
    }
}

/* PLAY TYPE */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PlayType {
    Player { points: u16 },
    Watcher,
}

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::PlayType> for PlayType {
    fn into(self) -> shared_model::game::PlayType {
        match self {
            PlayType::Player { points } => shared_model::game::PlayType::Player { points },
            PlayType::Watcher => shared_model::game::PlayType::Watcher,
        }
    }
}

/* REDDIT SUBMISSION DATA */

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Deserialize)]
pub struct RedditSubmissionData {
    pub subreddit: String,
    pub subreddit_id: String,
    pub id: String,
    pub permalink: String,
    pub created: u64,
    pub created_utc: u64,
    pub url: String,
    pub title: String,
    pub score: u64,
    pub downs: u64,
    pub ups: u64,
    pub over_18: bool,
    pub thumbnail: String,
    pub preview_image_url: Option<String>,
}
