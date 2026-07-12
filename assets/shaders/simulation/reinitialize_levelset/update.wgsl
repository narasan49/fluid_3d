@group(0) @binding(0) var labels1: texture_storage_3d<r8uint, read_write>;
@group(0) @binding(1) var levelset_air0: texture_storage_3d<r32float, read_write>;

const LARGE_FLOAT: f32 = 1e6;
const EPSIRON: f32 = 1e-6;

const LABEL_NONE: u32 = 0u;
const LABEL_SOURCE: u32 = 1u;
const LABEL_ACTIVE: u32 = 2u;

@compute @workgroup_size(8, 8, 4)
fn update(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let idx = vec3i(gid);
    let dim = vec3i(textureDimensions(levelset_air0));
    if any(idx >= dim) {
        return;
    }
    let label = textureLoad(labels1, idx).x;
    if label != LABEL_ACTIVE {
        return;
    }

    var p = textureLoad(levelset_air0, idx).x;
    var q = solve_quadratic_3d(levelset_air0, idx) * sign(p);
    textureStore(levelset_air0, idx, vec4f(q, 0.0, 0.0, 0.0));
    if abs(p - q) > EPSIRON {
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

    textureStore(labels1, idx, vec4u(LABEL_NONE, 0, 0, 0));
    for (var i = 0; i < 6; i++) {
        let neighbor = idx + neighbor_offsets[i];
        if all(neighbor >= vec3i(0)) && all(neighbor < dim) {
            let label_nb = textureLoad(labels1, neighbor).x;
            if label_nb != LABEL_ACTIVE && label_nb != LABEL_SOURCE {
                let p_nb = abs(textureLoad(levelset_air0, neighbor).x);
                let q_nb = solve_quadratic_3d(levelset_air0, neighbor);
                if p_nb > q_nb {
                    textureStore(levelset_air0, neighbor, vec4f(q_nb * sign(p), 0.0, 0.0, 0.0));
                    textureStore(labels1, neighbor, vec4u(LABEL_ACTIVE, 0, 0, 0));
                }
            }
        }
    }
}

fn solve_quadratic_3d(
    levelset: texture_storage_3d<r32float, read_write>,
    idx: vec3i,
) -> f32 {
    let phi_xmin = min(abs_load_levelset(levelset, idx + vec3i(-1, 0, 0)), abs_load_levelset(levelset, idx + vec3i(1, 0, 0)));
    let phi_ymin = min(abs_load_levelset(levelset, idx + vec3i(0, -1, 0)), abs_load_levelset(levelset, idx + vec3i(0, 1, 0)));
    let phi_zmin = min(abs_load_levelset(levelset, idx + vec3i(0, 0, -1)), abs_load_levelset(levelset, idx + vec3i(0, 0, 1)));

    var phi_sorted = sort_vec3f(vec3f(phi_xmin, phi_ymin, phi_zmin));

    let d0 = phi_sorted.z - phi_sorted.x;
    let d1 = phi_sorted.y - phi_sorted.x;
    if d0 < 1.0 {
        let phi_sum = phi_sorted.x + phi_sorted.y + phi_sorted.z;
        let phi_sq_sum = dot(phi_sorted, phi_sorted);
        return (phi_sum + sqrt(phi_sum * phi_sum - 3.0 * (phi_sq_sum - 1.0))) / 3.0;
    } else if d1 < 1.0 {
        return 0.5 * (phi_sorted.x + phi_sorted.y + sqrt(2.0 - d1 * d1));
    } else {
        return phi_sorted.x + 1.0;
    }
}

fn sort_vec3f(data: vec3f) -> vec3f {
    var sorted = data;
    if sorted.x > sorted.y {
        let tmp = sorted.x;
        sorted.x = sorted.y;
        sorted.y = tmp;
    }
    if sorted.x > sorted.z {
        let tmp = sorted.x;
        sorted.x = sorted.z;
        sorted.z = tmp;
    }
    if sorted.y > sorted.z {
        let tmp = sorted.y;
        sorted.y = sorted.z;
        sorted.z = tmp;
    }

    return sorted;
}

fn abs_load_levelset(
    levelset: texture_storage_3d<r32float, read_write>,
    idx: vec3i,
) -> f32 {
    let dim = vec3i(textureDimensions(levelset));
    if any(idx < vec3i(0)) || any(idx >= dim) {
        return LARGE_FLOAT;
    }

    return abs(textureLoad(levelset, idx).x);
}