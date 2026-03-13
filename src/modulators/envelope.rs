use crate::core::GfxModulator;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

/// A modulator that reacts to kick drums via a shared atomic value.
pub struct KickPumper {
    pub shared_value: Arc<AtomicU32>,
    pub base: f32,
    pub amount: f32,
}

impl GfxModulator for KickPumper {
    fn tick(&mut self, _time: f32) -> f32 {
        let val = f32::from_bits(self.shared_value.load(Ordering::Relaxed));
        self.base + val * self.amount
    }
}
