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

struct TextData { 
    char_ids: array<vec4<i32>, 8>,
    num_chars: i32,
    offset_x: f32,
    offset_y: f32,
    scale: f32,
    start_time: f32,
    end_time: f32,
    fade_in: f32,
    fade_out: f32,
    color: vec3<f32>,
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

struct FontData {
    chars: array<vec4<u32>, 14>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Bindings
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> text_data: TextData;
@group(1) @binding(1) var t_diffuse: texture_2d<f32>;
@group(1) @binding(2) var s_diffuse: sampler;
@group(2) @binding(0) var<uniform> font_data: FontData;

// Helpers
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn hash22(p: vec2<f32>) -> vec2<f32> {
    var p3 = fract(vec3<f32>(p.xyx) * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.xx + p3.yz) * p3.zy);
}

fn get_char(c: i32, p: vec2<i32>) -> f32 {
    if (p.x < 0 || p.x > 7 || p.y < 0 || p.y > 7 || c < 0 || c > 26) {
        return 0.0;
    }
    
    let bit_idx = u32(p.x + p.y * 8);
    let u32_idx = u32(c) * 2u + (bit_idx / 32u);
    let local_bit = bit_idx % 32u;
    
    let vec_idx = u32_idx / 4u;
    let comp_idx = u32_idx % 4u;
    let v = font_data.chars[vec_idx];
    
    var val = 0u;
    if (comp_idx == 0u) {
        val = v.x;
    } else if (comp_idx == 1u) {
        val = v.y;
    } else if (comp_idx == 2u) {
        val = v.z;
    } else {
        val = v.w;
    }
    
    return f32((val >> local_bit) & 1u);
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
    let res = vec2<f32>(globals.res_x, globals.res_y);
    var col = textureSampleLevel(t_diffuse, s_diffuse, in.uv, 0.0).rgb;

    if (t >= text_data.start_time && t < text_data.end_time) {
        let p_uv = (in.uv - 0.5) * vec2<f32>(res.x / res.y, 1.0);
        
        let tp_orig = p_uv * text_data.scale;
        var tp = tp_orig;

        let atom_size = 0.5; 
        let seed = floor(tp_orig / atom_size);

        let wipe_dir = vec2<f32>(cos(text_data.wipe_angle), sin(text_data.wipe_angle));
        let dist_along_wipe = dot(tp_orig, wipe_dir);
        
        let wipe_progress = text_data.explosion * (200.0 + text_data.wipe_width) - 100.0;
        let local_ex = max(0.0, (wipe_progress - dist_along_wipe) / text_data.wipe_width);

        if (local_ex > 0.0) {
            let noise_val = hash22(seed) - 0.5;
            let velocity = noise_val * 60.0;
            let grav = vec2<f32>(text_data.gravity_x, text_data.gravity_y);
            
            tp -= velocity * local_ex + 0.5 * grav * local_ex * local_ex;
        }
        
        var tx = tp.x + text_data.offset_x;
        var ty = tp.y + text_data.offset_y;

        if (text_data.mod_x_wavelength > 0.0) {
            tx += text_data.mod_x_amp * sin(ty * 6.28318 / text_data.mod_x_wavelength + t * 5.0);
        }
        if (text_data.mod_y_wavelength > 0.0) {
            ty += text_data.mod_y_amp * sin(tx * 6.28318 / text_data.mod_y_wavelength + t * 5.0);
        }
        
        let char_spacing = 9.0;
        let idx = i32(floor(tx / char_spacing));
        
        var char_id = -1;
        if (idx >= 0 && idx < text_data.num_chars && ty > -4.0 && ty < 4.0) {
            let v_idx = idx / 4;
            let c_idx = idx % 4;
            let val = text_data.char_ids[v_idx];
            if (c_idx == 0) {
                char_id = val.x;
            } else if (c_idx == 1) {
                char_id = val.y;
            } else if (c_idx == 2) {
                char_id = val.z;
            } else {
                char_id = val.w;
            }
        }

        if (char_id != -1) {
            let local_p = vec2<i32>(i32(floor(tx % char_spacing)), i32(floor(ty + 4.0)));
            
            var intensity = get_char(char_id, local_p);
            
            if (local_ex > 0.0) {
                let sparkle = hash(seed + t * 0.1);
                let visual_ex = clamp(local_ex, 0.0, 1.0);
                intensity *= (1.0 - visual_ex * 0.7) + sparkle * visual_ex * 1.5; 
            }

            let duration = text_data.end_time - text_data.start_time;
            let rel_t = t - text_data.start_time;
            
            let alpha_in = smoothstep(0.0, text_data.fade_in, rel_t);
            let alpha_out = smoothstep(duration, duration - text_data.fade_out, rel_t);
            let alpha = alpha_in * alpha_out;
            
            let text_col = text_data.color * intensity * alpha;
            col = max(col, text_col);
        }
    }
    
    return vec4<f32>(col, 1.0);
}
