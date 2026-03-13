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

@group(0) @binding(0) var<uniform> globals: Globals;

struct FlipData {
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    dist: f32,
    offset_x: f32,
    offset_y: f32,
    curvature: f32,
    _pad: f32,
}

@group(1) @binding(0) var<uniform> flip_data: FlipData;
@group(1) @binding(1) var t_front: texture_2d<f32>;
@group(1) @binding(2) var s_diffuse: sampler;
@group(1) @binding(3) var t_back: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((i32(idx) << 1u) & 2);
    let y = f32(i32(idx) & 2);
    out.pos = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    out.uv = vec2<f32>(x, y);
    return out;
}

fn rotateX(v: vec3<f32>, a: f32) -> vec3<f32> {
    let s = sin(a);
    let c = cos(a);
    return vec3<f32>(v.x, c * v.y - s * v.z, s * v.y + c * v.z);
}

fn rotateY(v: vec3<f32>, a: f32) -> vec3<f32> {
    let s = sin(a);
    let c = cos(a);
    return vec3<f32>(c * v.x + s * v.z, v.y, -s * v.x + c * v.z);
}

fn rotateZ(v: vec3<f32>, a: f32) -> vec3<f32> {
    let s = sin(a);
    let c = cos(a);
    return vec3<f32>(c * v.x - s * v.y, s * v.x + c * v.y, v.z);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let res = vec2<f32>(globals.res_x, globals.res_y);
    let aspect = res.x / res.y;
    let uv = (in.uv - 0.5) * vec2<f32>(aspect, 1.0);

    let ro_world = vec3<f32>(-flip_data.offset_x * aspect, -flip_data.offset_y, -flip_data.dist);
    let rd_world = normalize(vec3<f32>(uv, 2.0));

    var ro = ro_world;
    var rd = rd_world;

    ro = rotateZ(ro, -flip_data.rot_z);
    rd = rotateZ(rd, -flip_data.rot_z);
    ro = rotateY(ro, -flip_data.rot_y);
    rd = rotateY(rd, -flip_data.rot_y);
    ro = rotateX(ro, -flip_data.rot_x);
    rd = rotateX(rd, -flip_data.rot_x);

    let c = flip_data.curvature;
    let A = c * (rd.x * rd.x + rd.y * rd.y);
    let B = 2.0 * c * (ro.x * rd.x + ro.y * rd.y) - rd.z;
    let C_val = c * (ro.x * ro.x + ro.y * ro.y) - ro.z;

    var t_hit = -1.0;
    if (abs(A) < 0.0001) {
        if (abs(B) > 0.0001) {
            t_hit = -C_val / B;
        }
    } else {
        let disc = B * B - 4.0 * A * C_val;
        if (disc >= 0.0) {
            let t1 = (-B - sqrt(disc)) / (2.0 * A);
            let t2 = (-B + sqrt(disc)) / (2.0 * A);
            if (t1 > 0.0) {
                t_hit = t1;
            } else if (t2 > 0.0) {
                t_hit = t2;
            }
        }
    }

    if (t_hit < 0.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let p = ro + rd * t_hit;
    let lx = p.x / aspect;
    let ly = p.y;

    if (abs(lx) > 0.5 || abs(ly) > 0.5) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let tex_uv = vec2<f32>(lx + 0.5, 0.5 - ly);

    let n_local = normalize(vec3<f32>(-2.0 * c * p.x, -2.0 * c * p.y, 1.0));
    var n_world = n_local;
    n_world = rotateX(n_world, flip_data.rot_x);
    n_world = rotateY(n_world, flip_data.rot_y);
    n_world = rotateZ(n_world, flip_data.rot_z);

    let viewing_front = dot(rd_world, n_world) < 0.0;

    var col: vec3<f32>;
    if (viewing_front) {
        col = textureSampleLevel(t_front, s_diffuse, vec2<f32>(1.0 - tex_uv.x, 1.0 - tex_uv.y), 0.0).rgb;
    } else {
        col = textureSampleLevel(t_back, s_diffuse, vec2<f32>(tex_uv.x, 1.0 - tex_uv.y), 0.0).rgb;
    }

    let lighting = abs(dot(rd_world, n_world)) * 0.8 + 0.2;
    return vec4<f32>(col * lighting, 1.0);
}
