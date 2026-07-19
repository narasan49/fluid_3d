#import fluid3d::fluid_uniform::{FluidUniform, BOUNDARY_OPEN, BOUNDARY_WALL}

@group(0) @binding(0) var u_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(1) var v_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(2) var w_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(3) var u1: texture_storage_3d<rgba16float, read>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

const X = vec3u(1, 0, 0);
const Y = vec3u(0, 1, 0);
const Z = vec3u(0, 0, 1);

@compute @workgroup_size(8, 8, 4)
fn collocated_to_mac(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim_collocated = textureDimensions(u1);
    if any(gid >= dim_collocated) {
        return;
    }
    let u_plus = textureLoad(u1, gid).xyz;
    let u_minus = vec3f(
        textureLoad(u1, gid - X).x,
        textureLoad(u1, gid - Y).y,
        textureLoad(u1, gid - Z).z,
    );
    let u = 0.5 * (u_plus + u_minus);

    if all(gid < dim_collocated + X) {
        if gid.x == 0 {
            if fluid_uniform.boundary_condition_min.x == BOUNDARY_OPEN {
                textureStore(u_mac, gid, vec4f(u_plus.x, 0.0, 0.0, 0.0));
            } else {
                textureStore(u_mac, gid, vec4f(0.0));
            }
        } else if gid.x == dim_collocated.x {
            if fluid_uniform.boundary_condition_max.x == BOUNDARY_OPEN {
                textureStore(u_mac, gid, vec4f(u_minus.x, 0.0, 0.0, 0.0));
            } else {
                textureStore(u_mac, gid, vec4f(0.0));
            }
        } else {
            textureStore(u_mac, gid, vec4f(u.x, 0.0, 0.0, 0.0));
        }
    }

    if all(gid < dim_collocated + Y) {
        if gid.y == 0 {
            if fluid_uniform.boundary_condition_min.y == BOUNDARY_OPEN {
                textureStore(v_mac, gid, vec4f(u_plus.y, 0.0, 0.0, 0.0));
            } else {
                textureStore(v_mac, gid, vec4f(0.0));
            }
        } else if gid.y == dim_collocated.y {
            if fluid_uniform.boundary_condition_max.y == BOUNDARY_OPEN {
                textureStore(v_mac, gid, vec4f(u_minus.y, 0.0, 0.0, 0.0));
            } else {
                textureStore(v_mac, gid, vec4f(0.0));
            }
        } else {
            textureStore(v_mac, gid, vec4f(u.y, 0.0, 0.0, 0.0));
        }
    }

    if all(gid < dim_collocated + Z) {
        if gid.z == 0 {
            if fluid_uniform.boundary_condition_min.z == BOUNDARY_OPEN {
                textureStore(w_mac, gid, vec4f(u_plus.z, 0.0, 0.0, 0.0));
            } else {
                textureStore(w_mac, gid, vec4f(0.0));
            }
        } else if gid.z == dim_collocated.z {
            if fluid_uniform.boundary_condition_max.z == BOUNDARY_OPEN {
                textureStore(w_mac, gid, vec4f(u_minus.z, 0.0, 0.0, 0.0));
            } else {
                textureStore(w_mac, gid, vec4f(0.0));
            }
        } else {
            textureStore(w_mac, gid, vec4f(u.z, 0.0, 0.0, 0.0));
        }
    }
}