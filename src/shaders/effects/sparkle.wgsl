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
@group(1) @binding(1) var t_diffuse: texture_2d<f32>;
@group(1) @binding(2) var s_diffuse: sampler;

// Helpers
fn hash13(p: f32) -> vec3<f32> {
    var p3 = fract(vec3<f32>(p) * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.xxy + p3.yzz) * p3.zyx);
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
    let kick = globals.kick;
    let res = vec2<f32>(globals.res_x, globals.res_y);
    let uv = (in.uv - 0.5) * vec2<f32>(res.x / res.y, 1.0);
    
    var col = textureSampleLevel(t_diffuse, s_diffuse, in.uv, 0.0).rgb;
    
    let ro = vec3<f32>(0.0, 0.0, -3.0);
    let rd = normalize(vec3<f32>(uv, 1.5));
    
    for (var i: f32 = 0.0; i < 40.0; i += 1.0) {
        let h = hash13(i * 123.456);
        let speed = 0.2 + h.x * 0.5;
        let p = (h - 0.5) * 10.0;
        
        let pos = p + vec3<f32>(
            sin(t * speed + h.y * 6.28),
            cos(t * speed * 0.8 + h.z * 6.28),
            sin(t * speed * 1.2)
        ) * (2.0 + kick * 2.0);
        
        let pa = ro - pos;
        let ba = rd;
        let d = length(pa - ba * dot(pa, ba));
        
        let brightness = (0.002 / (d * d + 0.001)) * smoothstep(0.4, 0.1, d);
        let p_col = h * vec3<f32>(1.0, 0.7, 0.4) + vec3<f32>(0.5, 0.5, 1.0) * (1.0 - h.x);
        col += p_col * brightness * (0.5 + kick * 0.5) * smoothstep(10.0, 0.0, length(pos - ro));
    }
    
    return vec4<f32>(col, 1.0);
}
