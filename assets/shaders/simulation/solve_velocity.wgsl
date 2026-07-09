#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u0: texture_storage_3d<rgba16float, write>;
@group(0) @binding(1) var u1: texture_storage_3d<rgba16float, read>;
@group(0) @binding(2) var p: texture_storage_3d<r32float, read>;
@group(0) @binding(3) var fluid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(4) var u_solid: texture_storage_3d<rgba16float, read>;
@group(0) @binding(5) var levelset_air0: texture_storage_3d<r32float, read>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

const X = vec3u(1, 0, 0);
const Y = vec3u(0, 1, 0);
const Z = vec3u(0, 0, 1);

@compute @workgroup_size(8, 8, 4)
fn solve_velocity(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u0);
    if any(gid >= dim) {
        return;
    }
    let level_center = textureLoad(levelset_air0, gid).x;
    if level_center >= 0 {
        textureStore(u0, gid, vec4f(0.0));
        return;
    }
    let f_minus = textureLoad(fluid_fraction, gid).xyz;
    let f_plus = vec3f(
        textureLoad(fluid_fraction, gid + X).x,
        textureLoad(fluid_fraction, gid + Y).y,
        textureLoad(fluid_fraction, gid + Z).z,
    );
    if all(f_minus == vec3f(0.0)) && all(f_plus == vec3f(0.0)) {
        textureStore(u0, gid, textureLoad(u_solid, gid));
        return;
    }

    let factor = fluid_uniform.dt / (2.0 * fluid_uniform.dx * fluid_uniform.rho);

    var du = vec3f(0.0);
    let p_center = textureLoad(p, gid).x;
    let p_minus = vec3f(
        textureLoad(p, gid - X).x,
        textureLoad(p, gid - Y).x,
        textureLoad(p, gid - Z).x,
    );
    let p_plus = vec3f(
        textureLoad(p, gid + X).x,
        textureLoad(p, gid + Y).x,
        textureLoad(p, gid + Z).x,
    );
    let level_minus = vec3f(
        textureLoad(levelset_air0, gid - X).x,
        textureLoad(levelset_air0, gid - Y).x,
        textureLoad(levelset_air0, gid - Z).x,
    );
    let level_plus = vec3f(
        textureLoad(levelset_air0, gid + X).x,
        textureLoad(levelset_air0, gid + Y).x,
        textureLoad(levelset_air0, gid + Z).x,
    );
    if gid.x > 0 && gid.x < (dim.x - 1) {
        let px_plus = adjacent_pressure(level_center, level_plus.x, p_center, p_plus.x, f_plus.x);
        let px_minus = adjacent_pressure(level_center, level_minus.x, p_center, p_minus.x, f_minus.x);
        du.x = factor * (px_plus - px_minus);
    }
    if gid.y > 0 && gid.y < (dim.y - 1) {
        let py_plus = adjacent_pressure(level_center, level_plus.y, p_center, p_plus.y, f_plus.y);
        let py_minus = adjacent_pressure(level_center, level_minus.y, p_center, p_minus.y, f_minus.y);
        du.y = factor * (py_plus - py_minus);
    }
    if gid.z > 0 && gid.z < (dim.z - 1) {
        let pz_plus = adjacent_pressure(level_center, level_plus.z, p_center, p_plus.z, f_plus.z);
        let pz_minus = adjacent_pressure(level_center, level_minus.z, p_center, p_minus.z, f_minus.z);
        du.z = factor * (pz_plus - pz_minus);
    }

    let u = textureLoad(u1, gid).xyz;
    textureStore(u0, gid, vec4f(u - du, 0.0));
}

fn adjacent_pressure(level: f32, level_adj: f32, p: f32, p_adj: f32, f:f32) -> f32 {
    if level_adj >= 0.0 {
        return p * level_adj / level;
    } else {
        return (1.0 - f) * p + f * p_adj;
    }
}