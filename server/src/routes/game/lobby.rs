use std::collections::{HashMap, HashSet};
use std::time::Duration;

use chrono::Utc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

use crate::routes::game::from_lobby_message::FromLobbyMessage;
use crate::routes::game::lobbies_storage::LobbiesStorage;
use crate::routes::game::to_lobby_message::{ClientInfo, RegisterType, ToLobbyMessage};

const PLAYING_STATE_SOLUTION_TIME_IN_SECONDS: u64 = 30;

pub async fn start_lobby_task(
    count_of_questions: Option<u64>,
    minimum_score_per_question: Option<i64>,
    maximum_answer_time_per_question: Option<u64>,
    lobbies_storage: LobbiesStorage,
) -> crate::model::InviteCode {
    let (invite_code, mut unbounded_receiver, broadcast_sender) = lobbies_storage.create().await;

    let return_invite_code = invite_code.clone();

    let (unbounded_sender, _) = lobbies_storage.retrieve(&invite_code).await.unwrap();

    tokio::spawn(async move {
        let mut game = crate::model::Game {
            configuration: crate::model::GameConfiguration {
                count_of_questions: count_of_questions.unwrap_or_else(|| {
                    u64::try_from(crate::data::calculate_count_of_questions(
                        minimum_score_per_question,
                    ))
                    .unwrap()
                }),
                minimum_score_per_question,
                maximum_answer_time_per_question,
            },
            game_state: crate::model::GameState::InLobby,
            players: Vec::new(),
        };

        while let Some(to_lobby_message) = unbounded_receiver.recv().await {
            let process_client_message_result = process_client_message(
                to_lobby_message,
                &invite_code,
                &mut game,
                &broadcast_sender,
                &lobbies_storage,
            )
            .await;
            if matches!(
                process_client_message_result,
                ProcessClientMessageResult::Exit
            ) {
                break;
            }
        }
    });

    tokio::spawn(async move {
        while !unbounded_sender.is_closed() {
            unbounded_sender
                .send(ToLobbyMessage::IntervalUpdate)
                .unwrap();
            tokio::time::sleep(Duration::from_millis(333)).await;
        }
    });

    return_invite_code
}

async fn process_client_message(
    to_lobby_message: ToLobbyMessage,
    invite_code: &crate::model::InviteCode,
    game: &mut crate::model::Game,
    broadcast_sender: &tokio::sync::broadcast::Sender<FromLobbyMessage>,
    lobbies_storage: &LobbiesStorage,
) -> ProcessClientMessageResult {
    let broadcast_game_update = |game: crate::model::Game| {
        broadcast_sender
            .send(FromLobbyMessage::GameFullUpdate(game))
            .unwrap();
    };

    let client_game_update = |game: crate::model::Game, client_info: &ClientInfo| {
        client_info
            .callback
            .send(FromLobbyMessage::GameFullUpdate(game))
            .unwrap();
    };

    match to_lobby_message {
        ToLobbyMessage::Register {
            client_info,
            name,
            just_watch,
            register_type,
        } => {
            // Process
            if game.players.iter().any(|player| player.name == name) {
                // Respond
                client_info
                    .callback
                    .send(FromLobbyMessage::PlayerNameAlreadyInUse)
                    .unwrap();

                ProcessClientMessageResult::Continue
            } else {
                game.players
                    .retain(|player| player.id != client_info.player_id);
                game.players.push(crate::model::Player {
                    id: client_info.player_id,
                    name,
                    play_type: if just_watch {
                        crate::model::PlayType::Watcher
                    } else {
                        crate::model::PlayType::Player { points: 0 }
                    },
                });

                // Respond
                let create_or_join_response = match register_type {
                    RegisterType::Creator => FromLobbyMessage::LobbyCreated(game.clone()),
                    RegisterType::Joiner => FromLobbyMessage::LobbyJoined(game.clone()),
                };
                client_info.callback.send(create_or_join_response).unwrap();

                broadcast_game_update(game.clone());

                ProcessClientMessageResult::Continue
            }
        }
        ToLobbyMessage::Disconnect { client_info } => {
            // Process
            game.players
                .retain(|player| player.id != client_info.player_id);

            if game.players.is_empty() {
                lobbies_storage.remove(invite_code).await;

                ProcessClientMessageResult::Exit
            } else {
                // Update
                match process_playing_update(game) {
                    ProcessPlayingUpdateResult::Broadcast
                    | ProcessPlayingUpdateResult::DoNothing => {
                        // Do nothing; broadcasting anyway
                    }
                }

                // Respond
                broadcast_game_update(game.clone());

                ProcessClientMessageResult::Continue
            }
        }
        ToLobbyMessage::IntervalUpdate => {
            match &mut game.game_state {
                crate::model::GameState::InLobby | crate::model::GameState::Aftermath { .. } => {
                    // Do nothing
                    ProcessClientMessageResult::Continue
                }
                crate::model::GameState::Playing { playing_state, .. } => {
                    let should_update = match playing_state {
                        crate::model::PlayingState::Question { time_until, .. } => {
                            let count_of_player_type_player = game
                                .players
                                .iter()
                                .filter(|player| player.is_player())
                                .count();
                            count_of_player_type_player == 0
                                || time_until
                                    .as_ref()
                                    .map_or(false, |time_until| *time_until < Utc::now())
                        }
                        crate::model::PlayingState::Solution { time_until, .. } => {
                            *time_until < Utc::now()
                        }
                    };
                    if should_update {
                        match process_playing_update(game) {
                            ProcessPlayingUpdateResult::Broadcast => {
                                broadcast_game_update(game.clone());
                            }
                            ProcessPlayingUpdateResult::DoNothing => {}
                        }
                    }
                    ProcessClientMessageResult::Continue
                }
            }
        }
        ToLobbyMessage::ClientMessage {
            client_info,
            client_message: shared_model::network::ClientMessage::RequestFullUpdate,
        } => {
            // Respond
            client_game_update(game.clone(), &client_info);

            ProcessClientMessageResult::Continue
        }
        ToLobbyMessage::ClientMessage {
            client_message: shared_model::network::ClientMessage::StartGame,
            ..
        } => {
            match game.game_state {
                crate::model::GameState::InLobby => {
                    // Process
                    game.game_state = create_new_game_state_playing(game);

                    // Respond
                    broadcast_game_update(game.clone());

                    ProcessClientMessageResult::Continue
                }
                crate::model::GameState::Playing { .. }
                | crate::model::GameState::Aftermath { .. } => {
                    // Not starting game, because it's already running

                    ProcessClientMessageResult::Continue
                }
            }
        }
        ToLobbyMessage::ClientMessage {
            client_info,
            client_message: shared_model::network::ClientMessage::ChooseAnswer(answer),
        } => {
            match &mut game.game_state {
                crate::model::GameState::Playing {
                    playing_state:
                        crate::model::PlayingState::Question {
                            time_until,
                            answers,
                        },
                    ..
                } => {
                    // Process
                    let is_within_time_limit = time_until
                        .as_ref()
                        .map_or(true, |time_until| *time_until >= Utc::now());
                    if is_within_time_limit {
                        answers.insert(client_info.player_id, answer.into());

                        // Update
                        match process_playing_update(game) {
                            ProcessPlayingUpdateResult::Broadcast
                            | ProcessPlayingUpdateResult::DoNothing => {
                                // Do nothing; broadcasting anyway
                            }
                        }

                        // Respond
                        broadcast_game_update(game.clone());

                        ProcessClientMessageResult::Continue
                    } else {
                        // Respond
                        client_info
                            .callback
                            .send(FromLobbyMessage::AnswerNotInTimeLimit)
                            .unwrap();

                        ProcessClientMessageResult::Continue
                    }
                }
                crate::model::GameState::InLobby
                | crate::model::GameState::Playing {
                    playing_state: crate::model::PlayingState::Solution { .. },
                    ..
                }
                | crate::model::GameState::Aftermath { .. } => {
                    // Not processing skip request, because not in GameState::Playing PlayingState::Question

                    ProcessClientMessageResult::Continue
                }
            }
        }
        ToLobbyMessage::ClientMessage {
            client_info,
            client_message: shared_model::network::ClientMessage::RequestSkip,
        } => {
            match &mut game.game_state {
                crate::model::GameState::Playing {
                    playing_state: crate::model::PlayingState::Solution { skip_request, .. },
                    ..
                } => {
                    // Process
                    if !skip_request.contains(&client_info.player_id) {
                        skip_request.insert(client_info.player_id);
                    }

                    // Update
                    match process_playing_update(game) {
                        ProcessPlayingUpdateResult::Broadcast
                        | ProcessPlayingUpdateResult::DoNothing => {
                            // Do nothing; broadcasting anyway
                        }
                    }

                    // Respond
                    broadcast_game_update(game.clone());

                    ProcessClientMessageResult::Continue
                }
                crate::model::GameState::InLobby
                | crate::model::GameState::Playing {
                    playing_state: crate::model::PlayingState::Question { .. },
                    ..
                }
                | crate::model::GameState::Aftermath { .. } => {
                    // Not processing skip request, because not in GameState::Playing PlayingState::Solution

                    ProcessClientMessageResult::Continue
                }
            }
        }
        ToLobbyMessage::ClientMessage {
            client_info,
            client_message: shared_model::network::ClientMessage::RequestPlayAgain,
        } => {
            match &mut game.game_state {
                crate::model::GameState::Aftermath {
                    restart_requests, ..
                } => {
                    restart_requests.push(client_info.player_id);

                    // Update
                    match process_playing_update(game) {
                        ProcessPlayingUpdateResult::Broadcast
                        | ProcessPlayingUpdateResult::DoNothing => {
                            // Do nothing; broadcasting anyway
                        }
                    }

                    // Respond
                    broadcast_game_update(game.clone());

                    ProcessClientMessageResult::Continue
                }
                crate::model::GameState::InLobby | crate::model::GameState::Playing { .. } => {
                    // Not processing play again request, because not in GameState::Aftermath

                    ProcessClientMessageResult::Continue
                }
            }
        }
    }
}

#[must_use]
fn process_playing_update(game: &mut crate::model::Game) -> ProcessPlayingUpdateResult {
    match &mut game.game_state {
        crate::model::GameState::InLobby => ProcessPlayingUpdateResult::DoNothing,
        crate::model::GameState::Aftermath {
            restart_requests, ..
        } => {
            let count_of_players_wanting_restart = game
                .players
                .iter()
                .filter(|player| player.is_player())
                .filter(|player| restart_requests.contains(&player.id))
                .count();
            let count_of_player_type_player = game
                .players
                .iter()
                .filter(|player| player.is_player())
                .count();
            if count_of_players_wanting_restart * 2 >= count_of_player_type_player {
                game.game_state = create_new_game_state_playing(game);
                ProcessPlayingUpdateResult::Broadcast
            } else {
                ProcessPlayingUpdateResult::DoNothing
            }
        }
        crate::model::GameState::Playing {
            previous_questions,
            current_question,
            playing_state,
        } => {
            match playing_state {
                crate::model::PlayingState::Question {
                    time_until,
                    answers,
                } => {
                    let count_of_player_type_player = game
                        .players
                        .iter()
                        .filter(|player| player.is_player())
                        .count();
                    let all_non_watchers_have_answered = game
                        .players
                        .iter()
                        .filter(|player| player.is_player())
                        .all(|player| answers.contains_key(&player.id));
                    if count_of_player_type_player == 0
                        || all_non_watchers_have_answered
                        || time_until
                            .as_ref()
                            .map_or(false, |time_until| *time_until < Utc::now())
                    {
                        // Give out points
                        let correct_players: Vec<crate::model::PlayerId> = game
                            .players
                            .iter_mut()
                            .filter_map(|player| {
                                let player_answer_is_correct = answers
                                    .get(&player.id)
                                    .map(|answer| *answer == current_question.answer);
                                match player_answer_is_correct {
                                    Some(true) => {
                                        match &mut player.play_type {
                                            crate::model::PlayType::Player { points } => {
                                                *points += 10;
                                            }
                                            crate::model::PlayType::Watcher => {}
                                        }
                                        Some(player.id)
                                    }
                                    Some(false) | None => None,
                                }
                            })
                            .collect();

                        // Reward minority correct players extra points
                        let are_correct_players_a_minority = (correct_players.len() as f64)
                            .lt(&(count_of_player_type_player as f64 / 2.0));
                        if are_correct_players_a_minority {
                            game.players
                                .iter_mut()
                                .filter(|player| correct_players.contains(&player.id))
                                .for_each(|correct_player| match &mut correct_player.play_type {
                                    crate::model::PlayType::Player { points } => {
                                        *points += 5;
                                    }
                                    crate::model::PlayType::Watcher => {}
                                });
                        }

                        // Switch to Solution
                        *playing_state = crate::model::PlayingState::Solution {
                            time_until: Utc::now()
                                + chrono::Duration::seconds(
                                    i64::try_from(PLAYING_STATE_SOLUTION_TIME_IN_SECONDS).unwrap(),
                                ),
                            answers: answers.clone(),
                            skip_request: HashSet::new(),
                        };

                        ProcessPlayingUpdateResult::Broadcast
                    } else {
                        ProcessPlayingUpdateResult::DoNothing
                    }
                }
                crate::model::PlayingState::Solution {
                    time_until,
                    answers,
                    skip_request,
                } => {
                    let all_non_watchers_have_requested_skip = game
                        .players
                        .iter()
                        .filter(|player| player.is_player())
                        .all(|player| skip_request.contains(&player.id));
                    if all_non_watchers_have_requested_skip || *time_until < Utc::now() {
                        // STORE
                        previous_questions.push((*current_question, answers.clone()));

                        // RENEW
                        let maximum_questions =
                            usize::try_from(game.configuration.count_of_questions).unwrap();
                        if previous_questions.len() < maximum_questions {
                            *current_question =
                                crate::data_model_bridge::get_random_answered_question(
                                    game.configuration.minimum_score_per_question,
                                    Some(
                                        &previous_questions
                                            .iter()
                                            .map(|k| k.0.question_id)
                                            .collect::<HashSet<_>>(),
                                    ),
                                    None,
                                )
                                .unwrap()
                                .unwrap();
                            *playing_state = create_new_playing_state_question(&game.configuration);

                            ProcessPlayingUpdateResult::Broadcast
                        } else {
                            game.game_state = crate::model::GameState::Aftermath {
                                ranked_players: game
                                    .players
                                    .iter()
                                    .filter_map(|player| match player.play_type {
                                        crate::model::PlayType::Player { points } => {
                                            Some((player.id, player.name.clone(), points))
                                        }
                                        crate::model::PlayType::Watcher => None,
                                    })
                                    .collect(),
                                restart_requests: Vec::new(),
                            };

                            ProcessPlayingUpdateResult::Broadcast
                        }
                    } else {
                        ProcessPlayingUpdateResult::DoNothing
                    }
                }
            }
        }
    }
}

fn create_new_game_state_playing(game: &crate::model::Game) -> crate::model::GameState {
    let current_question = crate::data_model_bridge::get_random_answered_question(
        game.configuration.minimum_score_per_question,
        None,
        None,
    )
    .unwrap()
    .unwrap();

    crate::model::GameState::Playing {
        previous_questions: Vec::new(),
        current_question,
        playing_state: create_new_playing_state_question(&game.configuration),
    }
}

fn create_new_playing_state_question(
    game_configuration: &crate::model::GameConfiguration,
) -> crate::model::PlayingState {
    crate::model::PlayingState::Question {
        time_until: game_configuration.maximum_answer_time_per_question.map(
            |maximum_answer_time_per_question| {
                Utc::now()
                    + chrono::Duration::seconds(
                        i64::try_from(maximum_answer_time_per_question).unwrap(),
                    )
            },
        ),
        answers: HashMap::new(),
    }
}

enum ProcessPlayingUpdateResult {
    Broadcast,
    DoNothing,
}

enum ProcessClientMessageResult {
    Continue,
    Exit,
}
