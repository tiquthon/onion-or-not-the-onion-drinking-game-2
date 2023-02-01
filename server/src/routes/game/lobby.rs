use std::collections::HashMap;

use chrono::Utc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

use crate::routes::game::lobbies_storage::{ClientInfo, LobbiesStorage};
use crate::routes::game::to_lobby_message::{RegisterType, ToLobbyMessage};

pub async fn start_lobby_task(
    count_of_questions: Option<u64>,
    minimum_score_per_question: Option<i64>,
    maximum_answer_time_per_question: Option<u64>,
    lobbies_storage: LobbiesStorage,
) -> crate::model::InviteCode {
    let (invite_code, mut unbounded_receiver, broadcast_sender) = lobbies_storage.create().await;

    let return_invite_code = invite_code.clone();

    tokio::spawn(async move {
        let mut game = crate::model::Game {
            configuration: crate::model::GameConfiguration {
                count_of_questions,
                minimum_score_per_question,
                maximum_answer_time_per_question,
            },
            game_state: crate::model::GameState::InLobby,
            players: vec![],
        };

        while let Some((client_info, to_lobby_message)) = unbounded_receiver.recv().await {
            let process_client_message_result = process_client_message(
                client_info,
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

    return_invite_code
}

async fn process_client_message(
    client_info: ClientInfo,
    to_lobby_message: ToLobbyMessage,
    invite_code: &crate::model::InviteCode,
    game: &mut crate::model::Game,
    broadcast_sender: &tokio::sync::broadcast::Sender<shared_model::network::ServerMessage>,
    lobbies_storage: &LobbiesStorage,
) -> ProcessClientMessageResult {
    let generate_game_full_update = |game: crate::model::Game| -> shared_model::game::Game {
        game.into_shared_model_game(invite_code.clone(), client_info.player_id, crate::data::get)
    };

    let broadcast_game_update = |game: crate::model::Game| {
        let game = generate_game_full_update(game);
        let update_all_response = shared_model::network::ServerMessage::GameFullUpdate(game);
        broadcast_sender.send(update_all_response).unwrap();
    };

    let client_game_update = |game: crate::model::Game| {
        let game = generate_game_full_update(game);
        let response = shared_model::network::ServerMessage::GameFullUpdate(game);
        client_info.callback.send(response).unwrap();
    };

    match to_lobby_message {
        ToLobbyMessage::Register {
            name,
            just_watch,
            register_type,
        } => {
            // Process
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
            let game = generate_game_full_update(game.clone());

            let create_or_join_response = match register_type {
                RegisterType::Creator => {
                    shared_model::network::ServerMessage::LobbyCreated(game.clone())
                }
                RegisterType::Joiner => {
                    shared_model::network::ServerMessage::LobbyJoined(game.clone())
                }
            };
            client_info.callback.send(create_or_join_response).unwrap();

            let update_all_response = shared_model::network::ServerMessage::GameFullUpdate(game);
            broadcast_sender.send(update_all_response).unwrap();

            ProcessClientMessageResult::Continue
        }
        ToLobbyMessage::Disconnect => {
            // Process
            game.players
                .retain(|player| player.id != client_info.player_id);

            if game.players.is_empty() {
                lobbies_storage.remove(invite_code).await;

                ProcessClientMessageResult::Exit
            } else {
                // Respond
                broadcast_game_update(game.clone());

                ProcessClientMessageResult::Continue
            }
        }
        ToLobbyMessage::ClientMessage(shared_model::network::ClientMessage::RequestFullUpdate) => {
            // Respond
            client_game_update(game.clone());

            ProcessClientMessageResult::Continue
        }
        ToLobbyMessage::ClientMessage(shared_model::network::ClientMessage::StartGame) => {
            match game.game_state {
                crate::model::GameState::InLobby => {
                    // Process
                    let current_question =
                        crate::data_model_bridge::get_random_answered_question().unwrap();

                    game.game_state = crate::model::GameState::Playing {
                        previous_questions: Vec::new(),
                        current_question,
                        playing_state: crate::model::PlayingState::Question {
                            time_until: game.configuration.maximum_answer_time_per_question.map(
                                |maximum_answer_time_per_question| {
                                    Utc::now()
                                        + chrono::Duration::seconds(
                                            i64::try_from(maximum_answer_time_per_question)
                                                .unwrap(),
                                        )
                                },
                            ),
                            answers: HashMap::new(),
                        },
                    };

                    // Respond
                    broadcast_game_update(game.clone());

                    ProcessClientMessageResult::Continue
                }
                crate::model::GameState::Playing { .. } => {
                    // Not starting game, because it's already running

                    ProcessClientMessageResult::Continue
                }
                crate::model::GameState::Aftermath { .. } => todo!(),
            }
        }
        ToLobbyMessage::ClientMessage(shared_model::network::ClientMessage::ChooseAnswer(
            answer,
        )) => {
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
                        .map(|time_until| *time_until >= Utc::now())
                        .unwrap_or(true);
                    if is_within_time_limit {
                        answers.insert(client_info.player_id, answer.into());

                        // Respond
                        broadcast_game_update(game.clone());

                        ProcessClientMessageResult::Continue
                    } else {
                        // Respond
                        client_info
                            .callback
                            .send(shared_model::network::ServerMessage::AnswerNotInTimeLimit)
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
        ToLobbyMessage::ClientMessage(shared_model::network::ClientMessage::RequestSkip) => {
            match &mut game.game_state {
                crate::model::GameState::Playing {
                    playing_state: crate::model::PlayingState::Solution { skip_request, .. },
                    ..
                } => {
                    // Process
                    if !skip_request.contains(&client_info.player_id) {
                        skip_request.push(client_info.player_id);
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
    }
}

enum ProcessClientMessageResult {
    Continue,
    Exit,
}
