pub mod proposals;
pub mod results;

use tokio::sync::mpsc;
use std::collections::HashMap;

pub struct VotingSystem {
    active_proposals: HashMap<String, proposals::Proposal>,
    completed_votes: Vec<results::VoteResults>,
    event_sender: mpsc::Sender<super::NetworkEvent>,
}

impl VotingSystem {
    pub fn new(event_sender: mpsc::Sender<super::NetworkEvent>) -> Self {
        Self {
            active_proposals: HashMap::new(),
            completed_votes: Vec::new(),
            event_sender,
        }
    }

    pub async fn create_proposal(&mut self, proposal: proposals::Proposal) {
        self.active_proposals.insert(proposal.id.clone(), proposal.clone());
        self.event_sender.send(super::NetworkEvent::ProposalCreated(proposal)).await.ok();
    }
}