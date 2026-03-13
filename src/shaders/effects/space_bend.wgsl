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

struct SpaceBendData {
    mix_factor: f32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> data: SpaceBendData;
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
    let aspect = res.x / res.y;
    let uv = in.uv;
    
    let p = (uv - 0.5) * vec2<f32>(aspect, 1.0);
    let d = length(p);
    
    let radius = pow(t, 1.5) * 2.0;
    let front_pos = radius + 0.25; 
    
    let causality = smoothstep(front_pos + 0.05, front_pos - 0.05, d);
    
    let curve_strength = 0.4 * smoothstep(0.0, 0.2, t);
    let bulge = 1.0 + curve_strength * exp(-pow(d - radius, 2.0) * 8.0) * causality;
    let curved_p = p / bulge;
    
    let p2 = vec2<f32>(0.25 * aspect, -0.15);
    let d2 = length(p - p2);
    
    let wave_env = exp(-abs(d - front_pos) * 4.0) * causality;
    
    let ripples1 = (sin(d * 35.0 - t * 20.0) * 0.06 + sin(d * 12.0 - t * 10.0) * 0.08);
    let ripples2 = sin(d2 * 25.0 - t * 15.0) * 0.04;
    
    let total_ripple = (ripples1 + ripples2) * wave_env * (1.0 - t * 0.5);
    
    let dir = normalize(p + 0.00001);
    let distorted_p = curved_p + dir * total_ripple;
    let distorted_uv = (distorted_p / vec2<f32>(aspect, 1.0)) + 0.5;
    
    let mask_dist = length(distorted_p);
    let softness = 0.01; 
    let mask = smoothstep(radius - softness, radius + softness, mask_dist);
    
    let col_from = textureSampleLevel(t_from, s_diffuse, distorted_uv, 0.0).rgb;
    let col_to = textureSampleLevel(t_to, s_diffuse, uv, 0.0).rgb;
    
    let final_col = mix(col_to, col_from, mask);
    
    return vec4<f32>(final_col, 1.0);
}
