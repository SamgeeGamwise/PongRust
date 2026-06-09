pub struct Timer {
    remaining: f32,
}

impl Timer {
    pub fn new(seconds: f32) -> Self {
        Self {
            remaining: seconds,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.remaining > 0.0 {
            self.remaining -= delta_time;
        }
    }

    pub fn finished(&self) -> bool {
        self.remaining <= 0.0
    }

    pub fn reset(&mut self, seconds: f32) {
        self.remaining = seconds;
    }
}