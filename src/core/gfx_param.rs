use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

/// Interface for modulating graphics values over time.
pub trait GfxModulator: Send {
    /// Advances the modulator state and returns the new value.
    fn tick(&mut self, time: f32) -> f32;
}

/// Represents a graphics parameter that can be static, linked to an atomic variable, or dynamically modulated.
pub enum GfxParam {
    /// A fixed value that never changes.
    Static(f32),
    /// A value linked to an external atomic variable (e.g., from an audio engine).
    Linked(Arc<AtomicU32>),
    /// A value calculated dynamically by a modulator.
    Dynamic(Box<dyn GfxModulator>),
}

impl From<f32> for GfxParam {
    fn from(v: f32) -> Self {
        GfxParam::Static(v)
    }
}

impl From<Arc<AtomicU32>> for GfxParam {
    fn from(v: Arc<AtomicU32>) -> Self {
        GfxParam::Linked(v)
    }
}

impl<T: GfxModulator + 'static> From<T> for GfxParam {
    fn from(v: T) -> Self {
        GfxParam::Dynamic(Box::new(v))
    }
}

impl GfxParam {
    /// Returns the current value of the parameter at the given time.
    pub fn get_value(&mut self, time: f32) -> f32 {
        match self {
            GfxParam::Static(v) => *v,
            GfxParam::Linked(a) => f32::from_bits(a.load(Ordering::Relaxed)),
            GfxParam::Dynamic(m) => m.tick(time),
        }
    }
}
