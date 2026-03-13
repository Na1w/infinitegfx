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

struct CurtainData {
    mix_factor: f32,
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> data: CurtainData;
@group(1) @binding(1) var t_diffuse: texture_2d<f32>;
@group(1) @binding(2) var s_diffuse: sampler;

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
    let bg_col = textureSampleLevel(t_diffuse, s_diffuse, in.uv, 0.0).rgb;
    let curtain_y = data.mix_factor;
    let shutter_col = vec3<f32>(0.005, 0.006, 0.008);
    
    let dist_to_edge = abs(in.uv.y - curtain_y);
    let edge_glow = exp(-dist_to_edge * 60.0);
    let laser_color = vec3<f32>(0.0, 0.6, 1.0);
    
    var final_col = bg_col;
    
    if (in.uv.y < curtain_y) {
        final_col = shutter_col * (1.0 - in.uv.y * 0.1);
    } else {
        final_col = bg_col;
    }
    
    final_col += laser_color * edge_glow * 1.2;
    let core = exp(-dist_to_edge * 250.0) * 1.5;
    final_col += vec3<f32>(1.0) * core;

    return vec4<f32>(final_col, 1.0);
}
