use crate::core::{GfxFrameProcessor, GfxHandle, GfxParam, RenderContext};
use crate::font::DEFAULT_FONT;
use bytemuck;
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct TextUniforms {
    char_ids: [[i32; 4]; 8],
    num_chars: i32,
    offset_x: f32,
    offset_y: f32,
    scale: f32,
    start_time: f32,
    end_time: f32,
    fade_in: f32,
    fade_out: f32,
    color: [f32; 3],
    explosion: f32,
    mod_x_amp: f32,
    mod_x_wavelength: f32,
    mod_y_amp: f32,
    mod_y_wavelength: f32,
    gravity_x: f32,
    gravity_y: f32,
    wipe_angle: f32,
    wipe_width: f32,
}

/// A graphics processor for rendering text with various effects.
pub struct TextEffect {
    pipeline: Option<wgpu::RenderPipeline>,
    uniform_buf: Option<wgpu::Buffer>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    sampler: Option<wgpu::Sampler>,
    font_buf: Option<wgpu::Buffer>,
    font_bind_group: Option<wgpu::BindGroup>,
    font_loaded: bool,
    char_ids: [[i32; 4]; 8],
    num_chars: i32,
    pos_x: GfxParam,
    pos_y: GfxParam,
    scale: GfxParam,
    explosion: GfxParam,
    gravity_x: GfxParam,
    gravity_y: GfxParam,
    wipe_angle: GfxParam,
    wipe_width: GfxParam,
    start_time: f32,
    end_time: f32,
    fade_in: f32,
    fade_out: f32,
    color: [f32; 3],
    current_time: f32,
    mod_x_amp: GfxParam,
    mod_x_wavelength: GfxParam,
    mod_y_amp: GfxParam,
    mod_y_wavelength: GfxParam,
}

impl TextEffect {
    /// Creates a new text effect with the given text and timing.
    pub fn new(text: &str, start_time: f32, end_time: f32) -> Self {
        let mut ids = [[-1; 4]; 8];
        let mut count = 0;
        for (i, c) in text.chars().enumerate().take(32) {
            let v_idx = i / 4;
            let c_idx = i % 4;
            let char_id = match c.to_ascii_uppercase() {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                'E' => 4,
                'F' => 5,
                'G' => 6,
                'H' => 7,
                'I' => 8,
                'J' => 9,
                'K' => 10,
                'L' => 11,
                'M' => 12,
                'N' => 13,
                'O' => 14,
                'P' => 15,
                'Q' => 16,
                'R' => 17,
                'S' => 18,
                'T' => 19,
                'U' => 20,
                'V' => 21,
                'W' => 22,
                'X' => 23,
                'Y' => 24,
                'Z' => 25,
                ' ' => 26,
                _ => -1,
            };
            ids[v_idx][c_idx] = char_id;
            if char_id != -1 {
                count = i + 1;
            }
        }
        Self {
            pipeline: None,
            uniform_buf: None,
            bind_group_layout: None,
            sampler: None,
            font_buf: None,
            font_bind_group: None,
            font_loaded: false,
            char_ids: ids,
            num_chars: count as i32,
            pos_x: GfxParam::Static(0.0),
            pos_y: GfxParam::Static(0.0),
            scale: GfxParam::Static(1.0),
            explosion: GfxParam::Static(0.0),
            gravity_x: GfxParam::Static(0.0),
            gravity_y: GfxParam::Static(0.0),
            wipe_angle: GfxParam::Static(0.0),
            wipe_width: GfxParam::Static(50.0),
            start_time,
            end_time,
            fade_in: 0.5,
            fade_out: 0.5,
            color: [1.0, 0.6, 0.0],
            current_time: 0.0,
            mod_x_amp: GfxParam::Static(0.0),
            mod_x_wavelength: GfxParam::Static(1.0),
            mod_y_amp: GfxParam::Static(0.0),
            mod_y_wavelength: GfxParam::Static(1.0),
        }
    }

    /// Sets the position of the text.
    pub fn with_pos(mut self, x: impl Into<GfxParam>, y: impl Into<GfxParam>) -> Self {
        self.pos_x = x.into();
        self.pos_y = y.into();
        self
    }

    /// Sets the scale of the text.
    pub fn with_scale(mut self, scale: impl Into<GfxParam>) -> Self {
        self.scale = scale.into();
        self
    }

    /// Sets the explosion (fragmentation) amount.
    pub fn with_explosion(mut self, explosion: impl Into<GfxParam>) -> Self {
        self.explosion = explosion.into();
        self
    }

    /// Sets the gravity applied to text fragments.
    pub fn with_gravity(mut self, x: impl Into<GfxParam>, y: impl Into<GfxParam>) -> Self {
        self.gravity_x = x.into();
        self.gravity_y = y.into();
        self
    }

    /// Sets the wipe effect angle and width.
    pub fn with_wipe(mut self, angle: impl Into<GfxParam>, width: impl Into<GfxParam>) -> Self {
        self.wipe_angle = angle.into();
        self.wipe_width = width.into();
        self
    }

    /// Sets the fade-in and fade-out durations.
    pub fn with_fade(mut self, in_t: f32, out_t: f32) -> Self {
        self.fade_in = in_t;
        self.fade_out = out_t;
        self
    }

    /// Sets the color of the text.
    pub fn with_color(mut self, color: [f32; 3]) -> Self {
        self.color = color;
        self
    }

    /// Sets the modulation parameters for text displacement.
    pub fn with_modulation(
        mut self,
        x_amp: impl Into<GfxParam>,
        x_wave: impl Into<GfxParam>,
        y_amp: impl Into<GfxParam>,
        y_wave: impl Into<GfxParam>,
    ) -> Self {
        self.mod_x_amp = x_amp.into();
        self.mod_x_wavelength = x_wave.into();
        self.mod_y_amp = y_amp.into();
        self.mod_y_wavelength = y_wave.into();
        self
    }
}

impl GfxFrameProcessor for TextEffect {
    fn init(&mut self, handle: &GfxHandle, globals_layout: &wgpu::BindGroupLayout) {
        let device = handle.device;
        let format = handle.format;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("TextEffectShader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/effects/text.wgsl").into()),
        });

        let text_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("TextEffectLayout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
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
            ],
        });

        let font_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("FontLayout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let font_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("FontBuf"),
            size: 224,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let font_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &font_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: font_buf.as_entire_binding(),
            }],
            label: None,
        });

        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("TextUniforms"),
            size: 208,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("TextEffectPipelineLayout"),
            bind_group_layouts: &[globals_layout, &text_layout, &font_layout],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("TextEffectPipeline"),
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
        self.pipeline = Some(pipeline);
        self.bind_group_layout = Some(text_layout);
        self.uniform_buf = Some(uniform_buf);
        self.font_buf = Some(font_buf);
        self.font_bind_group = Some(font_bind_group);
    }

    fn update(&mut self, time: f32, queue: &wgpu::Queue, _globals_buf: &wgpu::Buffer) {
        self.current_time = time;

        if !self.font_loaded
            && let Some(buf) = &self.font_buf
        {
            let mut packed_font = [0u32; 56];
            for i in 0..27 {
                packed_font[i * 2] = DEFAULT_FONT[i][0];
                packed_font[i * 2 + 1] = DEFAULT_FONT[i][1];
            }
            queue.write_buffer(buf, 0, bytemuck::cast_slice(&packed_font));
            self.font_loaded = true;
        }

        let raw_scale = self.scale.get_value(time);
        let shader_scale = 120.0 / raw_scale.max(0.01);

        let uniforms = TextUniforms {
            char_ids: self.char_ids,
            num_chars: self.num_chars,
            offset_x: self.pos_x.get_value(time),
            offset_y: self.pos_y.get_value(time),
            scale: shader_scale,
            start_time: self.start_time,
            end_time: self.end_time,
            fade_in: self.fade_in,
            fade_out: self.fade_out,
            color: self.color,
            explosion: self.explosion.get_value(time),
            mod_x_amp: self.mod_x_amp.get_value(time),
            mod_x_wavelength: self.mod_x_wavelength.get_value(time),
            mod_y_amp: self.mod_y_amp.get_value(time),
            mod_y_wavelength: self.mod_y_wavelength.get_value(time),
            gravity_x: self.gravity_x.get_value(time),
            gravity_y: self.gravity_y.get_value(time),
            wipe_angle: self.wipe_angle.get_value(time),
            wipe_width: self.wipe_width.get_value(time),
        };
        if let Some(buf) = &self.uniform_buf {
            queue.write_buffer(buf, 0, bytemuck::bytes_of(&uniforms));
        }
    }

    fn resize(&mut self, _device: &wgpu::Device, _width: u32, _height: u32) {}

    fn render<'a>(&'a mut self, ctx: RenderContext<'a>) {
        let in_view = ctx.input_view.expect("TextEffect requires input texture");
        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: self.bind_group_layout.as_ref().unwrap(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buf.as_ref().unwrap().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(in_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(self.sampler.as_ref().unwrap()),
                },
            ],
            label: None,
        });
        let mut rpass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("TextEffectPass"),
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
        rpass.set_bind_group(2, self.font_bind_group.as_ref().unwrap(), &[]);
        rpass.draw(0..3, 0..1);
    }
}
