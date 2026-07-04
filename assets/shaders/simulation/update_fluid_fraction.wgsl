@group(0) @binding(0) var levelset_solid: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var fluid_fraction: texture_storage_3d<rgba16float, write>;

@compute @workgroup_size(8, 8, 4)
fn update_fluid_fraction(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(fluid_fraction);
    if any(gid >= dim) {
        return;
    }
    if any(gid == vec3u(0)) || any(gid == (dim - vec3u(1))) {
        textureStore(fluid_fraction, gid, vec4f(0.0));
        return;
    }

    let fraction = area_fraction(levelset_solid, gid - vec3u(1));

    textureStore(fluid_fraction, gid, vec4f(fraction, 0.0));
}

fn area_fraction(
    levelset: texture_storage_3d<r32float, read>,
    idx_min: vec3u,
) -> vec3f {
    let phi_vertices = levelset_vertices(levelset, idx_min);
    // -(マイナス)X 面のarea fraction
    // ボクセルの頂点のインデックス (local_idx, global_idx)
    // 0, (i-1/2, j-1/2, k-1/2)
    // 2, (i-1/2, j+1/2, k-1/2)
    // 4, (i-1/2, j-1/2, k+1/2)
    // 6, (i-1/2, j+1/2, k+1/2)
    let fraction_x = 0.5 * (
        area_fraction_triangle(vec3f(phi_vertices[0], phi_vertices[2], phi_vertices[6]))
        + area_fraction_triangle(vec3f(phi_vertices[0], phi_vertices[4], phi_vertices[6]))
    );

    // -Y 面のarea fraction
    // (local_idx, global_idx)
    // 0, (i-1/2, j-1/2, k-1/2)
    // 1, (i+1/2, j-1/2, k-1/2)
    // 4, (i-1/2, j-1/2, k+1/2)
    // 5, (i+1/2, j-1/2, k+1/2)
    let fraction_y = 0.5 * (
        area_fraction_triangle(vec3f(phi_vertices[0], phi_vertices[1], phi_vertices[5]))
        + area_fraction_triangle(vec3f(phi_vertices[0], phi_vertices[4], phi_vertices[5]))
    );

    // -Z 面のarea fraction
    // (local_idx, global_idx)
    // 0, (i-1/2, j-1/2, k-1/2)
    // 1, (i+1/2, j-1/2, k-1/2)
    // 2, (i-1/2, j+1/2, k-1/2)
    // 3, (i-1/2, j+1/2, k-1/2)
    let fraction_z = 0.5 * (
        area_fraction_triangle(vec3f(phi_vertices[0], phi_vertices[1], phi_vertices[3]))
        + area_fraction_triangle(vec3f(phi_vertices[0], phi_vertices[2], phi_vertices[3]))
    );

    return vec3f(fraction_x, fraction_y, fraction_z);
}

// (i + 1/2, j + 1/2, k + 1/2)以外のボクセルの頂点、7点におけるレベルセットを求める。
fn levelset_vertices(
    levelset: texture_storage_3d<r32float, read>,
    idx_min: vec3u,
) -> array<f32, 7> {
    let phi_centers = array(
        textureLoad(levelset, idx_min).x,
        textureLoad(levelset, idx_min + vec3u(1, 0, 0)).x,
        textureLoad(levelset, idx_min + vec3u(2, 0, 0)).x,
        textureLoad(levelset, idx_min + vec3u(0, 1, 0)).x,
        textureLoad(levelset, idx_min + vec3u(1, 1, 0)).x,
        textureLoad(levelset, idx_min + vec3u(2, 1, 0)).x,
        textureLoad(levelset, idx_min + vec3u(0, 2, 0)).x,
        textureLoad(levelset, idx_min + vec3u(1, 2, 0)).x,
        textureLoad(levelset, idx_min + vec3u(2, 2, 0)).x,
        textureLoad(levelset, idx_min + vec3u(0, 0, 1)).x,
        textureLoad(levelset, idx_min + vec3u(1, 0, 1)).x,
        textureLoad(levelset, idx_min + vec3u(2, 0, 1)).x,
        textureLoad(levelset, idx_min + vec3u(0, 1, 1)).x,
        textureLoad(levelset, idx_min + vec3u(1, 1, 1)).x,
        textureLoad(levelset, idx_min + vec3u(2, 1, 1)).x,
        textureLoad(levelset, idx_min + vec3u(0, 2, 1)).x,
        textureLoad(levelset, idx_min + vec3u(1, 2, 1)).x,
        textureLoad(levelset, idx_min + vec3u(2, 2, 1)).x,
        textureLoad(levelset, idx_min + vec3u(0, 0, 2)).x,
        textureLoad(levelset, idx_min + vec3u(1, 0, 2)).x,
        textureLoad(levelset, idx_min + vec3u(2, 0, 2)).x,
        textureLoad(levelset, idx_min + vec3u(0, 1, 2)).x,
        textureLoad(levelset, idx_min + vec3u(1, 1, 2)).x,
        textureLoad(levelset, idx_min + vec3u(2, 1, 2)).x,
        textureLoad(levelset, idx_min + vec3u(0, 2, 2)).x,
        textureLoad(levelset, idx_min + vec3u(1, 2, 2)).x,
    );

    var phi_vertices = array<f32, 7>(0, 0, 0, 0, 0, 0, 0);
    for (var i = 0u; i < 2u; i++) {
        for (var j = 0u; j < 2u; j++) {
            for (var k = 0u; k < 2u; k++) {
                let offset = i + 3 * j + 9 * k; // 0, 1, 3, 4, 9, 10, 12, 13
                // [i-0.5, j-0.5, k-0.5]
                phi_vertices[0] += phi_centers[offset];
                // [i+0.5, j-0.5, k-0.5]
                phi_vertices[1] += phi_centers[offset + 1];
                // [i-0.5, j+0.5, k-0.5]
                phi_vertices[2] += phi_centers[offset + 3];
                // [i+0.5, j+0.5, k-0.5]
                phi_vertices[3] += phi_centers[offset + 4];
                // [i-0.5, j-0.5, k+0.5]
                phi_vertices[4] += phi_centers[offset + 9];
                // [i+0.5, j-0.5, k+0.5]
                phi_vertices[5] += phi_centers[offset + 10];
                // [i-0.5, j+0.5, k+0.5]
                phi_vertices[6] += phi_centers[offset + 12];
                // [i+0.5, j+0.5, k+0.5] は使わない
            }
        }
    }

    return phi_vertices;
}

fn area_fraction_triangle(
    levels: vec3f,
) -> f32 {
    if all(levels == vec3f(0.0)) {
        return 0.0;
    }
    var phis = levels;
    // phi0 <= phi1 <= phi2
    if phis.x > phis.y {
        let tmp = phis.x;
        phis.x = phis.y;
        phis.y = tmp;
    }
    if phis.x > phis.z {
        let tmp = phis.x;
        phis.x = phis.z;
        phis.z = tmp;
    }
    if phis.y > phis.z {
        let tmp = phis.y;
        phis.y = phis.z;
        phis.z = tmp;
    }

    if 0.0 <= phis.x {
        return 1.0;
    } else if 0.0 <= phis.y {
        let theta20 = phis.x / (phis.x - phis.z);
        let theta10 = phis.x / (phis.x - phis.y);
        return 1.0 - theta10 * theta20;
    } else if 0.0 <= phis.z {
        let theta02 = phis.z / (phis.z - phis.x);
        let theta12 = phis.z / (phis.z - phis.y);
        return theta02 * theta12;
    } else {
        return 0.0;
    }
}