@group(0) @binding(0) var u_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(1) var v_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(2) var w_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(3) var u0: texture_storage_3d<rgba16float, write>;
@group(0) @binding(4) var non_fluid_fraction: texture_storage_3d<rgba16float, read>;

const X = vec3u(1, 0, 0);
const Y = vec3u(0, 1, 0);
const Z = vec3u(0, 0, 1);

@compute @workgroup_size(8, 8, 4)
fn mac_to_collocated(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u0);
    if any(gid >= dim) {
        return;
    }

    let u = 0.5 * vec3f(
        textureLoad(u_mac, gid).x + textureLoad(u_mac, gid + X).x,
        textureLoad(v_mac, gid).x + textureLoad(v_mac, gid + Y).x,
        textureLoad(w_mac, gid).x + textureLoad(w_mac, gid + Y).x,
    );
    textureStore(u0, gid, vec4f(u, 0.0));
}