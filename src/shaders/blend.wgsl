// Types
struct Globals {
    time: f32,
    kick: f32,
    sweep: f32,
    res_x: f32,
    res_y: f32,
    fade: f32,
    p2: f32,
    p3: f32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(1) var t_bg: texture_2d<f32>;
@group(1) @binding(2) var s_bg: sampler;
@group(1) @binding(3) var t_fg: texture_2d<f32>;
@group(1) @binding(4) var s_fg: sampler;

// Shaders
@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((i32(idx) << 1u) & 2);
    let y = f32(i32(idx) & 2);
    out.pos = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    out.uv = vec2<f32>(x, y); 
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let bg = textureSampleLevel(t_bg, s_bg, in.uv, 0.0).rgb;
    let fg = textureSampleLevel(t_fg, s_fg, in.uv, 0.0).rgb;
    
    return vec4<f32>(bg + fg, 1.0);
}
