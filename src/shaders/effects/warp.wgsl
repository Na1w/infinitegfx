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

struct WarpData {
    mix_factor: f32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> data: WarpData;
@group(1) @binding(1) var t_from: texture_2d<f32>;
@group(1) @binding(2) var s_diffuse: sampler;
@group(1) @binding(3) var t_to: texture_2d<f32>;

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
    let t = data.mix_factor;
    let uv = in.uv;
    
    let p = uv - 0.5;
    
    let dist = length(p);
    let angle = atan2(p.y, p.x);
    
    let strength = sin(t * 3.14159) * 2.0;
    let new_angle = angle + strength * exp(-dist * 3.0);
    
    let offset = vec2<f32>(cos(new_angle), sin(new_angle)) * dist;
    let distorted_uv = 0.5 + offset;
    
    let col_from = textureSampleLevel(t_from, s_diffuse, distorted_uv, 0.0).rgb;
    let col_to = textureSampleLevel(t_to, s_diffuse, uv, 0.0).rgb;
    
    let final_col = mix(col_from, col_to, smoothstep(0.0, 1.0, t));
    
    return vec4<f32>(final_col, 1.0);
}
