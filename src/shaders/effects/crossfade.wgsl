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

struct CrossfadeData {
    mix_factor: f32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> data: CrossfadeData;
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
    let res = vec2<f32>(globals.res_x, globals.res_y);
    let p = (in.uv - 0.5) * vec2<f32>(res.x / res.y, 1.0);
    
    let dist = length(p);
    
    let radius = t * 1.5; 
    let edge_width = 0.05;
    
    let mask = smoothstep(radius - edge_width, radius + edge_width, dist);
    
    let col_from = textureSampleLevel(t_from, s_diffuse, in.uv, 0.0).rgb;
    let col_to = textureSampleLevel(t_to, s_diffuse, in.uv, 0.0).rgb;
    
    var final_col = mix(col_to, col_from, mask);
    
    let ring = smoothstep(0.1, 0.0, abs(dist - radius));
    let ring_col = vec3<f32>(0.5, 0.8, 1.0) * ring * (1.0 - t);
    
    final_col += ring_col;
    
    return vec4<f32>(final_col, 1.0);
}
