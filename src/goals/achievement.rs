use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementTracker {
    achievements: HashMap<Uuid, Achievement>,
    completed_goals: HashSet<Uuid>,
    milestones: HashMap<String, Milestone>,
    progress_history: Vec<ProgressEvent>,
    achievement_stats: AchievementStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    id: Uuid,
    name: String,
    description: String,
    difficulty: f32,
    requirements: Vec<Requirement>,
    reward_factor: f32,
    completion_date: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    name: String,
    threshold: f32,
    current_progress: f32,
    achieved: bool,
    achievement_date: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    description: String,
    required_value: f32,
    current_value: f32,
    completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    goal_id: Uuid,
    achievement_id: Option<Uuid>,
    event_type: ProgressType,
    value: f32,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressType {
    GoalCompletion,
    MilestoneReached,
    RequirementMet,
    Achievement,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AchievementStats {
    total_completed: u32,
    total_attempted: u32,
    success_rate: f32,
    average_completion_time: f32,
    highest_difficulty_achieved: f32,
}

impl Default for AchievementTracker {
    fn default() -> Self {
        Self {
            achievements: HashMap::new(),
            completed_goals: HashSet::new(),
            milestones: HashMap::new(),
            progress_history: Vec::new(),
            achievement_stats: AchievementStats::default(),
        }
    }
}

impl AchievementTracker {
    pub fn update(&mut self, delta_time: f32) {
        // Update progress on all achievements
        for achievement in self.achievements.values_mut() {
            self.update_achievement_progress(achievement);
        }

        // Update milestones
        for milestone in self.milestones.values_mut() {
            self.check_milestone_completion(milestone);
        }

        // Update statistics
        self.update_stats();
    }

    pub fn track_completion(&mut self, goal_id: Uuid, description: &str) {
        self.completed_goals.insert(goal_id);

        let event = ProgressEvent {
            goal_id,
            achievement_id: None,
            event_type: ProgressType::GoalCompletion,
            value: 1.0,
            timestamp: 0.0, // Current time should be passed
        };

        self.progress_history.push(event);
        self.check_achievements_for_goal(goal_id);
    }

    pub fn add_achievement(&mut self, name: String, description: String, difficulty: f32, requirements: Vec<Requirement>) {
        let achievement = Achievement {
            id: Uuid::new_v4(),
            name,
            description,
            difficulty,
            requirements,
            reward_factor: calculate_reward_factor(difficulty),
            completion_date: None,
        };

        self.achievements.insert(achievement.id, achievement);
    }

    pub fn add_milestone(&mut self, name: String, threshold: f32) {
        let milestone = Milestone {
            name: name.clone(),
            threshold,
            current_progress: 0.0,
            achieved: false,
            achievement_date: None,
        };

        self.milestones.insert(name, milestone);
    }

    pub fn update_progress(&mut self, goal_id: Uuid, value: f32) {
        // Record progress
        let event = ProgressEvent {
            goal_id,
            achievement_id: None,
            event_type: ProgressType::GoalCompletion,
            value,
            timestamp: 0.0,
        };

        self.progress_history.push(event);

        // Update related milestones
        for milestone in self.milestones.values_mut() {
            milestone.current_progress += value;
            self.check_milestone_completion(milestone);
        }
    }

    fn update_achievement_progress(&mut self, achievement: &mut Achievement) {
        let total_requirements = achievement.requirements.len();
        let completed_requirements = achievement.requirements
            .iter()
            .filter(|r| r.completed)
            .count();

        if total_requirements > 0 && completed_requirements == total_requirements {
            if achievement.completion_date.is_none() {
                achievement.completion_date = Some(0.0); // Current time should be passed

                let event = ProgressEvent {
                    goal_id: Uuid::new_v4(), // placeholder
                    achievement_id: Some(achievement.id),
                    event_type: ProgressType::Achievement,
                    value: 1.0,
                    timestamp: 0.0,
                };

                self.progress_history.push(event);
            }
        }
    }

    fn check_milestone_completion(&mut self, milestone: &mut Milestone) {
        if !milestone.achieved && milestone.current_progress >= milestone.threshold {
            milestone.achieved = true;
            milestone.achievement_date = Some(0.0); // Current time should be passed

            let event = ProgressEvent {
                goal_id: Uuid::new_v4(), // placeholder
                achievement_id: None,
                event_type: ProgressType::MilestoneReached,
                value: milestone.threshold,
                timestamp: 0.0,
            };

            self.progress_history.push(event);
        }
    }

    fn check_achievements_for_goal(&mut self, goal_id: Uuid) {
        for achievement in self.achievements.values_mut() {
            for requirement in &mut achievement.requirements {
                if !requirement.completed {
                    requirement.current_value += 1.0;
                    if requirement.current_value >= requirement.required_value {
                        requirement.completed = true;

                        let event = ProgressEvent {
                            goal_id,
                            achievement_id: Some(achievement.id),
                            event_type: ProgressType::RequirementMet,
                            value: 1.0,
                            timestamp: 0.0,
                        };

                        self.progress_history.push(event);
                    }
                }
            }
        }
    }

    fn update_stats(&mut self) {
        let completed = self.achievements.values()
            .filter(|a| a.completion_date.is_some())
            .count() as u32;

        self.achievement_stats.total_completed = completed;
        self.achievement_stats.total_attempted = self.achievements.len() as u32;
        
        self.achievement_stats.success_rate = if self.achievement_stats.total_attempted > 0 {
            completed as f32 / self.achievement_stats.total_attempted as f32
        } else {
            0.0
        };

        // Update highest difficulty achieved
        if let Some(max_difficulty) = self.achievements.values()
            .filter(|a| a.completion_date.is_some())
            .map(|a| a.difficulty)
            .max_by(|a, b| a.partial_cmp(b).unwrap()) {
            self.achievement_stats.highest_difficulty_achieved = max_difficulty;
        }
    }
}

fn calculate_reward_factor(difficulty: f32) -> f32 {
    // Higher difficulty yields higher rewards
    (1.0 + difficulty).powf(1.5)
}