use serde::{Serialize, Deserialize};
use super::traits::PersonalityTraits;
use super::emotions::{EmotionalState, Emotion};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorProfile {
    social_preference: f32,    // Preference for social interaction
    risk_tolerance: f32,       // Willingness to take risks
    decision_speed: f32,       // How quickly decisions are made
    adaptability: f32,         // Ability to adapt to changes
    assertiveness: f32,        // Tendency to assert oneself
    cooperation: f32,          // Tendency to cooperate with others
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorTendency {
    Social,
    Cautious,
    Decisive,
    Adaptive,
    Assertive,
    Cooperative,
}

impl Default for BehaviorProfile {
    fn default() -> Self {
        Self {
            social_preference: 0.5,
            risk_tolerance: 0.5,
            decision_speed: 0.5,
            adaptability: 0.5,
            assertiveness: 0.5,
            cooperation: 0.5,
        }
    }
}

impl BehaviorProfile {
    pub fn new(traits: &PersonalityTraits) -> Self {
        Self {
            social_preference: calculate_social_preference(traits),
            risk_tolerance: calculate_risk_tolerance(traits),
            decision_speed: calculate_decision_speed(traits),
            adaptability: calculate_adaptability(traits),
            assertiveness: calculate_assertiveness(traits),
            cooperation: calculate_cooperation(traits),
        }
    }

    pub fn update(&mut self, traits: &PersonalityTraits, emotional_state: &EmotionalState) {
        // Update behavior based on current emotional state and personality traits
        self.adapt_to_emotional_state(emotional_state);
        self.align_with_traits(traits);
    }

    pub fn adapt_to_emotion(&mut self, emotion: &Emotion) {
        match emotion {
            Emotion::Joy => {
                self.social_preference += 0.1;
                self.risk_tolerance += 0.05;
            },
            Emotion::Fear => {
                self.risk_tolerance -= 0.2;
                self.decision_speed -= 0.1;
            },
            Emotion::Anger => {
                self.cooperation -= 0.2;
                self.assertiveness += 0.2;
            },
            Emotion::Trust => {
                self.cooperation += 0.1;
                self.social_preference += 0.1;
            },
            _ => {}
        }

        self.clamp_values();
    }

    pub fn get_dominant_tendency(&self) -> BehaviorTendency {
        let tendencies = [
            (self.social_preference, BehaviorTendency::Social),
            (1.0 - self.risk_tolerance, BehaviorTendency::Cautious),
            (self.decision_speed, BehaviorTendency::Decisive),
            (self.adaptability, BehaviorTendency::Adaptive),
            (self.assertiveness, BehaviorTendency::Assertive),
            (self.cooperation, BehaviorTendency::Cooperative),
        ];

        tendencies.iter()
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .map(|&(_, ref tendency)| tendency.clone())
            .unwrap_or(BehaviorTendency::Adaptive)
    }

    fn adapt_to_emotional_state(&mut self, emotional_state: &EmotionalState) {
        let valence = emotional_state.get_emotional_valence();
        
        // Positive emotions increase social and cooperative tendencies
        if valence > 0.0 {
            self.social_preference += 0.05 * valence;
            self.cooperation += 0.05 * valence;
        } else {
            // Negative emotions decrease social tendencies but might increase assertiveness
            self.social_preference += 0.05 * valence;
            self.assertiveness -= 0.05 * valence;
        }

        self.clamp_values();
    }

    fn align_with_traits(&mut self, traits: &PersonalityTraits) {
        let target_social = calculate_social_preference(traits);
        let target_risk = calculate_risk_tolerance(traits);
        let target_speed = calculate_decision_speed(traits);
        let target_adapt = calculate_adaptability(traits);
        let target_assert = calculate_assertiveness(traits);
        let target_coop = calculate_cooperation(traits);

        // Gradually align with personality traits
        self.social_preference += (target_social - self.social_preference) * 0.1;
        self.risk_tolerance += (target_risk - self.risk_tolerance) * 0.1;
        self.decision_speed += (target_speed - self.decision_speed) * 0.1;
        self.adaptability += (target_adapt - self.adaptability) * 0.1;
        self.assertiveness += (target_assert - self.assertiveness) * 0.1;
        self.cooperation += (target_coop - self.cooperation) * 0.1;

        self.clamp_values();
    }

    fn clamp_values(&mut self) {
        self.social_preference = self.social_preference.clamp(0.0, 1.0);
        self.risk_tolerance = self.risk_tolerance.clamp(0.0, 1.0);
        self.decision_speed = self.decision_speed.clamp(0.0, 1.0);
        self.adaptability = self.adaptability.clamp(0.0, 1.0);
        self.assertiveness = self.assertiveness.clamp(0.0, 1.0);
        self.cooperation = self.cooperation.clamp(0.0, 1.0);
    }
}

fn calculate_social_preference(traits: &PersonalityTraits) -> f32 {
    (traits.extraversion * 0.6 + traits.agreeableness * 0.4).clamp(0.0, 1.0)
}

fn calculate_risk_tolerance(traits: &PersonalityTraits) -> f32 {
    (traits.openness * 0.4 + (1.0 - traits.neuroticism) * 0.6).clamp(0.0, 1.0)
}

fn calculate_decision_speed(traits: &PersonalityTraits) -> f32 {
    (traits.conscientiousness * 0.5 + (1.0 - traits.neuroticism) * 0.5).clamp(0.0, 1.0)
}

fn calculate_adaptability(traits: &PersonalityTraits) -> f32 {
    (traits.openness * 0.5 + traits.conscientiousness * 0.5).clamp(0.0, 1.0)
}

fn calculate_assertiveness(traits: &PersonalityTraits) -> f32 {
    (traits.extraversion * 0.4 + (1.0 - traits.agreeableness) * 0.6).clamp(0.0, 1.0)
}

fn calculate_cooperation(traits: &PersonalityTraits) -> f32 {
    (traits.agreeableness * 0.7 + traits.conscientiousness * 0.3).clamp(0.0, 1.0)
}