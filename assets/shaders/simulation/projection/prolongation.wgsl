@group(0) @binding(0) var x: texture_storage_3d<r32float, read_write>;
@group(0) @binding(1) var x_low: texture_storage_3d<r32float, read>;
@group(0) @binding(2) var levelset_air: texture_storage_3d<r32float, read>;

@compute @workgroup_size(8, 8, 4)
fn prolongation(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(x_low);
    if any(gid >= dim) {
        return;
    }

    let correction = textureLoad(x_low, gid).x;
    let offsets = array<vec3u, 8>(
        vec3u(0, 0, 0),
        vec3u(1, 0, 0),
        vec3u(0, 1, 0),
        vec3u(1, 1, 0),
        vec3u(0, 0, 1),
        vec3u(1, 0, 1),
        vec3u(0, 1, 1),
        vec3u(1, 1, 1),
    );

    for (var i = 0; i < 8; i++) {
        let fine_idx = 2 * gid + offsets[i];
        let level = textureLoad(levelset_air, fine_idx).x;
        if level < 0.0 {
            let corrected_x = correction + textureLoad(x, fine_idx).x;
            textureStore(x, fine_idx, vec4f(corrected_x, 0.0, 0.0, 0.0));
        }
    }
}