#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u1: texture_storage_3d<rgba16float, read_write>;
@group(0) @binding(1) var levelset_air0: texture_storage_3d<r32float, read>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@compute @workgroup_size(8, 8, 4)
fn apply_forces(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u1);
    if any(gid >= dim) {
        return;
    }

    let level = textureLoad(levelset_air0, gid).x;
    if level >= 0.0 {
        textureStore(u1, gid, vec4f(0.0));
        return;
    }

    let delta = fluid_uniform.gravity * fluid_uniform.dt / fluid_uniform.dx;
    let velocity = textureLoad(u1, gid).xyz;
    textureStore(u1, gid, vec4f(velocity + delta, 0.0));
}