pub mod voting;
pub mod ws;

use tokio::sync::mpsc;
use std::collections::HashMap;
use uuid::Uuid;

pub struct NetworkManager {
    ws_server: ws::WebSocketServer,
    voting_system: voting::VotingSystem,
    connections: HashMap<Uuid, Connection>,
    event_sender: mpsc::Sender<NetworkEvent>,
    event_receiver: mpsc::Receiver<NetworkEvent>,
}

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    ClientConnected(Uuid),
    ClientDisconnected(Uuid),
    VoteSubmitted(Uuid, String),
    ProposalCreated(voting::proposals::Proposal),
    ResultsUpdated(voting::results::VoteResults),
}

impl NetworkManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            ws_server: ws::WebSocketServer::new(tx.clone()),
            voting_system: voting::VotingSystem::new(tx),
            connections: HashMap::new(),
            event_sender: tx,
            event_receiver: rx,
        }
    }

    pub async fn start(&mut self) {
        self.ws_server.start().await;
    }

    pub async fn update(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            self.handle_event(event).await;
        }
    }
}