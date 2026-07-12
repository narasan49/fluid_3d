#import bevy_pbr::mesh_functions
#import bevy_render::view::View

// ToDo: アライメントの都合でvec4を使っている。要修正。
struct Vertex {
    @location(0) position: vec4f,
    @location(1) normal: vec4f,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) world_position: vec4f,
    @location(1) world_normal: vec3f,
}

struct MarchingCubesUniform {
    world_from_local: mat4x4f,
}

@group(0) @binding(0) var<uniform> mc_uniform: MarchingCubesUniform;
@group(1) @binding(0) var<uniform> view: View;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.world_position = mesh_functions::mesh_position_local_to_world(mc_uniform.world_from_local, vertex.position);
    out.clip_position = view.clip_from_world * out.world_position;
    out.world_normal = normalize(mat3x3f(
        mc_uniform.world_from_local[0].xyz,
        mc_uniform.world_from_local[1].xyz,
        mc_uniform.world_from_local[2].xyz,
    ) * vertex.normal.xyz);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4f {
    let normal = in.world_normal.xyz;
    let light_direction = normalize(vec3f(0.5, 0.3, 0.5));
    let light_incidence = max(dot(normal, light_direction), 0.0);
    let albedo = vec3f(0.5, 0.78, 0.83);

    let color = light_incidence * albedo;

    return vec4f(color, 1.0);
}