#import fluid3d::area_fraction::{load_area_fraction, fully_solid}
#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var div: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var levelset_air0: texture_storage_3d<r32float, read>;
@group(0) @binding(2) var fluid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(3) var p: texture_storage_3d<r32float, read_write>;
@group(0) @binding(4) var<uniform> dx_scale: f32;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@compute @workgroup_size(8, 8, 4)
fn gauss_seidel_red(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(p);
    if any(gid >= dim) {
        return;
    }

    if (gid.x + gid.y + gid.z) % 2 == 1 {
        let p_new = update_pressure(vec3i(gid));
        textureStore(p, gid, vec4f(p_new, 0.0, 0.0, 0.0));
    }
}

@compute @workgroup_size(8, 8, 4)
fn gauss_seidel_black(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = (textureDimensions(p));
    if any(gid >= dim) {
        return;
    }

    if (gid.x + gid.y + gid.z) % 2 == 0 {
        let p_new = update_pressure(vec3i(gid));
        textureStore(p, gid, vec4f(p_new, 0.0, 0.0, 0.0));
    }
}

fn update_pressure(
    idx: vec3i,
) -> f32 {
    let level_air_ij = textureLoad(levelset_air0, idx).x;
    if level_air_ij >= 0.0 {
        return 0.0;
    }

    let f = load_area_fraction(fluid_fraction, idx);
    if fully_solid(f) {
        return 0.0;
    }

    var denom = 0.0;
    let dx = dx_scale * fluid_uniform.dx;
    var nume = dx * dx * fluid_uniform.rho / fluid_uniform.dt * textureLoad(div, idx).x;

    let neighbor_offsets = array<vec3i, 6>(
        vec3i(-1, 0, 0),
        vec3i(1, 0, 0),
        vec3i(0, -1, 0),
        vec3i(0, 1, 0),
        vec3i(0, 0, -1),
        vec3i(0, 0, 1),
    );
    let dim = vec3i(textureDimensions(p));
    for (var i = 0; i < 6; i++) {
        let neighbor = idx + neighbor_offsets[i];
        if all(vec3i(0) <= neighbor) && all(neighbor < dim) {
            let level_neighbor = textureLoad(levelset_air0, neighbor).x;
            if level_neighbor < 0.0 {
                denom += f[i];
                nume += f[i] * textureLoad(p, neighbor).x;
            } else {
                let theta = clamp(level_air_ij / (level_air_ij - level_neighbor), 0.1, 1.0);
                denom += f[i] / theta;
            }
        }
    }

    if abs(denom) < 1e-6 {
        return 0.0;
    }

    let p_new = nume / denom;
    return p_new;
}
