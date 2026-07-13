#define_import_path fluid3d::fluid_uniform

struct FluidUniform {
    dx: f32,
    dt: f32,
    rho: f32,
    gravity: vec3f,
    transform: mat4x4f,
    resolution: vec3u,
    half_size: vec3f,
}