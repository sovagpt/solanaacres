use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSystem {
    experiences: VecDeque<LearningExperience>,
    learned_skills: HashMap<String, Skill>,
    learning_rate: f32,
    curiosity: f32,
    understanding_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningExperience {
    content: String,
    source: Option<Uuid>,
    timestamp: f32,
    importance: f32,
    understanding: f32,
    repetitions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    name: String,
    proficiency: f32,
    practice_count: u32,
    related_experiences: Vec<String>,
    last_practice: f32,
}

impl LearningSystem {
    pub fn get_skill_proficiency(&self, skill_name: &str) -> Option<f32> {
        self.learned_skills.get(skill_name).map(|s| s.proficiency)
    }

    pub fn get_recent_experiences(&self, count: usize) -> Vec<&LearningExperience> {
        self.experiences.iter().rev().take(count).collect()
    }

    pub fn get_understanding_level(&self, content: &str) -> f32 {
        self.experiences.iter()
            .find(|e| e.content == content)
            .map_or(0.0, |e| e.understanding)
    }

    fn process_experiences(&mut self, delta_time: f32) {
        for experience in &mut self.experiences {
            // Increase understanding based on repetitions and time spent
            if experience.repetitions > 0 {
                let understanding_gain = 0.1 * 
                    (experience.repetitions as f32).sqrt() * 
                    self.learning_rate;
                
                experience.understanding = 
                    (experience.understanding + understanding_gain).min(1.0);
            }

            // Connect experiences to skills
            self.connect_experience_to_skills(&experience.content);
        }
    }

    fn update_skills(&mut self, delta_time: f32) {
        for skill in self.learned_skills.values_mut() {
            // Decay skill proficiency if not practiced
            if skill.last_practice > 100.0 { // 100 time units without practice
                skill.proficiency *= 0.999f32.powf(delta_time);
            }

            // Update skill based on related experiences
            let experience_bonus = skill.related_experiences.len() as f32 * 0.01;
            skill.proficiency = (skill.proficiency + experience_bonus).min(1.0);
        }
    }

    fn adjust_learning_rate(&mut self) {
        // Adjust learning rate based on overall understanding
        let avg_understanding: f32 = if self.experiences.is_empty() {
            0.5
        } else {
            self.experiences.iter()
                .map(|e| e.understanding)
                .sum::<f32>() / self.experiences.len() as f32
        };

        // If understanding is high, slightly decrease learning rate
        if avg_understanding > self.understanding_threshold {
            self.learning_rate *= 0.99;
        } else {
            // If understanding is low, slightly increase learning rate
            self.learning_rate *= 1.01;
        }

        // Clamp learning rate
        self.learning_rate = self.learning_rate.clamp(0.1, 1.0);
    }

    fn find_similar_experience(&mut self, content: &str) -> Option<&mut LearningExperience> {
        self.experiences.iter_mut()
            .find(|e| self.calculate_content_similarity(&e.content, content) > 0.8)
    }

    fn calculate_content_similarity(&self, content1: &str, content2: &str) -> f32 {
        // Simple word overlap similarity
        let words1: Vec<&str> = content1.split_whitespace().collect();
        let words2: Vec<&str> = content2.split_whitespace().collect();

        let common_words = words1.iter()
            .filter(|w| words2.contains(w))
            .count();

        let max_words = words1.len().max(words2.len());
        if max_words == 0 {
            0.0
        } else {
            common_words as f32 / max_words as f32
        }
    }

    fn connect_experience_to_skills(&mut self, content: &str) {
        for skill in self.learned_skills.values_mut() {
            if content.to_lowercase().contains(&skill.name.to_lowercase()) {
                if !skill.related_experiences.contains(&content.to_string()) {
                    skill.related_experiences.push(content.to_string());
                }
            }
        }
    }

    pub fn get_learning_statistics(&self) -> LearningStatistics {
        LearningStatistics {
            total_experiences: self.experiences.len(),
            average_understanding: self.calculate_average_understanding(),
            mastered_skills: self.count_mastered_skills(),
            learning_rate: self.learning_rate,
            curiosity_level: self.curiosity,
        }
    }

    fn calculate_average_understanding(&self) -> f32 {
        if self.experiences.is_empty() {
            0.0
        } else {
            self.experiences.iter()
                .map(|e| e.understanding)
                .sum::<f32>() / self.experiences.len() as f32
        }
    }

    fn count_mastered_skills(&self) -> usize {
        self.learned_skills.values()
            .filter(|s| s.proficiency > 0.9)
            .count()
    }
}

#[derive(Debug, Clone)]
pub struct LearningStatistics {
    pub total_experiences: usize,
    pub average_understanding: f32,
    pub mastered_skills: usize,
    pub learning_rate: f32,
    pub curiosity_level: f32,
}