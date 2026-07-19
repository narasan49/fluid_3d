#define_import_path fluid3d::fluid_uniform

const BOUNDARY_WALL = 0u;
const BOUNDARY_OPEN = 1u;

struct FluidUniform {
    dx: f32,
    dt: f32,
    rho: f32,
    gravity: vec3f,
    transform: mat4x4f,
    resolution: vec3u,
    half_size: vec3f,
    boundary_condition_min: vec3u,
    boundary_condition_max: vec3u,
}