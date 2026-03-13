/// Wrapper for graphics backend resources to hide implementation details (e.g., wgpu).
pub struct GfxHandle<'a> {
    /// The GPU device used for resource creation.
    pub device: &'a wgpu::Device,
    /// The GPU command queue.
    pub queue: &'a wgpu::Queue,
    /// The preferred texture format for rendering.
    pub format: wgpu::TextureFormat,
}

impl<'a> GfxHandle<'a> {
    /// Creates a uniform buffer for global parameters.
    ///
    /// Returns the buffer, its layout, and a pre-configured bind group.
    pub fn create_globals_buffer(
        &self,
        size: u64,
    ) -> (wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("GlobalsBuf"),
            size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let layout = self
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("GlobalsLayout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            label: Some("GlobalsBindGroup"),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        (buffer, layout, bind_group)
    }
}
