use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

pub mod relationships;
pub mod influence;
pub mod groups;
pub mod gossip;

use relationships::{Relationship, RelationshipType};
use influence::SocialInfluence;
use groups::{Group, GroupDynamics};
use gossip::GossipNetwork;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNetwork {
    relationships: HashMap<(Uuid, Uuid), Relationship>,
    influences: HashMap<Uuid, SocialInfluence>,
    groups: Vec<Group>,
    gossip_network: GossipNetwork,
}

impl Default for SocialNetwork {
    fn default() -> Self {
        Self {
            relationships: HashMap::new(),
            influences: HashMap::new(),
            groups: Vec::new(),
            gossip_network: GossipNetwork::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialBehavior {
    trustworthiness: f32,
    charisma: f32,
    group_affinity: f32,
    social_connections: Vec<Uuid>,
    current_group: Option<Uuid>,
}

impl SocialNetwork {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update relationships
        for relationship in self.relationships.values_mut() {
            relationship.update(delta_time);
        }

        // Update group dynamics
        for group in &mut self.groups {
            group.update(delta_time);
        }

        // Process gossip and information spread
        self.gossip_network.update(delta_time);
    }

    pub fn add_relationship(&mut self, npc1: Uuid, npc2: Uuid, relationship_type: RelationshipType) {
        let relationship = Relationship::new(relationship_type);
        self.relationships.insert((npc1, npc2), relationship);
    }

    pub fn get_relationship(&self, npc1: Uuid, npc2: Uuid) -> Option<&Relationship> {
        self.relationships.get(&(npc1, npc2))
            .or_else(|| self.relationships.get(&(npc2, npc1)))
    }

    pub fn get_social_circle(&self, npc_id: Uuid) -> Vec<Uuid> {
        self.relationships.iter()
            .filter_map(|((id1, id2), rel)| {
                if *id1 == npc_id {
                    Some(*id2)
                } else if *id2 == npc_id {
                    Some(*id1)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn spread_information(&mut self, source: Uuid, information: String, credibility: f32) {
        self.gossip_network.spread_information(source, information, credibility);
    }

    pub fn create_group(&mut self, members: Vec<Uuid>, group_type: String) -> Uuid {
        let group = Group::new(members, group_type);
        let group_id = group.id;
        self.groups.push(group);
        group_id
    }
}

impl SocialBehavior {
    pub fn new(trustworthiness: f32, charisma: f32, group_affinity: f32) -> Self {
        Self {
            trustworthiness,
            charisma,
            group_affinity,
            social_connections: Vec::new(),
            current_group: None,
        }
    }

    pub fn update(&mut self, network: &mut SocialNetwork, own_id: Uuid) {
        // Update social connections
        self.social_connections = network.get_social_circle(own_id);
        
        // Update group participation
        if let Some(group_id) = self.current_group {
            if !network.groups.iter().any(|g| g.id == group_id) {
                self.current_group = None;
            }
        }
    }

    pub fn get_social_influence(&self) -> f32 {
        (self.charisma + self.trustworthiness) / 2.0
    }
}