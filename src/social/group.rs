use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub members: HashSet<Uuid>,
    pub leaders: HashSet<Uuid>,
    pub group_type: String,
    pub cohesion: f32,
    pub influence: f32,
    relationships: HashMap<(Uuid, Uuid), f32>,
    shared_beliefs: Vec<SharedBelief>,
    activities: Vec<GroupActivity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SharedBelief {
    content: String,
    strength: f32,
    supporters: HashSet<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroupActivity {
    activity_type: String,
    participants: HashSet<Uuid>,
    impact: f32,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDynamics {
    groups: HashMap<Uuid, Group>,
    member_affiliations: HashMap<Uuid, HashSet<Uuid>>,
}

impl Group {
    pub fn new(members: Vec<Uuid>, group_type: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: format!("{} Group", group_type),
            members: members.into_iter().collect(),
            leaders: HashSet::new(),
            group_type,
            cohesion: 0.5,
            influence: 0.5,
            relationships: HashMap::new(),
            shared_beliefs: Vec::new(),
            activities: Vec::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update group cohesion based on activities and relationships
        self.update_cohesion();
        
        // Update group influence
        self.update_influence();
        
        // Clean up old activities
        self.cleanup_old_activities(delta_time);
    }

    pub fn add_member(&mut self, member_id: Uuid) {
        if self.members.insert(member_id) {
            // Initialize relationships with other members
            for &existing_member in &self.members {
                if existing_member != member_id {
                    self.relationships.insert((member_id, existing_member), 0.0);
                }
            }
        }
    }

    pub fn remove_member(&mut self, member_id: &Uuid) {
        self.members.remove(member_id);
        self.leaders.remove(member_id);
        
        // Remove relationships involving this member
        self.relationships.retain(|(m1, m2), _| m1 != member_id && m2 != member_id);
    }

    pub fn add_shared_belief(&mut self, content: String, initial_strength: f32, supporters: HashSet<Uuid>) {
        self.shared_beliefs.push(SharedBelief {
            content,
            strength: initial_strength,
            supporters,
        });
    }

    pub fn record_activity(&mut self, activity_type: String, participants: HashSet<Uuid>, impact: f32) {
        self.activities.push(GroupActivity {
            activity_type,
            participants,
            impact,
            timestamp: 0.0, // Current time should be passed in
        });

        // Update relationships between participants
        for member1 in &participants {
            for member2 in &participants {
                if member1 != member2 {
                    let relationship = self.relationships.entry((*member1, *member2)).or_insert(0.0);
                    *relationship = (*relationship + impact * 0.1).clamp(-1.0, 1.0);
                }
            }
        }
    }

    fn update_cohesion(&mut self) {
        if self.members.is_empty() {
            self.cohesion = 0.0;
            return;
        }

        // Calculate average relationship strength
        let avg_relationship = if self.relationships.is_empty() {
            0.0
        } else {
            self.relationships.values().sum::<f32>() / self.relationships.len() as f32
        };

        // Calculate shared belief alignment
        let belief_alignment = self.calculate_belief_alignment();

        // Calculate recent activity participation
        let activity_participation = self.calculate_activity_participation();

        // Update cohesion based on all factors
        self.cohesion = (avg_relationship * 0.4 + belief_alignment * 0.3 + activity_participation * 0.3)
            .clamp(0.0, 1.0);
    }

    fn update_influence(&mut self) {
        self.influence = (self.cohesion * 0.6 + self.members.len() as f32 * 0.01).clamp(0.0, 1.0);
    }

    fn calculate_belief_alignment(&self) -> f32 {
        if self.shared_beliefs.is_empty() {
            return 0.5;
        }

        let total_alignment: f32 = self.shared_beliefs.iter()
            .map(|belief| {
                let supporter_ratio = belief.supporters.len() as f32 / self.members.len() as f32;
                supporter_ratio * belief.strength
            })
            .sum();

        (total_alignment / self.shared_beliefs.len() as f32).clamp(0.0, 1.0)
    }

    fn calculate_activity_participation(&self) -> f32 {
        if self.activities.is_empty() {
            return 0.0;
        }

        let recent_activities: Vec<_> = self.activities.iter()
            .rev()
            .take(5)
            .collect();

        let participation_rate: f32 = recent_activities.iter()
            .map(|activity| {
                (activity.participants.len() as f32 / self.members.len() as f32) * activity.impact
            })
            .sum();

        (participation_rate / recent_activities.len() as f32).clamp(0.0, 1.0)
    }

    fn cleanup_old_activities(&mut self, current_time: f32) {
        self.activities.retain(|activity| {
            current_time - activity.timestamp < 1000.0 // Keep activities from last 1000 time units
        });
    }
}