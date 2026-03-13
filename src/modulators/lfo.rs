use crate::core::GfxModulator;

/// A modulator that performs a linear sweep between two values.
pub struct LinearSweep {
    pub start: f32,
    pub end: f32,
    pub start_time: f32,
    pub duration: f32,
}

impl GfxModulator for LinearSweep {
    fn tick(&mut self, time: f32) -> f32 {
        let t = (time - self.start_time) / self.duration;
        let t = t.clamp(0.0, 1.0);
        self.start + (self.end - self.start) * t
    }
}

/// An LFO modulator whose range can change linearly over time.
pub struct RampingLfo {
    pub frequency: f32,
    pub min_start: f32,
    pub max_start: f32,
    pub min_end: f32,
    pub max_end: f32,
    pub start_time: f32,
    pub duration: f32,
}

impl GfxModulator for RampingLfo {
    fn tick(&mut self, time: f32) -> f32 {
        let t = ((time - self.start_time) / self.duration).clamp(0.0, 1.0);
        let current_min = self.min_start + (self.min_end - self.min_start) * t;
        let current_max = self.max_start + (self.max_end - self.max_start) * t;

        let sine = (time * self.frequency * std::f32::consts::TAU).sin() * 0.5 + 0.5;
        current_min + (current_max - current_min) * sine
    }
}
