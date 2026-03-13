use super::context::RenderContext;
use super::gfx_frame_processor::GfxFrameProcessor;
use wgpu;

/// Manages a chain of graphics processors executed sequentially.
///
/// The `GfxChain` handles the orchestration of multiple `GfxFrameProcessor`s,
/// managing intermediate textures (ping-pong) and optional isolation/blending.
pub struct GfxChain {
    /// The sequence of processors to be executed.
    processors: Vec<Box<dyn GfxFrameProcessor>>,
    /// Internal texture used for ping-pong rendering.
    ping_texture: Option<wgpu::Texture>,
    /// Internal texture used for ping-pong rendering.
    pong_texture: Option<wgpu::Texture>,
    /// View for the ping texture.
    ping_view: Option<wgpu::TextureView>,
    /// View for the pong texture.
    pong_view: Option<wgpu::TextureView>,
    /// The texture format used by the chain.
    format: wgpu::TextureFormat,
    /// Current width of the internal textures.
    width: u32,
    /// Current height of the internal textures.
    height: u32,
    /// Whether this chain is isolated (renders to its own buffers then blends).
    is_isolated: bool,
    /// Pipeline for blending the isolated result back.
    pipeline: Option<wgpu::RenderPipeline>,
    /// Bind group layout for blending.
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    /// Sampler for blending.
    sampler: Option<wgpu::Sampler>,
    /// A 1x1 black texture used as a fallback input.
    dummy_texture: Option<wgpu::Texture>,
    /// View for the dummy texture.
    dummy_view: Option<wgpu::TextureView>,
}

impl GfxChain {
    /// Creates a new, empty graphics chain with the specified texture format.
    pub fn new(format: wgpu::TextureFormat) -> Self {
        Self {
            processors: Vec::new(),
            ping_texture: None,
            pong_texture: None,
            ping_view: None,
            pong_view: None,
            format,
            width: 0,
            height: 0,
            is_isolated: false,
            pipeline: None,
            bind_group_layout: None,
            sampler: None,
            dummy_texture: None,
            dummy_view: None,
        }
    }

    /// Marks the chain as isolated.
    ///
    /// Isolated chains render their processors to internal textures and then
    /// blend the result over the incoming `input_view` using an alpha blend.
    pub fn isolated(mut self) -> Self {
        self.is_isolated = true;
        self
    }

    /// Adds a processor to the end of the chain.
    pub fn and(mut self, processor: impl GfxFrameProcessor + 'static) -> Self {
        self.processors.push(Box::new(processor));
        self
    }

    /// Returns the texture view of the last rendering result in the chain.
    ///
    /// This is useful for transitions or effects that need to sample a completed scene.
    pub fn last_result_view(&self) -> Option<&wgpu::TextureView> {
        let num = self.processors.len();
        if num == 0 {
            return self.dummy_view.as_ref();
        }
        if num.is_multiple_of(2) {
            self.pong_view.as_ref()
        } else {
            self.ping_view.as_ref()
        }
    }

    /// Renders the chain to its own internal ping-pong textures.
    ///
    /// This forces the final result into the chain's internal buffers regardless
    /// of the `target_view` provided in the context.
    pub fn render_to_self<'a>(&'a mut self, ctx: RenderContext<'a>) {
        if self.processors.is_empty() {
            return;
        }
        self.ensure_textures(ctx.device, self.width.max(1), self.height.max(1));

        let num = self.processors.len();
        let is_even = num.is_multiple_of(2);

        self.render_internal(ctx, true, is_even);
    }

    /// Internal rendering logic handling both direct and isolated rendering.
    fn render_internal<'a>(&'a mut self, ctx: RenderContext<'a>, to_self: bool, is_even: bool) {
        if self.processors.is_empty() {
            return;
        }
        self.ensure_textures(ctx.device, self.width.max(1), self.height.max(1));

        if self.dummy_texture.is_none() {
            let dummy_desc = wgpu::TextureDescriptor {
                label: Some("Dummy"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            };
            let dt = ctx.device.create_texture(&dummy_desc);
            let dv = dt.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = ctx
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("ClearDummy"),
                });
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ClearDummy"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &dv,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            ctx.queue.submit(Some(encoder.finish()));
            self.dummy_view = Some(dv);
            self.dummy_texture = Some(dt);
        }

        {
            // Initial clear of the first buffer in the ping-pong sequence.
            let _ = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("InitialClear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: self.ping_view.as_ref().unwrap(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        let num_processors = self.processors.len();
        let mut is_ping = true;

        for (i, p) in self.processors.iter_mut().enumerate() {
            let out_view = if i == num_processors - 1 && !self.is_isolated && !to_self {
                ctx.target_view
            } else if i == num_processors - 1 && to_self {
                if is_even {
                    self.pong_view.as_ref().unwrap()
                } else {
                    self.ping_view.as_ref().unwrap()
                }
            } else if is_ping {
                self.ping_view.as_ref().unwrap()
            } else {
                self.pong_view.as_ref().unwrap()
            };

            let in_view = if i == 0 {
                if self.is_isolated {
                    self.dummy_view.as_ref()
                } else {
                    ctx.input_view.or(self.dummy_view.as_ref())
                }
            } else if is_ping {
                self.pong_view.as_ref()
            } else {
                self.ping_view.as_ref()
            };

            p.render(RenderContext {
                device: ctx.device,
                encoder: ctx.encoder,
                target_view: out_view,
                input_view: in_view,
                globals_bind_group: ctx.globals_bind_group,
                time: ctx.time,
                queue: ctx.queue,
                globals_buf: ctx.globals_buf,
            });
            is_ping = !is_ping;
        }

        if self.is_isolated {
            // Blend the isolated result back onto the input background.
            let group_result_view = if is_ping {
                self.pong_view.as_ref().unwrap()
            } else {
                self.ping_view.as_ref().unwrap()
            };
            let bg_view = ctx
                .input_view
                .or(self.dummy_view.as_ref())
                .expect("Background missing for blend");
            let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: self.bind_group_layout.as_ref().unwrap(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(bg_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(self.sampler.as_ref().unwrap()),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(group_result_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::Sampler(self.sampler.as_ref().unwrap()),
                    },
                ],
                label: None,
            });

            let final_target = if to_self {
                if is_even {
                    self.pong_view.as_ref().unwrap()
                } else {
                    self.ping_view.as_ref().unwrap()
                }
            } else {
                ctx.target_view
            };

            let mut rpass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ChainBlendPass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: final_target,
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

    /// Ensures that internal textures match the requested dimensions.
    fn ensure_textures(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if self.width == width && self.height == height && self.ping_texture.is_some() {
            return;
        }
        let size = wgpu::Extent3d {
            width: width.max(1),
            height: height.max(1),
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("ChainPingPong"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let t1 = device.create_texture(&desc);
        let t2 = device.create_texture(&desc);
        self.ping_view = Some(t1.create_view(&wgpu::TextureViewDescriptor::default()));
        self.pong_view = Some(t2.create_view(&wgpu::TextureViewDescriptor::default()));
        self.ping_texture = Some(t1);
        self.pong_texture = Some(t2);
        self.width = width;
        self.height = height;
        for p in &mut self.processors {
            p.resize(device, width, height);
        }
    }
}

impl GfxFrameProcessor for GfxChain {
    fn init(&mut self, handle: &crate::core::GfxHandle, globals_layout: &wgpu::BindGroupLayout) {
        let device = handle.device;
        let format = handle.format;
        self.format = format;
        for p in &mut self.processors {
            p.init(handle, globals_layout);
        }
        if self.is_isolated {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("ChainBlendShader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/blend.wgsl").into()),
            });
            let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ChainBlendBGL"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("ChainBlendLayout"),
                bind_group_layouts: &[globals_layout, &bgl],
                immediate_size: 0,
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ChainBlendPipeline"),
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
            self.sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor::default()));
            self.bind_group_layout = Some(bgl);
            self.pipeline = Some(pipeline);
        }
    }
    fn update(&mut self, time: f32, queue: &wgpu::Queue, globals_buf: &wgpu::Buffer) {
        for p in &mut self.processors {
            p.update(time, queue, globals_buf);
        }
    }
    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.ensure_textures(device, width, height);
    }
    fn render<'a>(&'a mut self, ctx: RenderContext<'a>) {
        self.render_internal(ctx, false, false);
    }
}
