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

struct EffectParams {
    intensity: f32,
    sustained_kick: f32,
    _pad2: f32,
    _pad3: f32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> params: EffectParams;
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
    let kick = params.sustained_kick;
    let final_intensity = params.intensity;
    
    var col = textureSampleLevel(t_diffuse, s_diffuse, in.uv, 0.0).rgb;

    if (final_intensity > 0.0 && kick > 0.01) {
        var glitch_uv = in.uv;
        
        if (hash(vec2<f32>(floor(t * 24.0), 0.0)) < (kick * final_intensity)) {
            let block_y = floor(in.uv.y * 15.0);
            if (hash(vec2<f32>(block_y, floor(t * 15.0))) > 0.3) {
                glitch_uv.x += (hash(vec2<f32>(block_y, t)) - 0.5) * 0.5 * kick * final_intensity;
            }
        }
        
        col = textureSampleLevel(t_diffuse, s_diffuse, glitch_uv, 0.0).rgb;

        let block_uv = floor(in.uv * vec2<f32>(320.0, 240.0));
        let noise = hash(block_uv + t * 15.0);
        let noise_mask = step(0.01, length(col));
        col += (noise - 0.5) * 0.1 * kick * final_intensity * noise_mask; 
        
        if (kick > 0.5) {
            let split = 0.02 * kick * final_intensity;
            let r_col = textureSampleLevel(t_diffuse, s_diffuse, glitch_uv + vec2<f32>(split, 0.0), 0.0).r;
            let b_col = textureSampleLevel(t_diffuse, s_diffuse, glitch_uv - vec2<f32>(split, 0.0), 0.0).b;
            col.r = mix(col.r, r_col, noise_mask);
            col.b = mix(col.b, b_col, noise_mask);
        }
    }
    
    return vec4<f32>(col, 1.0);
}
