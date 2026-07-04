@group(0) @binding(0) var labels0: texture_storage_3d<r8uint, read>;
@group(0) @binding(1) var labels1: texture_storage_3d<r8uint, write>;

const LABEL_NONE: u32 = 0u;
const LABEL_SOURCE: u32 = 1u;
const LABEL_ACTIVE: u32 = 2u;

@compute @workgroup_size(8, 8, 4)
fn initialize_active_labels(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let idx = vec3i(gid);
    let dim = vec3i(textureDimensions(labels0));
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

    // Sourceの隣接セルをActiveにマークする
    let label = textureLoad(labels0, idx).x;
    if label == LABEL_SOURCE {
        textureStore(labels1, idx, vec4u(LABEL_SOURCE, 0, 0, 0));
        return;
    }

    for (var i = 0; i < 6; i++) {
        let neighbor = idx + neighbor_offsets[i];
        if all(neighbor >= vec3i(0)) && all(neighbor < dim) {
            let label_nb = textureLoad(labels0, neighbor).x;
            if label_nb == LABEL_SOURCE {
                textureStore(labels1, idx, vec4u(LABEL_ACTIVE, 0, 0, 0));
                return;
            }
        }
    }

    textureStore(labels1, idx, vec4u(LABEL_NONE, 0, 0, 0));
}