use crate::core::GfxModulator;
use infinitedsp_core::core::audio_param::AudioParam;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

/// Bridge between audio parameters and graphics modulators.
pub struct AudioBridge {
    pub param: AudioParam,
    pub shared_sample_rate: Arc<AtomicU32>,
    pub last_clock: u64,
}

impl GfxModulator for AudioBridge {
    fn tick(&mut self, time: f32) -> f32 {
        let sr = f32::from_bits(self.shared_sample_rate.load(Ordering::Relaxed));
        let sr = if sr < 1.0 { 44100.0 } else { sr };
        let current_clock = (time * sr) as u64;

        if current_clock > self.last_clock {
            let diff = (current_clock - self.last_clock) as usize;
            let mut temp_buf = [0.0; 64];
            let mut remaining = diff;
            let mut internal_clock = self.last_clock;

            while remaining > 0 {
                let chunk = remaining.min(64);
                self.param.process(&mut temp_buf[..chunk], internal_clock);
                remaining -= chunk;
                internal_clock += chunk as u64;
            }
            self.last_clock = current_clock;
        }

        self.param.get_value_at(current_clock)
    }
}
