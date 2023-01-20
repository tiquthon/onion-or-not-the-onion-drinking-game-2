use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};

use uuid::Uuid;

use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

const INVITE_CODE_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const POSSIBLE_INVITE_CODE_COMBINATIONS: usize = INVITE_CODE_CHARS.len()
    * (INVITE_CODE_CHARS.len() - 1)
    * (INVITE_CODE_CHARS.len() - 2)
    * (INVITE_CODE_CHARS.len() - 3);

/* SERVER */

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Server {
    pub previous_invite_codes: Vec<InviteCode>,
    pub games: HashMap<InviteCode, Game>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            previous_invite_codes: Vec::new(),
            games: HashMap::new(),
        }
    }

    pub fn reduce_previous_invite_codes(&mut self) {
        if self.previous_invite_codes.len() > (POSSIBLE_INVITE_CODE_COMBINATIONS / 10) {
            let left_index =
                self.previous_invite_codes.len() - (self.previous_invite_codes.len() / 100);
            self.previous_invite_codes = self.previous_invite_codes[left_index..].to_vec();
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

/* INVITE CODE */

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct InviteCode(pub String);

impl InviteCode {
    pub fn generate() -> Self {
        use rand::seq::SliceRandom;
        Self(
            INVITE_CODE_CHARS
                .choose_multiple(&mut rand::thread_rng(), 4)
                .collect::<String>(),
        )
    }
}

impl Display for InviteCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::InviteCode> for InviteCode {
    fn into(self) -> shared_model::game::InviteCode {
        shared_model::game::InviteCode(self.0)
    }
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
        F: Fn(&QuestionId) -> RedditSubmissionData,
    {
        shared_model::game::Game {
            invite_code: invite_code.into(),
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
    pub count_of_questions: Option<u64>,
    pub minimum_score_per_question: Option<i64>,
    pub maximum_answer_time_per_question: Option<u64>,
}

/* GAME STATE */

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    InLobby,
    Playing {
        previous_questions: Vec<(QuestionId, HashMap<PlayerId, Answer>)>,
        current_question: QuestionId,
        playing_state: PlayingState,
    },
    Aftermath {
        questions: Vec<(QuestionId, HashMap<PlayerId, Answer>)>,
        restart_request: Vec<PlayerId>,
    },
}

impl GameState {
    pub fn into_shared_model_game_state<F>(
        self,
        own_id: &PlayerId,
        f: F,
    ) -> shared_model::game::GameState
    where
        F: Fn(&QuestionId) -> RedditSubmissionData,
    {
        match self {
            GameState::InLobby => shared_model::game::GameState::InLobby,
            GameState::Playing {
                current_question,
                playing_state,
                ..
            } => shared_model::game::GameState::Playing {
                current_question: f(&current_question).into(),
                playing_state: playing_state.into_shared_model_playing_state(own_id),
            },
            GameState::Aftermath {
                questions,
                restart_request,
            } => shared_model::game::GameState::Aftermath {
                questions: questions
                    .into_iter()
                    .map(|(question_id, player_answer_map)| {
                        (
                            f(&question_id).into(),
                            player_answer_map
                                .into_iter()
                                .map(|(player_id, answer)| (player_id.into(), answer.into()))
                                .collect(),
                        )
                    })
                    .collect(),
                restart_requests: restart_request.into_iter().map(Into::into).collect(),
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

/* PLAYING STATE */

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PlayingState {
    Question {
        time_until: DateTime<Utc>,
        answers: HashMap<PlayerId, Answer>,
    },
    Solution {
        time_until: DateTime<Utc>,
        answers: HashMap<PlayerId, Answer>,
        skip_request: Vec<PlayerId>,
    },
}

impl PlayingState {
    pub fn into_shared_model_playing_state(
        self,
        own_id: &PlayerId,
    ) -> shared_model::game::PlayingState {
        match self {
            PlayingState::Question {
                time_until,
                answers,
            } => {
                let own_answer = answers.get(own_id).copied();
                shared_model::game::PlayingState::Question {
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

/* ANSWER */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Answer {
    TheOnion,
    NotTheOnion,
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
pub struct PlayerName(pub String);

impl Display for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for PlayerName {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            panic!()
        }
        Ok(Self(value.trim().to_string()))
    }
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

// Allowing clippy::from_over_into, because don't want to and can't implement From<_> for shared_model.
#[allow(clippy::from_over_into)]
impl Into<shared_model::game::Question> for RedditSubmissionData {
    fn into(self) -> shared_model::game::Question {
        shared_model::game::Question {
            url: self.url,
            title: self.title,
            preview_image_url: self.preview_image_url,
        }
    }
}
