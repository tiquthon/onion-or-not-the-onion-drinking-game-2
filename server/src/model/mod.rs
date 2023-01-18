use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use uuid::Uuid;

const INVITE_CODE_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const POSSIBLE_INVITE_CODE_COMBINATIONS: usize = INVITE_CODE_CHARS.len()
    * (INVITE_CODE_CHARS.len() - 1)
    * (INVITE_CODE_CHARS.len() - 2)
    * (INVITE_CODE_CHARS.len() - 3);

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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct InviteCode(String);

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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Game {
    pub game_state: GameState,
    pub players: Vec<Player>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    InLobby,
    Playing {
        previous_questions: Vec<QuestionId>,
        current_question: QuestionId,
        playing_state: PlayingState,
    },
    Aftermath {
        questions: Vec<QuestionId>,
        restart_request: Vec<PlayerId>,
    },
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub points: u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PlayerId(Uuid);

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

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PlayingState {
    Question {
        time_until: DateTime<Utc>,
        answers: HashMap<PlayerId, Answer>,
    },
    Answer {
        time_until: DateTime<Utc>,
        answers: HashMap<PlayerId, Answer>,
        skip_request: Vec<PlayerId>,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Answer {
    TheOnion,
    NotTheOnion,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Question {
    id: QuestionId,
    internal: RedditSubmissionData,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct QuestionId(Uuid);

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
