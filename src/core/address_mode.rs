/// Abstraction for texture sampling modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GfxAddressMode {
    /// Clamps the texture coordinates to the [0.0, 1.0] range.
    ClampToEdge,
    /// Repeats the texture when coordinates are outside the [0.0, 1.0] range.
    Repeat,
    /// Repeats the texture but mirrors it on every other repetition.
    MirrorRepeat,
}
