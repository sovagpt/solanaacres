use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSharing {
    shared_knowledge: HashMap<String, SharedKnowledge>,
    teaching_history: HashMap<Uuid, TeachingHistory>,
    learning_preferences: HashSet<String>,
    sharing_willingness: f32,
    teaching_ability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedKnowledge {
    content: String,
    source: Option<Uuid>,
    recipients: HashSet<Uuid>,
    success_rate: f32,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TeachingHistory {
    successful_teachings: u32,
    failed_teachings: u32,
    student_feedback: Vec<TeachingFeedback>,
    last_teaching: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TeachingFeedback {
    student_id: Uuid,
    success_rate: f32,
    comments: String,
    timestamp: f32,
}

impl Default for KnowledgeSharing {
    fn default() -> Self {
        Self {
            shared_knowledge: HashMap::new(),
            teaching_history: HashMap::new(),
            learning_preferences: HashSet::new(),
            sharing_willingness: 0.5,
            teaching_ability: 0.5,
        }
    }
}

impl KnowledgeSharing {
    pub fn update(&mut self, delta_time: f32) {
        // Update teaching ability based on history
        self.update_teaching_ability();
        
        // Clean up old shared knowledge
        self.cleanup_old_knowledge(delta_time);
    }

    pub fn share_knowledge(
        &mut self,
        content: String,
        source: Option<Uuid>,
        recipient: Uuid,
    ) -> bool {
        let success_chance = self.calculate_sharing_success(recipient);
        let success = rand::random::<f32>() < success_chance;

        let shared = SharedKnowledge {
            content: content.clone(),
            source,
            recipients: HashSet::from([recipient]),
            success_rate: if success { 1.0 } else { 0.0 },
            timestamp: 0.0, // Current time should be passed
        };

        self.shared_knowledge.insert(content, shared);
        
        // Update teaching history
        if let Some(source_id) = source {
            self.update_teaching_history(source_id, success);
        }

        success
    }

    pub fn receive_feedback(
        &mut self,
        teacher_id: Uuid,
        student_id: Uuid,
        success_rate: f32,
        comments: String,
    ) {
        let feedback = TeachingFeedback {
            student_id,
            success_rate,
            comments,
            timestamp: 0.0, // Current time should be passed
        };

        if let Some(history) = self.teaching_history.get_mut(&teacher_id) {
            history.student_feedback.push(feedback);
            
            // Update teaching stats
            if success_rate > 0.7 {
                history.successful_teachings += 1;
            } else {
                history.failed_teachings += 1;
            }
        }
    }

    pub fn add_learning_preference(&mut self, preference: String) {
        self.learning_preferences.insert(preference);
    }

    pub fn get_teaching_effectiveness(&self, teacher_id: &Uuid) -> Option<f32> {
        self.teaching_history.get(teacher_id).map(|history| {
            let total = history.successful_teachings + history.failed_teachings;
            if total == 0 {
                0.5
            } else {
                history.successful_teachings as f32 / total as f32
            }
        })
    }

    fn calculate_sharing_success(&self, recipient: Uuid) -> f32 {
        // Base success chance from teaching ability
        let base_chance = self.teaching_ability;
        
        // Modify based on sharing willingness
        let willingness_factor = self.sharing_willingness;
        
        // Consider past success with this recipient
        let history_factor = self.get_recipient_success_rate(recipient);
        
        (base_chance * 0.4 + willingness_factor * 0.3 + history_factor * 0.3).clamp(0.1, 0.9)
    }

    fn get_recipient_success_rate(&self, recipient: Uuid) -> f32 {
        let mut successful = 0;
        let mut total = 0;

        for shared in self.shared_knowledge.values() {
            if shared.recipients.contains(&recipient) {
                total += 1;
                if shared.success_rate > 0.7 {
                    successful += 1;
                }
            }
        }

        if total == 0 {
            0.5
        } else {
            successful as f32 / total as f32
        }
    }

    fn update_teaching_ability(&mut self) {
        let mut total_success_rate = 0.0;
        let mut count = 0;

        for history in self.teaching_history.values() {
            if !history.student_feedback.is_empty() {
                total_success_rate += history.student_feedback.iter()
                    .map(|f| f.success_rate)
                    .sum::<f32>() / history.student_feedback.len() as f32;
                count += 1;
            }
        }

        if count > 0 {
            self.teaching_ability = (self.teaching_ability * 0.7 + 
                (total_success_rate / count as f32) * 0.3).clamp(0.0, 1.0);
        }
    }

    fn cleanup_old_knowledge(&mut self, current_time: f32) {
        const KNOWLEDGE_RETENTION_TIME: f32 = 1000.0;
        self.shared_knowledge.retain(|_, knowledge| {
            current_time - knowledge.timestamp < KNOWLEDGE_RETENTION_TIME
        });
    }
}