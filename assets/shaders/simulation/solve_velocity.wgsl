#import fluid3d::area_fraction::{load_area_fraction, fully_solid}
#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u0: texture_storage_3d<rgba16float, write>;
@group(0) @binding(1) var u1: texture_storage_3d<rgba16float, read>;
@group(0) @binding(2) var p: texture_storage_3d<r32float, read>;
@group(0) @binding(3) var fluid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(4) var u_solid: texture_storage_3d<rgba16float, read>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@compute @workgroup_size(8, 8, 4)
fn solve_velocity(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u0);
    if any(gid >= dim) {
        return;
    }
    let f = load_area_fraction(fluid_fraction, vec3i(gid));
    if fully_solid(f) {
        textureStore(u0, gid, textureLoad(u_solid, gid));
        return;
    }

    let factor = fluid_uniform.dt / (2.0 * fluid_uniform.dx * fluid_uniform.rho);

    var du = vec3f(0.0);
    let p_center = textureLoad(p, gid).x;
    if gid.x > 0 && gid.x < (dim.x - 1) {
        let p_xplus = (1.0 - f[1]) * p_center + f[1] * textureLoad(p, gid + vec3u(1, 0, 0)).x;
        let p_xminus = (1.0 - f[0]) * p_center + f[0] * textureLoad(p, gid - vec3u(1, 0, 0)).x;
        
        du.x = factor * (p_xplus - p_xminus);
    }
    if gid.y > 0 && gid.y < (dim.y - 1) {
        let p_yplus = (1.0 - f[3]) * p_center + f[3] * textureLoad(p, gid + vec3u(0, 1, 0)).x;
        let p_yminus = (1.0 - f[2]) * p_center + f[2] * textureLoad(p, gid - vec3u(0, 1, 0)).x;
        du.y = factor * (p_yplus - p_yminus);
    }
    if gid.z > 0 && gid.z < (dim.z - 1) {
        let p_zplus = (1.0 - f[5]) * p_center + f[5] * textureLoad(p, gid + vec3u(0, 0, 1)).x;
        let p_zminus = (1.0 - f[4]) * p_center + f[4] * textureLoad(p, gid - vec3u(0, 0, 1)).x;
        du.z = factor * (p_zplus - p_zminus);
    }

    let u = textureLoad(u1, gid).xyz;
    textureStore(u0, gid, vec4f(u - du, 0.0));
}