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

struct HazeData {
    intensity: f32,
    p1: f32,
    p2: f32,
    p3: f32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> data: HazeData;
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
    let steps = 32.0;
    let weight = 1.0 / steps;
    var col = vec3<f32>(0.0);
    
    let dir = in.uv - 0.5;
    let blur_dist = pow(data.intensity, 1.5) * 0.4;
    
    let chromatic_shift = data.intensity * 0.005;
    
    for (var i = 0.0; i < 32.0; i += 1.0) {
        let t_ratio = i / steps;
        let scale = 1.0 - (t_ratio * blur_dist);
        
        let offset = t_ratio * chromatic_shift;
        let uv_r = 0.5 + dir * scale + vec2<f32>(offset, 0.0);
        let uv_g = 0.5 + dir * scale;
        let uv_b = 0.5 + dir * scale - vec2<f32>(offset, 0.0);
        
        let r = textureSampleLevel(t_diffuse, s_diffuse, uv_r, 0.0).r;
        let g = textureSampleLevel(t_diffuse, s_diffuse, uv_g, 0.0).g;
        let b = textureSampleLevel(t_diffuse, s_diffuse, uv_b, 0.0).b;
        
        let boost = 1.0 + t_ratio * data.intensity * 1.5;
        col += vec3<f32>(r, g, b) * weight * boost;
    }
    
    col = min(col, vec3<f32>(1.5));
    
    return vec4<f32>(col, 1.0);
}
