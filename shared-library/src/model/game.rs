use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};

use uuid::Uuid;

/* GAME */

#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub invite_code: InviteCode,
    pub configuration: GameConfiguration,
    pub game_state: GameState,
    pub players: Vec<Player>,
    pub this_player_id: PlayerId,
}

/* INVITE CODE */

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct InviteCode(pub String);

impl Display for InviteCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/* GAME CONFIGURATION */

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct GameConfiguration {
    pub count_of_questions: Option<u64>,
    pub minimum_score_per_question: Option<i64>,
    pub maximum_answer_time_per_question: Option<u64>,
}

/* GAME STATE */

#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum GameState {
    InLobby,
    Playing {
        current_question: Question,
        playing_state: PlayingState,
    },
    Aftermath {
        questions: Vec<(AnsweredQuestion, HashMap<PlayerId, Answer>)>,
        restart_requests: Vec<PlayerId>,
    },
}

/* QUESTION */

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Question {
    pub url: String,
    pub title: String,
    pub preview_image_url: Option<String>,
}

/* ANSWERED QUESTION */

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct AnsweredQuestion {
    pub question: Question,
    pub answer: Answer,
}

/* PLAYING STATE */

#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)] // Default, Display
pub enum PlayingState {
    Question {
        time_until: DateTime<Utc>,
        answers: Vec<PlayerId>,
        own_answer: Option<Answer>,
    },
    Solution {
        time_until: DateTime<Utc>,
        correct_answer: Answer,
        answers: HashMap<PlayerId, Answer>,
        skip_request: Vec<PlayerId>,
    },
}

/* ANSWER */

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum Answer {
    TheOnion,
    NotTheOnion,
}

/* PLAYER */

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Player {
    pub id: PlayerId,
    pub name: PlayerName,
    pub play_type: PlayType,
}

/* PLAYER ID */

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct PlayerId(pub Uuid);

impl Display for PlayerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/* PLAYER NAME */

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct PlayerName(pub String);

impl Display for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/* PLAY TYPE */

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum PlayType {
    Player { points: u16 },
    Watcher,
}
