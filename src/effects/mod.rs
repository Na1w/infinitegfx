pub mod node;
pub mod text;

pub use node::{ShaderInputs, ShaderNode};
pub use text::TextEffect;

use crate::core::GfxParam;

/// Creates a fade effect.
pub fn fade(start_time: f32, duration: f32) -> ShaderNode {
    ShaderNode::new(
        "FadeEffect",
        include_str!("../shaders/effects/fade.wgsl"),
        ShaderInputs::One,
    )
    .with_uniforms(move |t| [start_time, duration, t, 0.0])
}

/// Creates a glitch effect.
pub fn glitch(intensity: impl Into<GfxParam>, kick: impl Into<GfxParam>) -> ShaderNode {
    let mut intensity = intensity.into();
    let mut kick = kick.into();
    ShaderNode::new(
        "GlitchEffect",
        include_str!("../shaders/effects/glitch.wgsl"),
        ShaderInputs::One,
    )
    .with_uniforms(move |t| [intensity.get_value(t), kick.get_value(t), 0.0, 0.0])
}

/// Creates a flip effect (transition).
pub fn flip() -> ShaderNode {
    ShaderNode::new(
        "FlipEffect",
        include_str!("../shaders/effects/flip.wgsl"),
        ShaderInputs::Two,
    )
    .with_uniform_size(32)
}

/// Creates a warp effect (transition).
pub fn warp() -> ShaderNode {
    ShaderNode::new(
        "WarpEffect",
        include_str!("../shaders/effects/warp.wgsl"),
        ShaderInputs::Two,
    )
    .with_uniform_size(16)
}

/// Creates a crossfade effect (transition).
pub fn crossfade() -> ShaderNode {
    ShaderNode::new(
        "Crossfade",
        include_str!("../shaders/effects/crossfade.wgsl"),
        ShaderInputs::Two,
    )
    .with_uniform_size(16)
}

/// Creates a space-bend effect (transition).
pub fn space_bend() -> ShaderNode {
    ShaderNode::new(
        "SpaceBend",
        include_str!("../shaders/effects/space_bend.wgsl"),
        ShaderInputs::Two,
    )
    .with_uniform_size(16)
}

/// Renders a solid color.
pub fn solid_color(color: [f32; 4]) -> ShaderNode {
    ShaderNode::new(
        "SolidColor",
        include_str!("../shaders/effects/solid.wgsl"),
        ShaderInputs::None,
    )
    .with_uniforms(move |_| color)
}

/// Creates a laser curtain effect.
pub fn curtain() -> ShaderNode {
    ShaderNode::new(
        "CurtainEffect",
        include_str!("../shaders/effects/curtain.wgsl"),
        ShaderInputs::One,
    )
    .with_uniform_size(16) // Matches CurtainData
}

/// Creates a haze/displacement effect.
pub fn haze(intensity: impl Into<GfxParam>, sweep: impl Into<GfxParam>) -> ShaderNode {
    let mut intensity = intensity.into();
    let mut sweep = sweep.into();
    ShaderNode::new(
        "HazeEffect",
        include_str!("../shaders/effects/haze.wgsl"),
        ShaderInputs::One,
    )
    .with_uniforms(move |t| [intensity.get_value(t), sweep.get_value(t), 0.0, 0.0])
}

/// Creates a sparkle/bloom effect.
pub fn sparkle() -> ShaderNode {
    ShaderNode::new(
        "SparkleEffect",
        include_str!("../shaders/effects/sparkle.wgsl"),
        ShaderInputs::One,
    )
}

/// Creates a glass distortion effect.
pub fn glass(intensity: impl Into<GfxParam>) -> ShaderNode {
    let mut intensity = intensity.into();
    ShaderNode::new(
        "GlassEffect",
        include_str!("../shaders/effects/glass.wgsl"),
        ShaderInputs::One,
    )
    .with_uniforms(move |t| [intensity.get_value(t), 0.0, 0.0, 0.0])
}
