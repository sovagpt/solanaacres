#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwarenessSystem {
    is_aware: bool,
    awareness_level: f32,
    reality_perception: f32,
    doubt_level: f32,
}

impl AwarenessSystem {
    pub fn new() -> Self {
        Self {
            is_aware: false,
            awareness_level: 0.0,
            reality_perception: 1.0,
            doubt_level: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.is_aware {
            // Occasionally question reality
            if rand::random::<f32>() < 0.01 {
                self.doubt_level += rand::random::<f32>() * 0.1;
            }

            // Reality perception shifts based on doubt
            self.reality_perception = 1.0 - (self.doubt_level * 0.2);
        }
    }

    pub fn process_anomaly(&mut self, strength: f32) {
        if !self.is_aware {
            self.doubt_level += strength * 0.1;
            if self.doubt_level > 0.9 {
                self.trigger_awareness();
            }
        }
    }

    fn trigger_awareness(&mut self) {
        self.is_aware = true;
        self.awareness_level = 1.0;
        self.reality_perception = 0.5;
    }
}