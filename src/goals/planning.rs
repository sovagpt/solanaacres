use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planner {
    plans: HashMap<Uuid, Plan>,
    action_templates: HashMap<String, ActionTemplate>,
    current_plan: Option<Uuid>,
    planning_horizon: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    id: Uuid,
    goal_id: Uuid,
    steps: VecDeque<PlanStep>,
    status: PlanStatus,
    estimated_completion_time: f32,
    fallback_plans: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    action: String,
    prerequisites: Vec<String>,
    expected_duration: f32,
    completed: bool,
    outcome: Option<StepOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTemplate {
    name: String,
    prerequisites: Vec<String>,
    effects: Vec<String>,
    average_duration: f32,
    success_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanStatus {
    InProgress,
    Completed,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepOutcome {
    Success,
    Failure(String),
    Partial(f32),
}

impl Default for Planner {
    fn default() -> Self {
        Self {
            plans: HashMap::new(),
            action_templates: HashMap::new(),
            current_plan: None,
            planning_horizon: 100.0,
        }
    }
}

impl Planner {
    pub fn create_plan(&mut self, goal_id: Uuid) -> Uuid {
        let plan = Plan {
            id: Uuid::new_v4(),
            goal_id,
            steps: VecDeque::new(),
            status: PlanStatus::InProgress,
            estimated_completion_time: 0.0,
            fallback_plans: Vec::new(),
        };

        let plan_id = plan.id;
        self.plans.insert(plan_id, plan);
        plan_id
    }

    pub fn add_action_template(&mut self, template: ActionTemplate) {
        self.action_templates.insert(template.name.clone(), template);
    }

    pub fn generate_steps(&mut self, plan_id: Uuid, goal_state: &str) -> bool {
        if let Some(plan) = self.plans.get_mut(&plan_id) {
            // Clear existing steps
            plan.steps.clear();

            // Find path to goal state using available actions
            let steps = self.plan_path_to_goal(goal_state);
            if let Some(steps) = steps {
                plan.steps = steps;
                plan.estimate_completion_time();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn execute_next_step(&mut self, plan_id: Uuid) -> Option<StepOutcome> {
        if let Some(plan) = self.plans.get_mut(&plan_id) {
            if let Some(step) = plan.steps.front_mut() {
                if !self.check_prerequisites(&step.prerequisites) {
                    return Some(StepOutcome::Failure("Prerequisites not met".to_string()));
                }

                // Simulate action execution
                let success_rate = self.get_action_success_rate(&step.action);
                if rand::random::<f32>() < success_rate {
                    step.completed = true;
                    step.outcome = Some(StepOutcome::Success);
                    
                    // Remove completed step
                    plan.steps.pop_front();
                    
                    // Update plan status
                    if plan.steps.is_empty() {
                        plan.status = PlanStatus::Completed;
                    }

                    Some(StepOutcome::Success)
                } else {
                    Some(StepOutcome::Failure("Action failed".to_string()))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn plan_path_to_goal(&self, goal_state: &str) -> Option<VecDeque<PlanStep>> {
        // Simplified planning for now - could be expanded with proper GOAP
        let mut steps = VecDeque::new();
        
        // Find actions that lead to goal state
        if let Some(action) = self.find_action_for_effect(goal_state) {
            steps.push_back(PlanStep {
                action: action.name,
                prerequisites: action.prerequisites,
                expected_duration: action.average_duration,
                completed: false,
                outcome: None,
            });

            // Add prerequisite actions
            for prereq in &action.prerequisites {
                if let Some(prereq_action) = self.find_action_for_effect(prereq) {
                    steps.push_front(PlanStep {
                        action: prereq_action.name,
                        prerequisites: prereq_action.prerequisites,
                        expected_duration: prereq_action.average_duration,
                        completed: false,
                        outcome: None,
                    });
                }
            }
            Some(steps)
        } else {
            None
        }
    }

    fn find_action_for_effect(&self, effect: &str) -> Option<&ActionTemplate> {
        self.action_templates.values()
            .find(|template| template.effects.iter().any(|e| e == effect))
    }

    fn check_prerequisites(&self, prerequisites: &[String]) -> bool {
        // In a real implementation, this would check the current world state
        true
    }

    fn get_action_success_rate(&self, action_name: &str) -> f32 {
        self.action_templates
            .get(action_name)
            .map_or(0.5, |template| template.success_rate)
    }
}

impl Plan {
    fn estimate_completion_time(&mut self) {
        self.estimated_completion_time = self.steps.iter()
            .map(|step| step.expected_duration)
            .sum();
    }
}