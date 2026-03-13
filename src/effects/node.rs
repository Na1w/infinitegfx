use crate::core::{GfxAddressMode, GfxFrameProcessor, GfxHandle, RenderContext};
use wgpu;

/// Describes the number of inputs for a ShaderNode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderInputs {
    None,
    One,
    Two,
}

/// A universal node that can act as a source, filter, or transition.
/// It automatically handles WGPU boilerplate.
pub struct ShaderNode {
    label: String,
    shader_source: String,
    inputs: ShaderInputs,
    uniform_size: u64,
    uniform_updater: Option<Box<dyn FnMut(f32) -> Vec<u8> + Send>>,
    static_data: Option<Vec<u8>>,
    address_mode: GfxAddressMode,

    pub pipeline: Option<wgpu::RenderPipeline>,
    pub bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub sampler: Option<wgpu::Sampler>,
    pub uniform_buf: Option<wgpu::Buffer>,
    pub static_buf: Option<wgpu::Buffer>,
}

impl ShaderNode {
    /// Creates a new shader node.
    pub fn new(label: &str, shader_source: &str, inputs: ShaderInputs) -> Self {
        Self {
            label: label.to_string(),
            shader_source: shader_source.to_string(),
            inputs,
            uniform_size: 0,
            uniform_updater: None,
            static_data: None,
            address_mode: GfxAddressMode::ClampToEdge,
            pipeline: None,
            bind_group_layout: None,
            sampler: None,
            uniform_buf: None,
            static_buf: None,
        }
    }

    /// Sets static uniform data for the shader.
    pub fn with_static_data<T: bytemuck::Pod>(mut self, data: &T) -> Self {
        self.static_data = Some(bytemuck::bytes_of(data).to_vec());
        self
    }

    /// Sets a dynamic uniform updater for the shader.
    pub fn with_uniforms<T: bytemuck::Pod + Send + 'static>(
        mut self,
        mut updater: impl FnMut(f32) -> T + Send + 'static,
    ) -> Self {
        self.uniform_size = std::mem::size_of::<T>() as u64;
        self.uniform_updater = Some(Box::new(move |time| {
            bytemuck::bytes_of(&updater(time)).to_vec()
        }));
        self
    }

    /// Sets the size of the uniform buffer.
    pub fn with_uniform_size(mut self, size: u64) -> Self {
        self.uniform_size = size;
        self
    }

    /// Sets the texture address mode for the sampler.
    pub fn with_address_mode(mut self, mode: GfxAddressMode) -> Self {
        self.address_mode = mode;
        self
    }
}

fn map_address_mode(mode: GfxAddressMode) -> wgpu::AddressMode {
    match mode {
        GfxAddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
        GfxAddressMode::Repeat => wgpu::AddressMode::Repeat,
        GfxAddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
    }
}

impl GfxFrameProcessor for ShaderNode {
    fn init(&mut self, handle: &GfxHandle, globals_layout: &wgpu::BindGroupLayout) {
        let device = handle.device;
        let format = handle.format;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{}Shader", self.label)),
            source: wgpu::ShaderSource::Wgsl(self.shader_source.clone().into()),
        });

        let mut entries = Vec::new();

        if self.uniform_size > 0 {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });

            let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{}Uniforms", self.label)),
                size: self.uniform_size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.uniform_buf = Some(uniform_buf);
        }

        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        });

        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });

        if self.inputs == ShaderInputs::Two {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });
        }

        if let Some(data) = &self.static_data {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });

            use wgpu::util::DeviceExt;
            let static_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{}StaticBuf", self.label)),
                contents: data,
                usage: wgpu::BufferUsages::UNIFORM,
            });
            self.static_buf = Some(static_buf);
        }

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&format!("{}BGL", self.label)),
            entries: &entries,
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{}Layout", self.label)),
            bind_group_layouts: &[globals_layout, &bgl],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{}Pipeline", self.label)),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        self.sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: map_address_mode(self.address_mode),
            address_mode_v: map_address_mode(self.address_mode),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        }));
        self.pipeline = Some(pipeline);
        self.bind_group_layout = Some(bgl);
    }

    fn update(&mut self, time: f32, queue: &wgpu::Queue, _globals_buf: &wgpu::Buffer) {
        if let (Some(updater), Some(buf)) = (&mut self.uniform_updater, &self.uniform_buf) {
            let data = updater(time);
            queue.write_buffer(buf, 0, &data);
        }
    }

    fn resize(&mut self, _device: &wgpu::Device, _width: u32, _height: u32) {}

    fn render<'a>(&'a mut self, ctx: RenderContext<'a>) {
        if self.inputs == ShaderInputs::Two {
            return;
        }

        let mut entries = Vec::new();
        if let Some(buf) = &self.uniform_buf {
            entries.push(wgpu::BindGroupEntry {
                binding: 0,
                resource: buf.as_entire_binding(),
            });
        }

        let view = ctx.input_view.expect("ShaderNode requires input texture");
        entries.push(wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::TextureView(view),
        });
        entries.push(wgpu::BindGroupEntry {
            binding: 2,
            resource: wgpu::BindingResource::Sampler(self.sampler.as_ref().unwrap()),
        });

        if let Some(buf) = &self.static_buf {
            entries.push(wgpu::BindGroupEntry {
                binding: 4,
                resource: buf.as_entire_binding(),
            });
        }

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: self.bind_group_layout.as_ref().unwrap(),
            entries: &entries,
            label: None,
        });

        let mut rpass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(&format!("{}Pass", self.label)),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: ctx.target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        rpass.set_pipeline(self.pipeline.as_ref().unwrap());
        rpass.set_bind_group(0, ctx.globals_bind_group, &[]);
        rpass.set_bind_group(1, &bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}

impl ShaderNode {
    /// Renders the node using two input textures (e.g., for transitions).
    pub fn render_two<'a>(
        &'a self,
        ctx: RenderContext<'a>,
        view1: &wgpu::TextureView,
        view2: &wgpu::TextureView,
    ) {
        let mut entries = Vec::new();
        if let Some(buf) = &self.uniform_buf {
            entries.push(wgpu::BindGroupEntry {
                binding: 0,
                resource: buf.as_entire_binding(),
            });
        }
        entries.push(wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::TextureView(view1),
        });
        entries.push(wgpu::BindGroupEntry {
            binding: 2,
            resource: wgpu::BindingResource::Sampler(self.sampler.as_ref().unwrap()),
        });
        entries.push(wgpu::BindGroupEntry {
            binding: 3,
            resource: wgpu::BindingResource::TextureView(view2),
        });

        if let Some(buf) = &self.static_buf {
            entries.push(wgpu::BindGroupEntry {
                binding: 4,
                resource: buf.as_entire_binding(),
            });
        }

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: self.bind_group_layout.as_ref().unwrap(),
            entries: &entries,
            label: None,
        });

        let mut rpass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(&format!("{}Pass", self.label)),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: ctx.target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        rpass.set_pipeline(self.pipeline.as_ref().unwrap());
        rpass.set_bind_group(0, ctx.globals_bind_group, &[]);
        rpass.set_bind_group(1, &bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}
