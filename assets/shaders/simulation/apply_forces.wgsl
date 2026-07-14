#import fluid3d::fluid_uniform::FluidUniform
#import fluid3d::area_fraction::{load_area_fraction, fully_solid}

@group(0) @binding(0) var u1: texture_storage_3d<rgba16float, read_write>;
@group(0) @binding(1) var levelset_air0: texture_storage_3d<r32float, read>;
@group(0) @binding(2) var non_solid_fraction: texture_storage_3d<rgba16float, read>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

const CFL_SCALE: f32 = 5.0;

@compute @workgroup_size(8, 8, 4)
fn apply_forces(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u1);
    if any(gid >= dim) {
        return;
    }

    let level = textureLoad(levelset_air0, gid).x;
    if level >= 3.0 {
        textureStore(u1, gid, vec4f(0.0));
        return;
    }

    let f = load_area_fraction(non_solid_fraction, vec3i(gid));
    if fully_solid(f) {
        textureStore(u1, gid, vec4f(0.0));
        return;
    }

    let delta = fluid_uniform.gravity * fluid_uniform.dt / fluid_uniform.dx;
    let velocity = textureLoad(u1, gid).xyz;
    var new_velocity = velocity + delta;
    let cfl_speed = CFL_SCALE / fluid_uniform.dt;
    if length(new_velocity) > cfl_speed {
        new_velocity = cfl_speed * normalize(new_velocity);
    }
    textureStore(u1, gid, vec4f(new_velocity, 0.0));
}