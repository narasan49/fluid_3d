#import fluid3d::fluid_uniform::{FluidUniform, BOUNDARY_OPEN, BOUNDARY_WALL}
#import fluid3d::constants::APRON_WIDTH

@group(0) @binding(0) var u_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(1) var v_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(2) var w_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(3) var u1: texture_storage_3d<rgba16float, read>;
@group(0) @binding(4) var non_solid_fraction: texture_storage_3d<rgba16float, read>;

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
    let gid_col = gid + vec3u(APRON_WIDTH);
    let u_plus = textureLoad(u1, gid_col).xyz;
    let u_minus = vec3f(
        textureLoad(u1, gid_col - X).x,
        textureLoad(u1, gid_col - Y).y,
        textureLoad(u1, gid_col - Z).z,
    );
    let u = 0.5 * (u_plus + u_minus);

    let f = textureLoad(non_solid_fraction, gid).xyz;

    if all(gid < fluid_uniform.resolution + X) {
        if f.x == 0.0 {
            textureStore(u_mac, gid, vec4f(0.0));
        } else {
            textureStore(u_mac, gid, vec4f(u.x, 0.0, 0.0, 0.0));
        }
    }

    if all(gid < fluid_uniform.resolution + Y) {
        if f.y == 0.0 {
            textureStore(v_mac, gid, vec4f(0.0));
        } else {
            textureStore(v_mac, gid, vec4f(u.y, 0.0, 0.0, 0.0));
        }
    }

    if all(gid < fluid_uniform.resolution + Z) {
        if f.z == 0.0 {
            textureStore(w_mac, gid, vec4f(0.0));
        } else {
            textureStore(w_mac, gid, vec4f(u.z, 0.0, 0.0, 0.0));
        }
    }
}