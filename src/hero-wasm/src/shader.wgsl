// only xy is used
@group(0) @binding(0)
var<uniform> body_pos: array<vec4<f32>, 3>;
@group(0) @binding(1)
var<uniform> origin: vec4<f32>;
// array of f32 has the wrong alignment
@group(0) @binding(2)
var<uniform> body_mass: vec4<f32>;
// only x is used
@group(0) @binding(3)
var<uniform> scale: vec4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    var x: f32 = 0.0;
    var y: f32 = 0.0;

    if in_vertex_index == 0u {
        x = -1.0;
        y = -1.0;
    } else if in_vertex_index == 1u || in_vertex_index == 4u {
        x = 1.0;
        y = -1.0;
    } else if in_vertex_index == 2u || in_vertex_index == 3u {
        x = -1.0;
        y = 1.0;
    } else if in_vertex_index == 5u {
        x = 1.0;
        y = 1.0;
    }

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x, y);
    return out;
}

fn remap(value: f32) -> f32 {
    return 1.0 - 1.0 / (1.0 + value);
}

const BRIGHTNESS: f32 = 0.05;
const RADIUS: f32 = 1.0;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv_dist = length(in.tex_coords * scale.x + origin.xy) / scale.x;

    if uv_dist > 0.035 && uv_dist < 0.055 {
        return vec4<f32>(0.9, 0.9, 0.9, 1.0);
    }

    let current_pos = in.tex_coords * scale.x + origin.xy;

    var body_1_vec = body_pos[0].xy - current_pos;
    let body_1_dist = length(body_1_vec);

    if body_1_dist < RADIUS {
        return vec4<f32>(1.0, 0.2, 0.2, 1.0);
    }
    body_1_vec /= body_1_dist;

    var body_2_vec = body_pos[1].xy - current_pos;
    let body_2_dist = length(body_2_vec);

    if body_2_dist < RADIUS {
        return vec4<f32>(0.2, 1.0, 0.2, 1.0);
    }
    body_2_vec /= body_2_dist;

    var body_3_vec = body_pos[2].xy - current_pos;
    let body_3_dist = length(body_3_vec);

    if body_3_dist < RADIUS {
        return vec4<f32>(0.2, 0.2, 1.0, 1.0);
    }
    body_3_vec /= body_3_dist;

    let body_1_accel_vec = body_mass.x / (body_1_dist * body_1_dist) * body_1_vec;
    let body_2_accel_vec = body_mass.y / (body_2_dist * body_2_dist) * body_2_vec;
    let body_3_accel_vec = body_mass.z / (body_3_dist * body_3_dist) * body_3_vec;

    let accel_vec = normalize(body_1_accel_vec + body_2_accel_vec + body_3_accel_vec);
    
    let red = max(0.0, dot(accel_vec, body_1_accel_vec));
    let green = max(0.0, dot(accel_vec, body_2_accel_vec));
    let blue = max(0.0, dot(accel_vec, body_3_accel_vec));

    var rgb = vec3<f32>(
        remap(red),
        remap(green),
        remap(blue)
    );

    // rgb += pow(rgb, vec3<f32>(0.5, 0.5, 0.5)) * BRIGHTNESS;

    return vec4<f32>(
        rgb.x, rgb.y, rgb.z,
        1.0
    );
}
 