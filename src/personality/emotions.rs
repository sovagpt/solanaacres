use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    current_emotions: HashMap<Emotion, f32>,
    baseline_mood: f32,
    emotional_inertia: f32,
    volatility: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Emotion {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Trust,
    Disgust,
    Anticipation,
    Confusion,
    Curiosity,
}

impl Default for EmotionalState {
    fn default() -> Self {
        let mut current_emotions = HashMap::new();
        for emotion in [
            Emotion::Joy,
            Emotion::Sadness,
            Emotion::Anger,
            Emotion::Fear,
            Emotion::Surprise,
            Emotion::Trust,
            Emotion::Disgust,
            Emotion::Anticipation,
            Emotion::Confusion,
            Emotion::Curiosity,
        ] {
            current_emotions.insert(emotion, 0.0);
        }

        Self {
            current_emotions,
            baseline_mood: 0.5,
            emotional_inertia: 0.5,
            volatility: 0.1,
        }
    }
}

impl EmotionalState {
    pub fn update(&mut self, delta_time: f32) {
        // Decay all emotions towards baseline
        for intensity in self.current_emotions.values_mut() {
            let diff = self.baseline_mood - *intensity;
            *intensity += diff * delta_time * (1.0 - self.emotional_inertia);
        }
        
        // Update volatility based on emotional changes
        self.update_volatility();
    }

    pub fn generate_response(&mut self, event: &str, intensity: f32) -> Emotion {
        let emotion = self.analyze_event(event);
        self.add_emotion(emotion, intensity);
        emotion
    }

    pub fn add_emotion(&mut self, emotion: Emotion, intensity: f32) {
        if let Some(current) = self.current_emotions.get_mut(&emotion) {
            *current = (*current + intensity).clamp(0.0, 1.0);
        }
    }

    pub fn get_dominant_emotion(&self) -> Emotion {
        self.current_emotions
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(emotion, _)| *emotion)
            .unwrap_or(Emotion::Joy)
    }

    pub fn get_emotional_valence(&self) -> f32 {
        let positive = [Emotion::Joy, Emotion::Trust, Emotion::Anticipation, Emotion::Curiosity];
        let negative = [Emotion::Sadness, Emotion::Anger, Emotion::Fear, Emotion::Disgust];

        let positive_sum: f32 = positive.iter()
            .filter_map(|e| self.current_emotions.get(e))
            .sum();
        let negative_sum: f32 = negative.iter()
            .filter_map(|e| self.current_emotions.get(e))
            .sum();

        (positive_sum - negative_sum) / (positive.len() + negative.len()) as f32
    }

    pub fn get_volatility(&self) -> f32 {
        self.volatility
    }

    fn analyze_event(&self, event: &str) -> Emotion {
        // Simple event analysis - could be made more sophisticated
        match event {
            e if e.contains("danger") || e.contains("threat") => Emotion::Fear,
            e if e.contains("happy") || e.contains("success") => Emotion::Joy,
            e if e.contains("angry") || e.contains("unfair") => Emotion::Anger,
            e if e.contains("sad") || e.contains("loss") => Emotion::Sadness,
            e if e.contains("trust") || e.contains("friend") => Emotion::Trust,
            e if e.contains("disgust") || e.contains("gross") => Emotion::Disgust,
            e if e.contains("surprise") || e.contains("unexpected") => Emotion::Surprise,
            e if e.contains("anticipate") || e.contains("expect") => Emotion::Anticipation,
            e if e.contains("confused") || e.contains("unclear") => Emotion::Confusion,
            e if e.contains("curious") || e.contains("interesting") => Emotion::Curiosity,
            _ => Emotion::Surprise,
        }
    }

    fn update_volatility(&mut self) {
        let emotion_variance: f32 = self.current_emotions.values()
            .map(|&intensity| (intensity - self.baseline_mood).powi(2))
            .sum();
        
        self.volatility = (emotion_variance / self.current_emotions.len() as f32).sqrt();
    }
}