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
    if any(gid >= dim) {
        return;
    }
    let u_plus = textureLoad(u1, gid).xyz;
    let u_minus = vec3f(
        textureLoad(u1, gid - X).x,
        textureLoad(u1, gid - Y).y,
        textureLoad(u1, gid - Z).z,
    );
    let u = 0.5 * (u_plus + u_minus);

    if all(gid <= dim) && gid.x != 0 {
        textureStore(u_mac, gid, vec4f(u.x, 0.0, 0.0, 0.0));
    }
    if all(gid <= dim) && gid.y != 0 {
        // x: 0..n, y: 1..n, z: 0..n
        textureStore(v_mac, gid, vec4f(u.y, 0.0, 0.0, 0.0));
    }
    if all(gid <= dim) && gid.z != 0 {
        textureStore(w_mac, gid, vec4f(u.z, 0.0, 0.0, 0.0));
    }
}