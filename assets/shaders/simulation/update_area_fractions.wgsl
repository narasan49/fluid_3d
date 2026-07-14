@group(0) @binding(0) var levelset_solid: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var levelset_air0: texture_storage_3d<r32float, read>;
@group(0) @binding(2) var non_solid_fraction: texture_storage_3d<rgba16float, write>;
@group(0) @binding(3) var non_fluid_fraction: texture_storage_3d<rgba16float, write>;

@compute @workgroup_size(8, 8, 4)
fn update_area_fractions(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(non_solid_fraction);
    if any(gid >= dim) {
        return;
    }

    let f_non_solid = area_fraction(levelset_solid, vec3i(gid) - vec3i(1), vec3i(dim));
    textureStore(non_solid_fraction, gid, vec4f(f_non_solid, 0.0));

    let f_non_fluid = area_fraction(levelset_air0, vec3i(gid) - vec3i(1), vec3i(dim));
    textureStore(non_fluid_fraction, gid, vec4f(f_non_fluid, 0.0));
}

fn area_fraction(
    levelset: texture_storage_3d<r32float, read>,
    idx_min: vec3i,
    dim: vec3i,
) -> vec3f {
    let phi_vertices = levelset_vertices(levelset, idx_min, dim);
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
    idx_min: vec3i,
    dim: vec3i,
) -> array<f32, 7> {
    let phi_centers = array(
        load_levelset(levelset, idx_min, dim),
        load_levelset(levelset, idx_min + vec3i(1, 0, 0), dim),
        load_levelset(levelset, idx_min + vec3i(2, 0, 0), dim),
        load_levelset(levelset, idx_min + vec3i(0, 1, 0), dim),
        load_levelset(levelset, idx_min + vec3i(1, 1, 0), dim),
        load_levelset(levelset, idx_min + vec3i(2, 1, 0), dim),
        load_levelset(levelset, idx_min + vec3i(0, 2, 0), dim),
        load_levelset(levelset, idx_min + vec3i(1, 2, 0), dim),
        load_levelset(levelset, idx_min + vec3i(2, 2, 0), dim),
        load_levelset(levelset, idx_min + vec3i(0, 0, 1), dim),
        load_levelset(levelset, idx_min + vec3i(1, 0, 1), dim),
        load_levelset(levelset, idx_min + vec3i(2, 0, 1), dim),
        load_levelset(levelset, idx_min + vec3i(0, 1, 1), dim),
        load_levelset(levelset, idx_min + vec3i(1, 1, 1), dim),
        load_levelset(levelset, idx_min + vec3i(2, 1, 1), dim),
        load_levelset(levelset, idx_min + vec3i(0, 2, 1), dim),
        load_levelset(levelset, idx_min + vec3i(1, 2, 1), dim),
        load_levelset(levelset, idx_min + vec3i(2, 2, 1), dim),
        load_levelset(levelset, idx_min + vec3i(0, 0, 2), dim),
        load_levelset(levelset, idx_min + vec3i(1, 0, 2), dim),
        load_levelset(levelset, idx_min + vec3i(2, 0, 2), dim),
        load_levelset(levelset, idx_min + vec3i(0, 1, 2), dim),
        load_levelset(levelset, idx_min + vec3i(1, 1, 2), dim),
        load_levelset(levelset, idx_min + vec3i(2, 1, 2), dim),
        load_levelset(levelset, idx_min + vec3i(0, 2, 2), dim),
        load_levelset(levelset, idx_min + vec3i(1, 2, 2), dim),
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

fn load_levelset(
    levelset: texture_storage_3d<r32float, read>,
    idx: vec3i,
    dim: vec3i,
) -> f32 {
    // 計算領域端のfractionを決め打ちすると、流体がすり抜けてしまうため、範囲外を剛体とする。
    // パフォーマンスへのインパクトは128x32x64解像度で0.04 ms程度の増加
    if any(idx < vec3i(0)) || any(idx >= dim) {
        return -1.0;
    }
    return textureLoad(levelset, idx).x;
}