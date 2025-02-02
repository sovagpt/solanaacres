use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod beliefs;
pub mod learning;
pub mod sharing;

use beliefs::BeliefSystem;
use learning::LearningSystem;
use sharing::KnowledgeSharing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    beliefs: BeliefSystem,
    learning: LearningSystem,
    sharing: KnowledgeSharing,
    known_facts: HashMap<String, KnowledgeFact>,
    knowledge_connections: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeFact {
    content: String,
    certainty: f32,
    source: Option<Uuid>,
    timestamp: f32,
    importance: f32,
    category: String,
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self {
            beliefs: BeliefSystem::default(),
            learning: LearningSystem::default(),
            sharing: KnowledgeSharing::default(),
            known_facts: HashMap::new(),
            knowledge_connections: HashMap::new(),
        }
    }
}

impl KnowledgeBase {
    pub fn update(&mut self, delta_time: f32) {
        // Update belief system
        self.beliefs.update(delta_time);
        
        // Process learning
        self.learning.update(delta_time);
        
        // Update knowledge sharing status
        self.sharing.update(delta_time);
    }

    pub fn add_knowledge(&mut self, content: String, certainty: f32, source: Option<Uuid>, category: String) {
        let fact = KnowledgeFact {
            content: content.clone(),
            certainty,
            source,
            timestamp: 0.0, // Current time should be passed
            importance: self.calculate_importance(&content),
            category,
        };

        self.known_facts.insert(content.clone(), fact);
        self.connect_related_knowledge(&content);
    }

    pub fn query_knowledge(&self, query: &str) -> Vec<&KnowledgeFact> {
        self.known_facts
            .values()
            .filter(|fact| fact.content.contains(query))
            .collect()
    }

    pub fn get_knowledge_by_category(&self, category: &str) -> Vec<&KnowledgeFact> {
        self.known_facts
            .values()
            .filter(|fact| fact.category == category)
            .collect()
    }

    pub fn merge_knowledge(&mut self, other: &KnowledgeBase, trust_factor: f32) {
        for (content, fact) in &other.known_facts {
            if let Some(existing_fact) = self.known_facts.get_mut(content) {
                // Update existing knowledge
                existing_fact.certainty = 
                    (existing_fact.certainty + fact.certainty * trust_factor) / 2.0;
                existing_fact.importance =
                    (existing_fact.importance + fact.importance) / 2.0;
            } else {
                // Add new knowledge with adjusted certainty
                let mut new_fact = fact.clone();
                new_fact.certainty *= trust_factor;
                self.known_facts.insert(content.clone(), new_fact);
            }
        }
    }

    fn calculate_importance(&self, content: &str) -> f32 {
        // Basic importance calculation based on content length and connections
        let base_importance = content.len() as f32 / 100.0;
        let connection_factor = self.knowledge_connections
            .get(content)
            .map_or(0.0, |connections| connections.len() as f32 / 10.0);
        
        (base_importance + connection_factor).clamp(0.0, 1.0)
    }

    fn connect_related_knowledge(&mut self, content: &str) {
        let related: Vec<String> = self.known_facts
            .keys()
            .filter(|&k| k != content && self.are_related(k, content))
            .cloned()
            .collect();

        // Add connections both ways
        for related_content in &related {
            self.knowledge_connections
                .entry(content.to_string())
                .or_default()
                .push(related_content.clone());

            self.knowledge_connections
                .entry(related_content.clone())
                .or_default()
                .push(content.to_string());
        }
    }

    fn are_related(&self, content1: &str, content2: &str) -> bool {
        // Simple word matching for now
        let words1: Vec<&str> = content1.split_whitespace().collect();
        let words2: Vec<&str> = content2.split_whitespace().collect();
        
        words1.iter()
            .any(|word| words2.contains(word))
    }
}