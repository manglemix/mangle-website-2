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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let current_pos = in.tex_coords * scale.x - origin.xy;
    var dist = distance(current_pos, body_pos[0].xy);
    if dist < 1.0 {
        return vec4<f32>(1.0, 0.2, 0.2, 1.0);
    }
    let red = remap(body_mass.x / (dist * dist));

    dist = distance(current_pos, body_pos[1].xy);
    if dist < 1.0 {
        return vec4<f32>(0.2, 1.0, 0.2, 1.0);
    }
    let green = remap(body_mass.y / (dist * dist));

    dist = distance(current_pos, body_pos[2].xy);
    if dist < 1.0 {
        return vec4<f32>(0.2, 0.2, 1.0, 1.0);
    }
    let blue = remap(body_mass.z / (dist * dist));

    return vec4<f32>(red, green, blue, 1.0);
}
 