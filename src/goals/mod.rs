use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub mod planning;
pub mod desires;
pub mod motivation;
pub mod achievement;

use planning::Planner;
use desires::DesireSystem;
use motivation::MotivationSystem;
use achievement::AchievementTracker;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalSystem {
    planner: Planner,
    desires: DesireSystem,
    motivation: MotivationSystem,
    achievement: AchievementTracker,
    active_goals: HashMap<Uuid, Goal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    id: Uuid,
    description: String,
    priority: f32,
    deadline: Option<f32>,
    status: GoalStatus,
    dependencies: HashSet<Uuid>,
    subgoals: Vec<Uuid>,
    progress: f32,
    motivation_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GoalStatus {
    Active,
    Completed,
    Failed,
    OnHold,
    Abandoned,
}

impl Default for GoalSystem {
    fn default() -> Self {
        Self {
            planner: Planner::default(),
            desires: DesireSystem::default(),
            motivation: MotivationSystem::default(),
            achievement: AchievementTracker::default(),
            active_goals: HashMap::new(),
        }
    }
}

impl GoalSystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update desire system
        self.desires.update(delta_time);
        
        // Update motivation system
        self.motivation.update(delta_time);
        
        // Update goals and their progress
        self.update_goals(delta_time);
        
        // Track achievements
        self.achievement.update(delta_time);
    }

    pub fn create_goal(&mut self, description: String, priority: f32, deadline: Option<f32>) -> Uuid {
        let goal = Goal {
            id: Uuid::new_v4(),
            description,
            priority,
            deadline,
            status: GoalStatus::Active,
            dependencies: HashSet::new(),
            subgoals: Vec::new(),
            progress: 0.0,
            motivation_level: self.motivation.calculate_initial_motivation(priority),
        };

        let id = goal.id;
        self.active_goals.insert(id, goal);
        id
    }

    pub fn add_dependency(&mut self, goal_id: Uuid, dependency_id: Uuid) -> bool {
        if let Some(goal) = self.active_goals.get_mut(&goal_id) {
            goal.dependencies.insert(dependency_id);
            true
        } else {
            false
        }
    }

    pub fn add_subgoal(&mut self, parent_id: Uuid, description: String, priority: f32) -> Option<Uuid> {
        if self.active_goals.contains_key(&parent_id) {
            let subgoal_id = self.create_goal(description, priority, None);
            if let Some(parent) = self.active_goals.get_mut(&parent_id) {
                parent.subgoals.push(subgoal_id);
            }
            Some(subgoal_id)
        } else {
            None
        }
    }

    pub fn update_progress(&mut self, goal_id: Uuid, progress: f32) {
        if let Some(goal) = self.active_goals.get_mut(&goal_id) {
            goal.progress = progress.clamp(0.0, 1.0);
            
            if goal.progress >= 1.0 {
                goal.status = GoalStatus::Completed;
                self.achievement.track_completion(goal_id, &goal.description);
            }
        }
    }

    fn update_goals(&mut self, delta_time: f32) {
        for goal in self.active_goals.values_mut() {
            // Update motivation level
            goal.motivation_level = self.motivation.calculate_current_motivation(
                goal.priority,
                goal.progress,
                delta_time
            );

            // Check deadlines
            if let Some(deadline) = goal.deadline {
                if deadline <= 0.0 && goal.status == GoalStatus::Active {
                    goal.status = GoalStatus::Failed;
                }
            }

            // Update progress of parent goals based on subgoals
            if !goal.subgoals.is_empty() {
                self.update_parent_progress(goal);
            }
        }
    }

    fn update_parent_progress(&mut self, parent: &mut Goal) {
        let mut total_progress = 0.0;
        let mut completed_subgoals = 0;

        for subgoal_id in &parent.subgoals {
            if let Some(subgoal) = self.active_goals.get(subgoal_id) {
                total_progress += subgoal.progress;
                if subgoal.status == GoalStatus::Completed {
                    completed_subgoals += 1;
                }
            }
        }

        parent.progress = total_progress / parent.subgoals.len() as f32;

        if completed_subgoals == parent.subgoals.len() {
            parent.status = GoalStatus::Completed;
            self.achievement.track_completion(parent.id, &parent.description);
        }
    }

    pub fn get_active_goals(&self) -> Vec<&Goal> {
        self.active_goals
            .values()
            .filter(|g| g.status == GoalStatus::Active)
            .collect()
    }

    pub fn get_goal_status(&self, goal_id: Uuid) -> Option<GoalStatus> {
        self.active_goals.get(&goal_id).map(|g| g.status.clone())
    }

    pub fn abandon_goal(&mut self, goal_id: Uuid) {
        if let Some(goal) = self.active_goals.get_mut(&goal_id) {
            goal.status = GoalStatus::Abandoned;
            
            // Also abandon subgoals
            for subgoal_id in &goal.subgoals {
                self.abandon_goal(*subgoal_id);
            }
        }
    }
}