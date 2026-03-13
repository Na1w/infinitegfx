/// Contains basic resources for a single frame.
pub struct GfxFrame<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub target_view: &'a wgpu::TextureView,
    pub width: u32,
    pub height: u32,
    pub time: f32,
}

/// Contains resources and state required for rendering by a processor.
pub struct RenderContext<'a> {
    /// The GPU device.
    pub device: &'a wgpu::Device,
    /// The command encoder for recording GPU operations.
    pub encoder: &'a mut wgpu::CommandEncoder,
    /// The texture view to render into.
    pub target_view: &'a wgpu::TextureView,
    /// Optional input texture view for effects (e.g., post-processing).
    pub input_view: Option<&'a wgpu::TextureView>,
    /// Bind group containing global shader parameters.
    pub globals_bind_group: &'a wgpu::BindGroup,
    /// The current playback time in seconds.
    pub time: f32,
    /// The GPU command queue.
    pub queue: &'a wgpu::Queue,
    /// The buffer containing global shader parameters.
    pub globals_buf: &'a wgpu::Buffer,
}
