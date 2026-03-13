pub mod backend;
pub mod core;
pub mod effects;
pub mod font;
pub mod modulators;

pub use backend::GfxBackend;
pub use core::gfx_chain::GfxChain;
pub use core::{
    GfxAddressMode, GfxFrameProcessor, GfxHandle, GfxModulator, GfxParam, RenderContext,
};
pub use modulators::{AudioBridge, KickPumper, LinearSweep, RampingLfo};

/// Standard structure for global parameters passed to all shaders.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default, Debug)]
pub struct StandardGlobals {
    /// The current playback time in seconds.
    pub time: f32,
    /// Kick drum intensity (0.0 to 1.0).
    pub kick: f32,
    /// Filter sweep intensity (0.0 to 1.0).
    pub sweep: f32,
    /// Horizontal resolution of the render target.
    pub res_x: f32,
    /// Vertical resolution of the render target.
    pub res_y: f32,
    /// Global fade-in/out progress.
    pub fade: f32,
    /// Reserved parameter 2.
    pub p2: f32,
    /// Reserved parameter 3 (often used for reactive rotation).
    pub p3: f32,
}
