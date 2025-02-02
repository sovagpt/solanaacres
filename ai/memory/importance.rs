use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportanceScoring {
    keyword_weights: HashMap<String, f32>,
    emotional_weight: f32,
    novelty_weight: f32,
    context_weight: f32,
    base_importance: f32,
}

impl Default for ImportanceScoring {
    fn default() -> Self {
        let mut keyword_weights = HashMap::new();
        
        // Initialize with some default important keywords
        keyword_weights.insert("death".to_string(), 0.9);
        keyword_weights.insert("danger".to_string(), 0.8);
        keyword_weights.insert("important".to_string(), 0.7);
        keyword_weights.insert("secret".to_string(), 0.7);
        keyword_weights.insert("remember".to_string(), 0.6);
        keyword_weights.insert("never".to_string(), 0.6);
        keyword_weights.insert("always".to_string(), 0.5);
        keyword_weights.insert("promise".to_string(), 0.5);

        Self {
            keyword_weights,
            emotional_weight: 0.3,
            novelty_weight: 0.2,
            context_weight: 0.2,
            base_importance: 0.1,
        }
    }
}

impl ImportanceScoring {
    pub fn calculate_importance(&self, content: &str, emotional_value: f32) -> f32 {
        let mut importance = self.base_importance;

        // Add keyword-based importance
        importance += self.calculate_keyword_importance(content);

        // Add emotional importance
        importance += emotional_value.abs() * self.emotional_weight;

        // Clamp final value between 0 and 1
        importance.clamp(0.0, 1.0)
    }

    pub fn calculate_with_context(&self, content: &str, emotional_value: f32, context_relevance: f32, novelty: f32) -> f32 {
        let mut importance = self.calculate_importance(content, emotional_value);

        // Add context importance
        importance += context_relevance * self.context_weight;

        // Add novelty importance
        importance += novelty * self.novelty_weight;

        importance.clamp(0.0, 1.0)
    }

    fn calculate_keyword_importance(&self, content: &str) -> f32 {
        let content = content.to_lowercase();
        let mut importance = 0.0;
        let mut matches = 0;

        for (keyword, weight) in &self.keyword_weights {
            if content.contains(&keyword.to_lowercase()) {
                importance += weight;
                matches += 1;
            }
        }

        if matches > 0 {
            importance / matches as f32
        } else {
            0.0
        }
    }

    pub fn add_keyword(&mut self, keyword: String, weight: f32) {
        self.keyword_weights.insert(keyword, weight.clamp(0.0, 1.0));
    }

    pub fn remove_keyword(&mut self, keyword: &str) {
        self.keyword_weights.remove(keyword);
    }

    pub fn adjust_weights(&mut self, 
        emotional: Option<f32>,
        novelty: Option<f32>,
        context: Option<f32>,
        base: Option<f32>
    ) {
        if let Some(emotional) = emotional {
            self.emotional_weight = emotional.clamp(0.0, 1.0);
        }
        if let Some(novelty) = novelty {
            self.novelty_weight = novelty.clamp(0.0, 1.0);
        }
        if let Some(context) = context {
            self.context_weight = context.clamp(0.0, 1.0);
        }
        if let Some(base) = base {
            self.base_importance = base.clamp(0.0, 1.0);
        }
    }

    pub fn analyze_emotional_patterns(&self, content: &str) -> EmotionalPattern {
        EmotionalPattern {
            positive_keywords: self.count_emotional_keywords(content, true),
            negative_keywords: self.count_emotional_keywords(content, false),
            intensity: self.calculate_emotional_intensity(content),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPattern {
    positive_keywords: usize,
    negative_keywords: usize,
    intensity: f32,
}

impl ImportanceScoring {
    fn count_emotional_keywords(&self, content: &str, positive: bool) -> usize {
        let content = content.to_lowercase();
        let keywords = if positive {
            vec!["happy", "joy", "love", "excited", "wonderful"]
        } else {
            vec!["sad", "angry", "fear", "hate", "terrible"]
        };

        keywords.iter().filter(|&k| content.contains(k)).count()
    }

    fn calculate_emotional_intensity(&self, content: &str) -> f32 {
        let intensity_keywords = ["very", "extremely", "absolutely", "totally"];
        let content = content.to_lowercase();
        
        let matches = intensity_keywords.iter()
            .filter(|&k| content.contains(k))
            .count();

        (matches as f32 * 0.25).clamp(0.0, 1.0)
    }
}