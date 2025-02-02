use rand::prelude::*;
use super::{Personality, PersonalityTraits, EmotionalState, BehaviorProfile};

pub fn generate_personality() -> Personality {
    let traits = generate_traits();
    let emotional_state = EmotionalState::default();
    let behavior_profile = BehaviorProfile::new(&traits);
    
    Personality {
        traits,
        emotional_state,
        behavior_profile,
        stability: calculate_initial_stability(&traits),
    }
}

fn generate_traits() -> PersonalityTraits {
    let mut rng = thread_rng();
    
    PersonalityTraits::new(
        generate_trait_value(&mut rng),  // openness
        generate_trait_value(&mut rng),  // conscientiousness
        generate_trait_value(&mut rng),  // extraversion
        generate_trait_value(&mut rng),  // agreeableness
        generate_trait_value(&mut rng),  // neuroticism
    )
}

fn generate_trait_value(rng: &mut ThreadRng) -> f32 {
    // Use normal distribution centered at 0.5 with standard deviation of 0.15
    let normal = rand_distr::Normal::new(0.5, 0.15).unwrap();
    normal.sample(rng).clamp(0.0, 1.0)
}

fn calculate_initial_stability(traits: &PersonalityTraits) -> f32 {
    // Higher conscientiousness and lower neuroticism contribute to stability
    let base_stability = (traits.conscientiousness * 0.4 + (1.0 - traits.neuroticism) * 0.6);
    
    // Add some random variation
    let mut rng = thread_rng();
    let variation = rng.gen_range(-0.1..0.1);
    
    (base_stability + variation).clamp(0.0, 1.0)
}

pub fn generate_archetype(archetype: PersonalityArchetype) -> PersonalityTraits {
    let mut traits = match archetype {
        PersonalityArchetype::Leader => PersonalityTraits::new(
            0.7,  // High openness
            0.8,  // High conscientiousness
            0.8,  // High extraversion
            0.6,  // Moderate agreeableness
            0.3,  // Low neuroticism
        ),
        PersonalityArchetype::Caregiver => PersonalityTraits::new(
            0.6,  // Moderate openness
            0.7,  // High conscientiousness
            0.6,  // Moderate extraversion
            0.9,  // Very high agreeableness
            0.4,  // Low-moderate neuroticism
        ),
        PersonalityArchetype::Rebel => PersonalityTraits::new(
            0.8,  // High openness
            0.3,  // Low conscientiousness
            0.7,  // High extraversion
            0.3,  // Low agreeableness
            0.6,  // Moderate-high neuroticism
        ),
        PersonalityArchetype::Scholar => PersonalityTraits::new(
            0.9,  // Very high openness
            0.8,  // High conscientiousness
            0.4,  // Low-moderate extraversion
            0.6,  // Moderate agreeableness
            0.5,  // Moderate neuroticism
        ),
    };

    // Add some random variation to make each instance unique
    add_random_variation(&mut traits);
    traits
}

pub enum PersonalityArchetype {
    Leader,
    Caregiver,
    Rebel,
    Scholar,
}

fn add_random_variation(traits: &mut PersonalityTraits) {
    let mut rng = thread_rng();
    let variation_range = 0.1;

    traits.openness += rng.gen_range(-variation_range..variation_range);
    traits.conscientiousness += rng.gen_range(-variation_range..variation_range);
    traits.extraversion += rng.gen_range(-variation_range..variation_range);
    traits.agreeableness += rng.gen_range(-variation_range..variation_range);
    traits.neuroticism += rng.gen_range(-variation_range..variation_range);

    // Ensure all values remain in valid range
    traits.openness = traits.openness.clamp(0.0, 1.0);
    traits.conscientiousness = traits.conscientiousness.clamp(0.0, 1.0);
    traits.extraversion = traits.extraversion.clamp(0.0, 1.0);
    traits.agreeableness = traits.agreeableness.clamp(0.0, 1.0);
    traits.neuroticism = traits.neuroticism.clamp(0.0, 1.0);
}

// Additional utility functions for generating specific types of personalities

pub fn generate_unaware_personality() -> Personality {
    let mut personality = generate_personality();
    
    // Modify traits to be more susceptible to manipulation
    personality.traits.openness *= 0.7;      // Less open to questioning reality
    personality.traits.neuroticism *= 1.2;   // More prone to anxiety/stress
    personality.stability *= 0.8;            // Less stable overall
    
    personality
}

pub fn generate_aware_personality() -> Personality {
    let mut personality = generate_personality();
    
    // Modify traits to be more questioning and stable
    personality.traits.openness *= 1.3;      // More open to questioning reality
    personality.traits.conscientiousness *= 1.2; // More methodical
    personality.stability *= 1.2;            // More stable overall
    
    personality
}