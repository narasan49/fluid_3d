@group(0) @binding(0) var u_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(1) var v_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(2) var w_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(3) var u0: texture_storage_3d<rgba16float, write>;
@group(0) @binding(4) var levelset_air0: texture_storage_3d<r32float, read>;

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

    var u = vec3f(0);
    var counts = vec3u(0);
    let level_center = textureLoad(levelset_air0, gid).x;
    let level_xplus = 0.5 * (level_center + textureLoad(levelset_air0, gid + X).x);
    let level_xminus = 0.5 * (level_center + textureLoad(levelset_air0, gid - X).x);
    if level_xplus < 0.0 {
        u.x += textureLoad(u_mac, gid + X).x;
        counts.x += 1;
    }
    if level_xminus < 0.0 {
        u.x += textureLoad(u_mac, gid).x;
        counts.x += 1;
    }
    if counts.x > 0 {
        u.x /= f32(counts.x);
    }

    let level_yplus = 0.5 * (level_center + textureLoad(levelset_air0, gid + Y).x);
    let level_yminus = 0.5 * (level_center + textureLoad(levelset_air0, gid - Y).x);
    if level_yplus < 0.0 {
        u.y += textureLoad(v_mac, gid + Y).x;
        counts.y += 1;
    }
    if level_yminus < 0.0 {
        u.y += textureLoad(v_mac, gid).x;
        counts.y += 1;
    }
    if counts.y > 0 {
        u.y /= f32(counts.y);
    }

    let level_zplus = 0.5 * (level_center + textureLoad(levelset_air0, gid + Z).x);
    let level_zminus = 0.5 * (level_center + textureLoad(levelset_air0, gid - Z).x);
    if level_zplus < 0.0 {
        u.z += textureLoad(w_mac, gid + Z).x;
        counts.z += 1;
    }
    if level_zminus < 0.0 {
        u.z += textureLoad(w_mac, gid).x;
        counts.z += 1;
    }
    if counts.z > 0 {
        u.z /= f32(counts.z);
    }

    textureStore(u0, gid, vec4f(u, 0.0));
}