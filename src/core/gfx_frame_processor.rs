use super::{GfxHandle, RenderContext};

/// Interface for objects that can perform graphics operations and rendering per frame.
#[cfg(not(target_arch = "wasm32"))]
pub trait GfxFrameProcessor: Send {
    /// Initializes GPU resources (pipelines, bind groups, etc.).
    fn init(&mut self, handle: &GfxHandle, globals_layout: &wgpu::BindGroupLayout);

    /// Updates internal state before rendering (e.g., writing uniform buffers).
    fn update(&mut self, time: f32, queue: &wgpu::Queue, globals_buf: &wgpu::Buffer);

    /// Resizes internal textures to match the new dimensions.
    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32);

    /// Records GPU commands for rendering.
    fn render<'a>(&'a mut self, ctx: RenderContext<'a>);
}

/// Interface for objects that can perform graphics operations and rendering per frame.
#[cfg(target_arch = "wasm32")]
pub trait GfxFrameProcessor {
    /// Initializes GPU resources (pipelines, bind groups, etc.).
    fn init(&mut self, handle: &GfxHandle, globals_layout: &wgpu::BindGroupLayout);

    /// Updates internal state before rendering (e.g., writing uniform buffers).
    fn update(&mut self, time: f32, queue: &wgpu::Queue, globals_buf: &wgpu::Buffer);

    /// Resizes internal textures to match the new dimensions.
    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32);

    /// Records GPU commands for rendering.
    fn render<'a>(&'a mut self, ctx: RenderContext<'a>);
}
