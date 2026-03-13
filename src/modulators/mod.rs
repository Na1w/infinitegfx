pub mod audio;
pub mod envelope;
pub mod lfo;

pub use audio::AudioBridge;
pub use envelope::KickPumper;
pub use lfo::{LinearSweep, RampingLfo};
