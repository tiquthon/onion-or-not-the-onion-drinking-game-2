use actix_web::{web, Error, HttpRequest, HttpResponse};

use crate::routes::game::client::{start_client_network_task, ClientType};
use crate::routes::game::lobbies_storage::LobbiesStorage;
use crate::routes::game::lobby::start_lobby_task;

pub mod client;
pub mod lobbies_storage;
pub mod lobby;
pub mod to_lobby_message;

#[tracing::instrument(name = "Create Lobby", skip(req, body, lobbies))]
pub async fn create_lobby(
    req: HttpRequest,
    body: web::Payload,
    lobbies: web::Data<LobbiesStorage>,
    query: web::Query<CreateLobbyQuery>,
) -> Result<HttpResponse, Error> {
    let CreateLobbyQuery {
        player_name,
        just_watch,
        count_of_questions,
        minimum_score_per_question,
        maximum_answer_time_per_question,
    } = query.into_inner();

    let (response, session, msg_stream) = actix_ws::handle(&req, body)?;

    let invite_code = start_lobby_task(
        count_of_questions,
        minimum_score_per_question,
        maximum_answer_time_per_question,
        LobbiesStorage::clone(&lobbies),
    )
    .await;

    tracing::info!(
        "Created Lobby \"{invite_code}\" by player \"{player_name}\" (just_watch:{just_watch}) with \
        {count_of_questions:?} questions, {minimum_score_per_question:?} minimum score and \
        {maximum_answer_time_per_question:?} maximum answer time"
    );

    start_client_network_task(
        crate::model::PlayerName(player_name.clone()),
        invite_code.clone(),
        just_watch,
        LobbiesStorage::clone(&lobbies),
        session,
        msg_stream,
        ClientType::LobbyCreator,
    )
    .await;

    tracing::info!(
        "Player \"{player_name}\" joined lobby \"{invite_code}\" (just_watch:{just_watch})"
    );

    Ok(response)
}

#[derive(Debug, Clone, Hash, serde::Deserialize)]
pub struct CreateLobbyQuery {
    player_name: String,
    just_watch: bool,
    count_of_questions: Option<u64>,
    minimum_score_per_question: Option<i64>,
    maximum_answer_time_per_question: Option<u64>,
}

#[tracing::instrument(name = "Join Lobby", skip(req, body, lobbies))]
pub async fn join_lobby(
    req: HttpRequest,
    body: web::Payload,
    lobbies: web::Data<LobbiesStorage>,
    path: web::Path<String>,
    query: web::Query<JoinLobbyQuery>,
) -> Result<HttpResponse, Error> {
    let invite_code = path.into_inner();
    let JoinLobbyQuery {
        player_name,
        just_watch,
    } = query.into_inner();

    let (response, session, msg_stream) = actix_ws::handle(&req, body)?;

    start_client_network_task(
        crate::model::PlayerName(player_name.clone()),
        crate::model::InviteCode(invite_code.clone()),
        just_watch,
        LobbiesStorage::clone(&lobbies),
        session,
        msg_stream,
        ClientType::LobbyJoiner,
    )
    .await;

    tracing::info!(
        "Player \"{player_name}\" joined lobby \"{invite_code}\" (just_watch:{just_watch})"
    );

    Ok(response)
}

#[derive(Debug, Clone, Hash, serde::Deserialize)]
pub struct JoinLobbyQuery {
    player_name: String,
    just_watch: bool,
}
