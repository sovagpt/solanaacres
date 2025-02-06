pub struct TimeSystem {
    current_time: f32,
    time_scale: f32,
    day_cycle: DayCycle,
}

pub struct DayCycle {
    hour: u8,
    minute: u8,
    day: u32,
}

impl TimeSystem {
    pub fn new() -> Self {
        Self {
            current_time: 0.0,
            time_scale: 1.0,
            day_cycle: DayCycle {
                hour: 6, // Start at 6 AM
                minute: 0,
                day: 1,
            },
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time * self.time_scale;
        self.update_day_cycle(delta_time);
    }
}