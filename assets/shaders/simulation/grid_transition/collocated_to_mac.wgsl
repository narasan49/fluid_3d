@group(0) @binding(0) var u_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(1) var v_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(2) var w_mac: texture_storage_3d<r16float, write>;
@group(0) @binding(3) var u1: texture_storage_3d<rgba16float, read>;

const X = vec3u(1, 0, 0);
const Y = vec3u(0, 1, 0);
const Z = vec3u(0, 0, 1);

@compute @workgroup_size(8, 8, 4)
fn collocated_to_mac(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u1);
    let idx = gid + vec3u(1);
    if any(gid >= (dim - vec3u(1))) {
        return;
    }
    let u_plus = textureLoad(u1, idx).xyz;
    let u_minus = vec3f(
        textureLoad(u1, idx - X).x,
        textureLoad(u1, idx - Y).y,
        textureLoad(u1, idx - Z).z,
    );
    let u = 0.5 * (u_plus + u_minus);

    if all(idx <= dim + X) {
        textureStore(u_mac, idx, vec4f(u.x, 0.0, 0.0, 0.0));
    }
    if all(idx <= dim + Y) {
        textureStore(v_mac, idx, vec4f(u.y, 0.0, 0.0, 0.0));
    }
    if all(idx <= dim + Z) {
        textureStore(w_mac, idx, vec4f(u.z, 0.0, 0.0, 0.0));
    }
}