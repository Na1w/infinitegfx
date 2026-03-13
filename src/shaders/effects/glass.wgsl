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

struct GlassData {
    intensity: f32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> glass_data: GlassData;
@group(1) @binding(1) var t_diffuse: texture_2d<f32>;
@group(1) @binding(2) var s_diffuse: sampler;

// Helpers
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

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
    let t = globals.time;
    let intensity = glass_data.intensity;
    
    if (intensity <= 0.0) {
        return textureSampleLevel(t_diffuse, s_diffuse, in.uv, 0.0);
    }

    var uv = in.uv;
    
    let d1 = sin(uv.x * 6.0 + t * 1.2) * cos(uv.y * 5.0 + t * 0.8);
    let d2 = sin(uv.y * 8.0 - t * 1.5) * cos(uv.x * 7.0 - t * 1.1);
    
    let distortion = vec2<f32>(d1, d2) * intensity * 0.04;
    let final_uv = uv + distortion;
    
    let ab_strength = intensity * 0.008;
    let col_r = textureSampleLevel(t_diffuse, s_diffuse, final_uv + vec2<f32>(ab_strength, 0.0), 0.0).r;
    let col_g = textureSampleLevel(t_diffuse, s_diffuse, final_uv, 0.0).g;
    let col_b = textureSampleLevel(t_diffuse, s_diffuse, final_uv - vec2<f32>(ab_strength, 0.0), 0.0).b;
    
    return vec4<f32>(col_r, col_g, col_b, 1.0);
}
