#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u1: texture_storage_3d<rgba16float, read>;
@group(0) @binding(1) var u_solid: texture_storage_3d<rgba16float, read>;
@group(0) @binding(2) var fluid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(3) var div: texture_storage_3d<r32float, write>;
@group(0) @binding(4) var levelset_solid: texture_storage_3d<r32float, read>;
@group(0) @binding(5) var levelset_air0: texture_storage_3d<r32float, read>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@compute @workgroup_size(8, 8, 4)
fn divergence(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u1);
    if any(gid >= dim) {
        return;
    }
    let level_center = textureLoad(levelset_air0, gid).x;
    if level_center >= 0.0 {
        textureStore(div, gid, vec4f(0.0));
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
    let u_solid_center = textureLoad(u_solid, gid).xyz;
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
    
    let f_minus = textureLoad(fluid_fraction, gid).xyz;
    let f_plus = vec3f(
        textureLoad(fluid_fraction, gid + vec3u(1, 0, 0)).x,
        textureLoad(fluid_fraction, gid + vec3u(0, 1, 0)).y,
        textureLoad(fluid_fraction, gid + vec3u(0, 0, 1)).z,
    );

    let level_solid_plus = vec3f(
        textureLoad(levelset_solid, gid + vec3u(1, 0, 0)).x,
        textureLoad(levelset_solid, gid + vec3u(0, 1, 0)).x,
        textureLoad(levelset_solid, gid + vec3u(0, 0, 1)).x,
    );
    let level_solid_minus = vec3f(
        textureLoad(levelset_solid, gid - vec3u(1, 0, 0)).x,
        textureLoad(levelset_solid, gid - vec3u(0, 1, 0)).x,
        textureLoad(levelset_solid, gid - vec3u(0, 0, 1)).x,
    );

    let level_air_plus = vec3f(
        textureLoad(levelset_air0, gid + vec3u(1, 0, 0)).x,
        textureLoad(levelset_air0, gid + vec3u(0, 1, 0)).x,
        textureLoad(levelset_air0, gid + vec3u(0, 0, 1)).x,
    );
    let level_air_minus = vec3f(
        textureLoad(levelset_air0, gid - vec3u(1, 0, 0)).x,
        textureLoad(levelset_air0, gid - vec3u(0, 1, 0)).x,
        textureLoad(levelset_air0, gid - vec3u(0, 0, 1)).x,
    );

    // var du = vec3f(0.0);
    // if level_solid_plus.x >= 0.0 {
    //     du.x += 0.5 * (u_plus.x + u_center.x);
    // } else {
    //     du.x += u_solid_plus.x;
    // }
    // if level_solid_minus.x >= 0.0 {
    //     du.x -= 0.5 * (u_center.x + u_minus.x);
    // } else {
    //     du.x -= u_solid_minus.x;
    // }

    // if level_solid_plus.y >= 0.0 {
    //     du.y += 0.5 * (u_plus.y + u_center.y);
    // } else {
    //     du.y += u_solid_plus.y;
    // }
    // if level_solid_minus.y >= 0.0 {
    //     du.y -= 0.5 * (u_center.y + u_minus.y);
    // } else {
    //     du.y -= u_solid_minus.y;
    // }

    // if level_solid_plus.z >= 0.0 {
    //     du.z += 0.5 * (u_plus.z + u_center.z);
    // } else {
    //     du.z += u_solid_plus.z;
    // }
    // if level_solid_minus.z >= 0.0 {
    //     du.z -= 0.5 * (u_center.z + u_minus.z);
    // } else {
    //     du.z -= u_solid_minus.z;
    // }

    // if level_air_plus.x >= 0.0 || level_air_minus.x == 0.0 {
    //     du.x = 0.0;
    // }
    // if level_air_plus.y >= 0.0 || level_air_minus.y == 0.0 {
    //     du.y = 0.0;
    // }
    // if level_air_plus.z >= 0.0 || level_air_minus.z == 0.0 {
    //     du.z = 0.0;
    // }

    // let result = - (du.x + du.y + du.z) / fluid_uniform.dx;
    // var du = f_plus * 0.5 * (u_plus + u_center) - f_minus * 0.5 * (u_minus + u_center);
    // let du_solid = (1.0 - f_plus) * 0.5 * (u_solid_plus + u_solid_center) - (1.0 - f_minus) * 0.5 * (u_solid_minus + u_solid_center);
    // if level_air_plus.x >= 0.0 || level_air_minus.x == 0.0 {
    //     du.x = 0.0;
    // }
    // if level_air_plus.y >= 0.0 || level_air_minus.y == 0.0 {
    //     du.y = 0.0;
    // }
    // if level_air_plus.z >= 0.0 || level_air_minus.z == 0.0 {
    //     du.z = 0.0;
    // }
    // let result = - (du.x + du.y + du.z + du_solid.x + du_solid.y + du_solid.z) / fluid_uniform.dx;
    
    var du = f_plus * u_plus - f_minus * u_minus;
    let du_solid = (1.0 - f_plus) * u_solid_plus - (1.0 - f_minus) * u_solid_minus;
    if level_air_plus.x >= 0.0 || level_air_minus.x == 0.0 {
        du.x = 0.0;
    }
    if level_air_plus.y >= 0.0 || level_air_minus.y == 0.0 {
        du.y = 0.0;
    }
    if level_air_plus.z >= 0.0 || level_air_minus.z == 0.0 {
        du.z = 0.0;
    }
    let result = - 0.5 * (du.x + du.y + du.z + du_solid.x + du_solid.y + du_solid.z) / fluid_uniform.dx;

    textureStore(div, gid, vec4f(result, 0.0, 0.0, 0.0));
}