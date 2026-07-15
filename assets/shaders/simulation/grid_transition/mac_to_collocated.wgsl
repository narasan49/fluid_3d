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

    // 傾斜上に溜まる流体は軽減されるが、剛体内部に流体が生成される。
    let f_non_fluid = textureLoad(non_fluid_fraction, gid);
    let f_non_fluid_plus = vec3f(
        textureLoad(non_fluid_fraction, gid + vec3u(1, 0, 0)).x,
        textureLoad(non_fluid_fraction, gid + vec3u(0, 1, 0)).y,
        textureLoad(non_fluid_fraction, gid + vec3u(0, 0, 1)).z,
    );
    var u = vec3f(0.0);
    var counts = vec3i(0);
    if f_non_fluid_plus.x < 1.0 {
        u.x += textureLoad(u_mac, gid + X).x;
        counts.x += 1;
    }
    if f_non_fluid.x < 1.0 {
        u.x += textureLoad(u_mac, gid).x;
        counts.x += 1;
    }
    if counts.x > 0 {
        u.x /= f32(counts.x);
    }

    if f_non_fluid_plus.y < 1.0 {
        u.y += textureLoad(v_mac, gid + Y).x;
        counts.y += 1;
    }
    if f_non_fluid.y < 1.0 {
        u.y += textureLoad(v_mac, gid).x;
        counts.y += 1;
    }
    if counts.y > 0 {
        u.y /= f32(counts.y);
    }

    if f_non_fluid_plus.z < 1.0 {
        u.z += textureLoad(w_mac, gid + Z).x;
        counts.z += 1;
    }
    if f_non_fluid.z < 1.0 {
        u.z += textureLoad(w_mac, gid).x;
        counts.z += 1;
    }
    if counts.z > 0 {
        u.z /= f32(counts.z);
    }

    textureStore(u0, gid, vec4f(u, 0.0));
}