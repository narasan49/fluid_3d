#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u1: texture_storage_3d<rgba16float, read>;
@group(0) @binding(1) var u_solid: texture_storage_3d<rgba16float, read>;
@group(0) @binding(2) var solid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(3) var div: texture_storage_3d<r32float, write>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@compute @workgroup_size(8, 8, 4)
fn divergence(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u1);
    if any(gid >= dim) {
        return;
    }
    if any(gid == vec3u(0)) || any(gid == (dim - vec3u(1))) {
        return;
    }

    let u_center = textureLoad(u1, gid).xyz;
    let u_minus = vec3f(
        textureLoad(u1, gid - vec3u(1, 0, 0)).x,
        textureLoad(u1, gid - vec3u(0, 1, 0)).y,
        textureLoad(u1, gid - vec3u(0, 0, 1)).z,
    );
    let u_plus = vec3f(
        textureLoad(u1, gid + vec3u(1, 0, 0)).x,
        textureLoad(u1, gid + vec3u(0, 1, 0)).y,
        textureLoad(u1, gid + vec3u(0, 0, 1)).z,
    );
    let u_solid_center = textureLoad(u1, gid).xyz;
    let u_solid_minus = vec3f(
        textureLoad(u_solid, gid - vec3u(1, 0, 0)).x,
        textureLoad(u_solid, gid - vec3u(0, 1, 0)).y,
        textureLoad(u_solid, gid - vec3u(0, 0, 1)).z,
    );
    let u_solid_plus = vec3f(
        textureLoad(u_solid, gid + vec3u(1, 0, 0)).x,
        textureLoad(u_solid, gid + vec3u(0, 1, 0)).y,
        textureLoad(u_solid, gid + vec3u(0, 0, 1)).z,
    );
    
    let f_minus = textureLoad(solid_fraction, gid).xyz;
    let f_plus = vec3f(
        textureLoad(solid_fraction, gid + vec3u(1, 0, 0)).x,
        textureLoad(solid_fraction, gid + vec3u(0, 1, 0)).y,
        textureLoad(solid_fraction, gid + vec3u(0, 0, 1)).z,
    );
    
    let du = (1.0 - f_plus) * 0.5 * (u_plus + u_center) - (1.0 - f_minus) * 0.5 * (u_minus + u_center);
    let du_solid = f_plus * 0.5 * (u_solid_plus + u_solid_center) - f_minus * 0.5 * (u_solid_minus + u_solid_center);
    let result = - (du.x + du.y + du.z + du_solid.x + du_solid.y + du_solid.z) / fluid_uniform.dx;

    textureStore(div, gid, vec4f(result, 0.0, 0.0, 0.0));
}