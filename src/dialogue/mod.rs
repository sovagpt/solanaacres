use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

pub mod generation;
pub mod context;
pub mod emotion;
pub mod deception;
pub mod memory_recall;

use context::DialogueContext;
use emotion::EmotionalState;
use deception::DeceptionSystem;
use memory_recall::MemoryRecall;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSystem {
    context: DialogueContext,
    emotion_state: EmotionalState,
    deception: DeceptionSystem,
    memory_recall: MemoryRecall,
    conversation_history: VecDeque<DialogueEntry>,
    active_topics: HashMap<String, f32>, // topic -> interest level
    participant_states: HashMap<Uuid, ParticipantState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueEntry {
    speaker: Uuid,
    content: String,
    emotion: String,
    intent: DialogueIntent,
    deception_level: f32,
    timestamp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantState {
    id: Uuid,
    trust_level: f32,
    emotional_stance: String,
    topics_of_interest: Vec<String>,
    last_interaction: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueIntent {
    Question,
    Statement,
    Response,
    Greeting,
    Farewell,
    Agreement,
    Disagreement,
    Clarification,
    Deception,
}

impl Default for DialogueSystem {
    fn default() -> Self {
        Self {
            context: DialogueContext::default(),
            emotion_state: EmotionalState::default(),
            deception: DeceptionSystem::default(),
            memory_recall: MemoryRecall::default(),
            conversation_history: VecDeque::with_capacity(100),
            active_topics: HashMap::new(),
            participant_states: HashMap::new(),
        }
    }
}

impl DialogueSystem {
    pub fn update(&mut self, delta_time: f32) {
        // Update conversation context
        self.context.update(delta_time);
        
        // Update emotional state
        self.emotion_state.update(delta_time);
        
        // Update deception system
        self.deception.update(delta_time);
        
        // Clean up old conversation history
        self.cleanup_history();
    }

    pub fn generate_response(
        &mut self,
        speaker_id: Uuid,
        message: &str,
        current_context: Option<String>,
    ) -> DialogueEntry {
        // Get emotional context
        let emotional_context = self.emotion_state.get_current_emotion();

        // Check for deception
        let deception_level = self.deception.should_deceive(message);

        // Recall relevant memories
        let memories = self.memory_recall.recall_relevant(message);

        // Generate response based on all factors
        let response = generation::generate_response(
            message,
            &self.context,
            &emotional_context,
            deception_level,
            &memories,
        );

        let entry = DialogueEntry {
            speaker: speaker_id,
            content: response,
            emotion: emotional_context,
            intent: self.determine_intent(message),
            deception_level,
            timestamp: 0.0, // Current time should be passed
        };

        // Update conversation history
        self.conversation_history.push_back(entry.clone());
        
        entry
    }

    pub fn add_participant(&mut self, id: Uuid) {
        let state = ParticipantState {
            id,
            trust_level: 0.5,
            emotional_stance: "neutral".to_string(),
            topics_of_interest: Vec::new(),
            last_interaction: 0.0,
        };

        self.participant_states.insert(id, state);
    }

    pub fn update_trust(&mut self, participant_id: Uuid, change: f32) {
        if let Some(state) = self.participant_states.get_mut(&participant_id) {
            state.trust_level = (state.trust_level + change).clamp(0.0, 1.0);
        }
    }

    fn determine_intent(&self, message: &str) -> DialogueIntent {
        if message.ends_with('?') {
            DialogueIntent::Question
        } else if message.to_lowercase().contains("hello") || message.to_lowercase().contains("hi") {
            DialogueIntent::Greeting
        } else if message.to_lowercase().contains("bye") || message.to_lowercase().contains("goodbye") {
            DialogueIntent::Farewell
        } else {
            DialogueIntent::Statement
        }
    }

    fn cleanup_history(&mut self) {
        while self.conversation_history.len() > 100 {
            self.conversation_history.pop_front();
        }
    }

    pub fn get_recent_messages(&self, count: usize) -> Vec<&DialogueEntry> {
        self.conversation_history
            .iter()
            .rev()
            .take(count)
            .collect()
    }
}