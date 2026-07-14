@group(0) @binding(0) var r: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var levelset_air: texture_storage_3d<r32float, read>;
@group(0) @binding(2) var non_solid_fraction: texture_storage_3d<rgba16float, read>;

// restriction先の低解像度テクスチャ
@group(0) @binding(3) var b_low: texture_storage_3d<r32float, write>;
@group(0) @binding(4) var levelset_air_low: texture_storage_3d<r32float, write>;
@group(0) @binding(5) var non_solid_fraction_low: texture_storage_3d<rgba16float, write>;
@group(0) @binding(6) var x_low: texture_storage_3d<r32float, write>;

@compute @workgroup_size(8, 8, 4)
fn restriction(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(b_low);

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

    if all(gid < dim) {
        let r_average = 0.125 * (
            textureLoad(r, 2 * gid + offsets[0]).x
            + textureLoad(r, 2 * gid + offsets[1]).x
            + textureLoad(r, 2 * gid + offsets[2]).x
            + textureLoad(r, 2 * gid + offsets[3]).x
            + textureLoad(r, 2 * gid + offsets[4]).x
            + textureLoad(r, 2 * gid + offsets[5]).x
            + textureLoad(r, 2 * gid + offsets[6]).x
            + textureLoad(r, 2 * gid + offsets[7]).x
        );

        let phis = array<f32, 8>(
            textureLoad(levelset_air, 2 * gid + offsets[0]).x,
            textureLoad(levelset_air, 2 * gid + offsets[1]).x,
            textureLoad(levelset_air, 2 * gid + offsets[2]).x,
            textureLoad(levelset_air, 2 * gid + offsets[3]).x,
            textureLoad(levelset_air, 2 * gid + offsets[4]).x,
            textureLoad(levelset_air, 2 * gid + offsets[5]).x,
            textureLoad(levelset_air, 2 * gid + offsets[6]).x,
            textureLoad(levelset_air, 2 * gid + offsets[7]).x,
        );

        var phi_abs_min = phis[0];
        for (var i = 1; i < 8; i++) {
            if abs(phis[i]) < abs(phi_abs_min) {
                phi_abs_min = phis[i];
            }
        }

        textureStore(b_low, gid, vec4f(r_average, 0.0, 0.0, 0.0));
        textureStore(levelset_air_low, gid, vec4f(phi_abs_min, 0.0, 0.0, 0.0));
        textureStore(x_low, gid, vec4f(0.0));
    }

    let dim_fraction = textureDimensions(non_solid_fraction_low);
    if all(gid < dim_fraction) {
        let fractions = array<vec3f, 7>(
            textureLoad(non_solid_fraction, 2 * gid + offsets[0]).xyz,
            textureLoad(non_solid_fraction, 2 * gid + offsets[1]).xyz,
            textureLoad(non_solid_fraction, 2 * gid + offsets[2]).xyz,
            textureLoad(non_solid_fraction, 2 * gid + offsets[3]).xyz,
            textureLoad(non_solid_fraction, 2 * gid + offsets[4]).xyz,
            textureLoad(non_solid_fraction, 2 * gid + offsets[5]).xyz,
            textureLoad(non_solid_fraction, 2 * gid + offsets[6]).xyz,
        );

        let f_low = vec3f(
            0.25 * (fractions[0].x + fractions[2].x + fractions[4].x + fractions[6].x),
            0.25 * (fractions[0].y + fractions[1].y + fractions[4].y + fractions[5].y),
            0.25 * (fractions[0].z + fractions[1].z + fractions[2].z + fractions[3].z),
        );

        textureStore(non_solid_fraction_low, gid, vec4f(f_low, 0.0));
    }
}