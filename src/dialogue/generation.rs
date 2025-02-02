use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::ai::dialogue::context::DialogueContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseGenerator {
    templates: HashMap<String, Vec<ResponseTemplate>>,
    topic_keywords: HashMap<String, Vec<String>>,
    personality_modifiers: HashMap<String, PersonalityModifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResponseTemplate {
    pattern: String,
    response: String,
    tone: String,
    requirements: Vec<String>,
    weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersonalityModifier {
    style_adjustments: HashMap<String, String>,
    vocabulary_level: u32,
    formality_level: f32,
}

pub fn generate_response(
    input: &str,
    context: &DialogueContext,
    emotional_state: &str,
    deception_level: f32,
    memories: &[String],
) -> String {
    let mut generator = ResponseGenerator::default();
    generator.generate(input, context, emotional_state, deception_level, memories)
}

impl Default for ResponseGenerator {
    fn default() -> Self {
        let mut templates = HashMap::new();
        let mut topic_keywords = HashMap::new();
        let mut personality_modifiers = HashMap::new();

        // Initialize with some basic templates
        initialize_basic_templates(&mut templates);
        initialize_topic_keywords(&mut topic_keywords);
        initialize_personality_modifiers(&mut personality_modifiers);

        Self {
            templates,
            topic_keywords,
            personality_modifiers,
        }
    }
}

impl ResponseGenerator {
    pub fn generate(
        &mut self,
        input: &str,
        context: &DialogueContext,
        emotional_state: &str,
        deception_level: f32,
        memories: &[String],
    ) -> String {
        // Identify topic
        let topic = self.identify_topic(input);
        
        // Get relevant templates
        let templates = self.get_relevant_templates(&topic, emotional_state);
        
        // Select best template
        let template = self.select_template(templates, context, deception_level);
        
        // Fill template with context and memories
        self.fill_template(template, context, memories)
    }

    fn identify_topic(&self, input: &str) -> String {
        for (topic, keywords) in &self.topic_keywords {
            if keywords.iter().any(|k| input.to_lowercase().contains(&k.to_lowercase())) {
                return topic.clone();
            }
        }
        "general".to_string()
    }

    fn get_relevant_templates(&self, topic: &str, emotional_state: &str) -> Vec<ResponseTemplate> {
        let mut templates = Vec::new();
        
        // Get topic-specific templates
        if let Some(topic_templates) = self.templates.get(topic) {
            templates.extend(topic_templates.clone());
        }
        
        // Add emotional templates
        if let Some(emotional_templates) = self.templates.get(emotional_state) {
            templates.extend(emotional_templates.clone());
        }
        
        // Add general templates if none found
        if templates.is_empty() {
            if let Some(general_templates) = self.templates.get("general") {
                templates.extend(general_templates.clone());
            }
        }
        
        templates
    }

    fn select_template(&self, templates: Vec<ResponseTemplate>, context: &DialogueContext, deception_level: f32) -> ResponseTemplate {
        let mut best_template = templates[0].clone();
        let mut best_score = 0.0;

        for template in templates {
            let context_score = self.calculate_context_score(&template, context);
            let deception_score = if deception_level > 0.5 {
                1.0 - template.weight // Prefer less direct templates when deceiving
            } else {
                template.weight
            };
            
            let score = context_score * 0.7 + deception_score * 0.3;
            
            if score > best_score {
                best_score = score;
                best_template = template;
            }
        }

        best_template
    }

    fn fill_template(&self, template: ResponseTemplate, context: &DialogueContext, memories: &[String]) -> String {
        let mut response = template.response;
        
        // Replace context placeholders
        for (key, value) in context.get_variables() {
            response = response.replace(&format!("{{{}}}", key), value);
        }
        
        // Insert memories if template has memory placeholders
        if response.contains("{memory}") && !memories.is_empty() {
            let memory = memories.first().unwrap();
            response = response.replace("{memory}", memory);
        }
        
        response
    }

    fn calculate_context_score(&self, template: &ResponseTemplate, context: &DialogueContext) -> f32 {
        let mut score = 0.0;
        
        // Check if template requirements are met in context
        for req in &template.requirements {
            if context.has_variable(req) {
                score += 1.0;
            }
        }
        
        // Normalize score
        if !template.requirements.is_empty() {
            score /= template.requirements.len() as f32;
        }
        
        score
    }
}

fn initialize_basic_templates(templates: &mut HashMap<String, Vec<ResponseTemplate>>) {
    // Add basic greeting templates
    templates.insert("greeting".to_string(), vec![
        ResponseTemplate {
            pattern: "hello".to_string(),
            response: "Hello! {context_greeting}".to_string(),
            tone: "friendly".to_string(),
            requirements: vec![],
            weight: 1.0,
        },
    ]);
    
    // Add basic response templates
    templates.insert("general".to_string(), vec![
        ResponseTemplate {
            pattern: "".to_string(),
            response: "I understand. Please tell me more.".to_string(),
            tone: "neutral".to_string(),
            requirements: vec![],
            weight: 0.5,
        },
    ]);
}

fn initialize_topic_keywords(keywords: &mut HashMap<String, Vec<String>>) {
    keywords.insert("greeting".to_string(), vec![
        "hello".to_string(),
        "hi".to_string(),
        "hey".to_string(),
    ]);
    
    keywords.insert("farewell".to_string(), vec![
        "goodbye".to_string(),
        "bye".to_string(),
        "see you".to_string(),
    ]);
}

fn initialize_personality_modifiers(modifiers: &mut HashMap<String, PersonalityModifier>) {
    modifiers.insert("formal".to_string(), PersonalityModifier {
        style_adjustments: HashMap::new(),
        vocabulary_level: 3,
        formality_level: 0.8,
    });
}