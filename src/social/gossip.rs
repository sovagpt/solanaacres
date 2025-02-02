use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipNetwork {
    gossip_items: HashMap<Uuid, GossipItem>,
    npc_knowledge: HashMap<Uuid, HashSet<Uuid>>,  // NPC -> Known gossip items
    credibility_scores: HashMap<Uuid, f32>,       // NPC -> Credibility score
    propagation_paths: HashMap<Uuid, Vec<Uuid>>,  // Gossip -> Propagation path
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GossipItem {
    id: Uuid,
    content: String,
    source: Uuid,
    credibility: f32,
    timestamp: f32,
    affected_npcs: HashSet<Uuid>,
    truth_value: Option<bool>,
    emotional_impact: f32,
}

impl Default for GossipNetwork {
    fn default() -> Self {
        Self {
            gossip_items: HashMap::new(),
            npc_knowledge: HashMap::new(),
            credibility_scores: HashMap::new(),
            propagation_paths: HashMap::new(),
        }
    }
}

impl GossipNetwork {
    pub fn update(&mut self, delta_time: f32) {
        // Update gossip items
        self.decay_old_gossip(delta_time);
        
        // Update credibility scores
        self.update_credibility_scores();
    }

    pub fn spread_information(
        &mut self,
        source: Uuid,
        content: String,
        initial_credibility: f32
    ) -> Uuid {
        let gossip_id = Uuid::new_v4();
        
        let gossip = GossipItem {
            id: gossip_id,
            content,
            source,
            credibility: initial_credibility,
            timestamp: 0.0, // Current time should be passed in
            affected_npcs: HashSet::from([source]),
            truth_value: None,
            emotional_impact: 0.0,
        };

        self.gossip_items.insert(gossip_id, gossip);
        self.propagation_paths.insert(gossip_id, vec![source]);

        // Add to source's known gossip
        self.npc_knowledge
            .entry(source)
            .or_insert_with(HashSet::new)
            .insert(gossip_id);

        gossip_id
    }

    pub fn propagate_gossip(
        &mut self,
        gossip_id: Uuid,
        from_npc: Uuid,
        to_npc: Uuid,
        distortion_factor: f32
    ) -> bool {
        if let Some(gossip) = self.gossip_items.get_mut(&gossip_id) {
            // Check if receiver already knows this gossip
            if self.npc_knowledge
                .get(&to_npc)
                .map_or(false, |known| known.contains(&gossip_id))
            {
                return false;
            }

            // Apply distortion
            let credibility = (gossip.credibility * (1.0 - distortion_factor))
                .clamp(0.0, 1.0);

            // Update gossip
            gossip.affected_npcs.insert(to_npc);
            gossip.credibility = credibility;

            // Record propagation path
            if let Some(path) = self.propagation_paths.get_mut(&gossip_id) {
                path.push(to_npc);
            }

            // Add to receiver's known gossip
            self.npc_knowledge
                .entry(to_npc)
                .or_insert_with(HashSet::new)
                .insert(gossip_id);

            true
        } else {
            false
        }
    }

    pub fn verify_information(&mut self, gossip_id: Uuid, is_true: bool) {
        if let Some(gossip) = self.gossip_items.get_mut(&gossip_id) {
            gossip.truth_value = Some(is_true);
            
            // Update credibility scores of involved NPCs
            for &npc in &gossip.affected_npcs {
                let score = self.credibility_scores.entry(npc).or_insert(0.5);
                if is_true {
                    *score = (*score + 0.1).min(1.0);
                } else {
                    *score = (*score - 0.1).max(0.0);
                }
            }
        }
    }

    pub fn get_npc_known_gossip(&self, npc_id: &Uuid) -> Vec<&GossipItem> {
        self.npc_knowledge
            .get(npc_id)
            .map_or(Vec::new(), |gossip_ids| {
                gossip_ids
                    .iter()
                    .filter_map(|id| self.gossip_items.get(id))
                    .collect()
            })
    }

    pub fn get_propagation_path(&self, gossip_id: &Uuid) -> Option<&Vec<Uuid>> {
        self.propagation_paths.get(gossip_id)
    }

    fn decay_old_gossip(&mut self, delta_time: f32) {
        const DECAY_THRESHOLD: f32 = 1000.0; // Time until gossip starts to decay
        const REMOVAL_THRESHOLD: f32 = 2000.0; // Time until gossip is removed

        let mut to_remove = Vec::new();

        for (id, gossip) in &mut self.gossip_items {
            if gossip.timestamp > DECAY_THRESHOLD {
                gossip.credibility *= 0.99f32.powf(delta_time);
            }
            
            if gossip.timestamp > REMOVAL_THRESHOLD {
                to_remove.push(*id);
            }
        }

        // Remove old gossip
        for id in to_remove {
            self.remove_gossip(id);
        }
    }

    fn update_credibility_scores(&mut self) {
        for (npc_id, score) in &mut self.credibility_scores {
            let verified_gossip = self.get_npc_verified_gossip_ratio(*npc_id);
            *score = (*score * 0.9 + verified_gossip * 0.1).clamp(0.0, 1.0);
        }
    }

    fn get_npc_verified_gossip_ratio(&self, npc_id: Uuid) -> f32 {
        let known_gossip = match self.npc_knowledge.get(&npc_id) {
            Some(gossip_ids) => gossip_ids,
            None => return 0.5,
        };

        let mut total = 0;
        let mut correct = 0;

        for &gossip_id in known_gossip {
            if let Some(gossip) = self.gossip_items.get(&gossip_id) {
                if let Some(true_value) = gossip.truth_value {
                    total += 1;
                    if true_value {
                        correct += 1;
                    }
                }
            }
        }

        if total == 0 {
            0.5
        } else {
            correct as f32 / total as f32
        }
    }

    fn remove_gossip(&mut self, gossip_id: Uuid) {
        self.gossip_items.remove(&gossip_id);
        self.propagation_paths.remove(&gossip_id);
        
        // Remove from all NPCs' knowledge
        for known_gossip in self.npc_knowledge.values_mut() {
            known_gossip.remove(&gossip_id);
        }
    }
}