@group(0) @binding(0) var levelset_air1: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var levelset_air0: texture_storage_3d<r32float, write>;
@group(0) @binding(2) var labels0: texture_storage_3d<r8uint, write>;

const LARGE_FLOAT: f32 = 1e6;
const LABEL_NONE: u32 = 0u;
const LABEL_SOURCE: u32 = 1u;
const LABEL_ACTIVE: u32 = 2u;

@compute @workgroup_size(8, 8, 4)
fn initialize(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let idx = vec3i(gid);
    let dim = vec3i(textureDimensions(levelset_air1));
    if any(idx >= dim) {
        return;
    }

    let neighbor_offsets = array(
        vec3i(-1, 0, 0),
        vec3i(1, 0, 0),
        vec3i(0, -1, 0),
        vec3i(0, 1, 0),
        vec3i(0, 0, -1),
        vec3i(0, 0, 1),
    );

    let level = textureLoad(levelset_air1, gid).x;
    // 隣接セルのレベルセットとの正負が異なる点(境界をまたぐ点)を抽出。
    for (var i = 0; i < 6; i++) {
        let neighbor = idx + neighbor_offsets[i];
        if all(neighbor >= vec3i(0)) && all(neighbor < dim) {
            let level_neighbor = textureLoad(levelset_air1, neighbor).x;
            if (level * level_neighbor) <= 0.0 {
                textureStore(levelset_air0, idx, vec4f(level, 0.0, 0.0, 0.0));
                textureStore(labels0, idx, vec4u(LABEL_SOURCE, 0, 0, 0));
                return;
            }
        }
    }

    textureStore(levelset_air0, idx, vec4f(sign(level) * LARGE_FLOAT, 0.0, 0.0, 0.0));
    textureStore(labels0, idx, vec4u(LABEL_NONE, 0, 0, 0));
}