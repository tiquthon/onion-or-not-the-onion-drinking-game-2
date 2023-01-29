use std::collections::HashMap;
use std::sync::Arc;

use onion_or_not_the_onion_drinking_game_2_shared_library::model as shared_model;

use crate::model::POSSIBLE_INVITE_CODE_COMBINATIONS;
use crate::routes::game::to_lobby_message::ToLobbyMessage;

#[derive(Clone)]
pub struct LobbiesStorage {
    internal: Arc<tokio::sync::Mutex<InternalLobbiesStorage>>,
}

impl LobbiesStorage {
    pub async fn create(
        &self,
    ) -> (
        crate::model::InviteCode,
        tokio::sync::mpsc::UnboundedReceiver<(ClientInfo, ToLobbyMessage)>,
        tokio::sync::broadcast::Sender<shared_model::network::ServerMessage>,
    ) {
        let (broadcast_sender, broadcast_receiver) = tokio::sync::broadcast::channel(64);
        let (unbounded_sender, unbounded_receiver) = tokio::sync::mpsc::unbounded_channel();

        let mut locked_internal = tokio::sync::Mutex::lock(&self.internal).await;

        let possible_invite_code_combinations_remove_threshold =
            (POSSIBLE_INVITE_CODE_COMBINATIONS / 1000).clamp(
                64.min(POSSIBLE_INVITE_CODE_COMBINATIONS),
                1024.min(POSSIBLE_INVITE_CODE_COMBINATIONS),
            );
        if locked_internal.previous_invite_codes.len()
            > possible_invite_code_combinations_remove_threshold
        {
            let left_index = locked_internal.previous_invite_codes.len()
                - (locked_internal.previous_invite_codes.len() / 10);
            locked_internal.previous_invite_codes =
                locked_internal.previous_invite_codes[left_index..].to_vec();
        }

        let mut timeout: u64 = 100;
        let invite_code = loop {
            let invite_code = crate::model::InviteCode::generate();
            if !locked_internal.previous_invite_codes.contains(&invite_code) {
                break invite_code;
            }

            timeout = timeout.checked_sub(1).unwrap();
        };

        locked_internal
            .previous_invite_codes
            .push(invite_code.clone());
        locked_internal.lobbies.insert(
            invite_code.clone(),
            Lobby {
                sender_to_lobby: unbounded_sender,
                _broadcast_receiver_to_client: broadcast_receiver,
                broadcast_sender_to_client: broadcast_sender.clone(),
            },
        );

        drop(locked_internal);

        (invite_code, unbounded_receiver, broadcast_sender)
    }

    pub async fn retrieve(
        &self,
        invite_code: &crate::model::InviteCode,
    ) -> Option<(
        tokio::sync::mpsc::UnboundedSender<(ClientInfo, ToLobbyMessage)>,
        tokio::sync::broadcast::Receiver<shared_model::network::ServerMessage>,
    )> {
        let locked_internal = tokio::sync::Mutex::lock(&self.internal).await;

        let output = locked_internal.lobbies.get(invite_code).map(|lobby| {
            (
                lobby.sender_to_lobby.clone(),
                lobby.broadcast_sender_to_client.subscribe(),
            )
        });

        drop(locked_internal);

        output
    }

    pub async fn remove(&self, invite_code: &crate::model::InviteCode) {
        let mut locked_internal = tokio::sync::Mutex::lock(&self.internal).await;
        locked_internal.lobbies.remove(invite_code);
        drop(locked_internal);
    }
}

impl Default for LobbiesStorage {
    fn default() -> Self {
        LobbiesStorage {
            internal: Arc::new(tokio::sync::Mutex::new(InternalLobbiesStorage::default())),
        }
    }
}

#[derive(Default)]
struct InternalLobbiesStorage {
    previous_invite_codes: Vec<crate::model::InviteCode>,
    lobbies: HashMap<crate::model::InviteCode, Lobby>,
}

pub struct Lobby {
    sender_to_lobby: tokio::sync::mpsc::UnboundedSender<(ClientInfo, ToLobbyMessage)>,
    _broadcast_receiver_to_client:
        tokio::sync::broadcast::Receiver<shared_model::network::ServerMessage>,
    broadcast_sender_to_client:
        tokio::sync::broadcast::Sender<shared_model::network::ServerMessage>,
}

#[derive(Debug)]
pub struct ClientInfo {
    pub callback: tokio::sync::mpsc::UnboundedSender<shared_model::network::ServerMessage>,
    pub player_id: crate::model::PlayerId,
}
