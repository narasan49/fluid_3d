#import fluid3d::area_fraction::{load_area_fraction, fully_solid}
#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var b: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var levelset_air0: texture_storage_3d<r32float, read>;
@group(0) @binding(2) var fluid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(3) var x: texture_storage_3d<r32float, read>;
@group(0) @binding(4) var r: texture_storage_3d<r32float, write>;
@group(0) @binding(5) var<uniform> dx_scale: f32;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@compute @workgroup_size(8, 8, 4)
fn residual(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(x);
    if any(gid >= dim) {
        return;
    }
    let level_air_ij = textureLoad(levelset_air0, gid).x;
    if level_air_ij >= 0.0 {
        textureStore(r, gid, vec4f(0.0));
        return;
    }
    let idx = vec3i(gid);
    let f = load_area_fraction(fluid_fraction, idx);
    if fully_solid(f) {
        textureStore(r, gid, vec4f(0.0));
        return;
    }

    let neighbor_offsets = array<vec3i, 6>(
        vec3i(-1, 0, 0),
        vec3i(1, 0, 0),
        vec3i(0, -1, 0),
        vec3i(0, 1, 0),
        vec3i(0, 0, -1),
        vec3i(0, 0, 1),
    );
    let dimi = vec3i(dim);
    var residual = textureLoad(b, idx).x;
    let x_center = textureLoad(x, idx).x;
    let dx = dx_scale * fluid_uniform.dx;
    let factor = fluid_uniform.dt / (fluid_uniform.rho * dx * dx);

    for (var i = 0; i < 6; i++) {
        let neighbor = idx + neighbor_offsets[i];
        if all(vec3(0) <= neighbor) && all(neighbor < dimi) {
            let level_neighbor = textureLoad(levelset_air0, neighbor).x;
            if level_neighbor < 0.0 {
                residual -= f[i] * (x_center - textureLoad(x, neighbor).x) * factor;
            } else {
                let theta = clamp(level_air_ij / (level_air_ij - level_neighbor), 0.1, 1.0);
                residual -= f[i] / theta * x_center * factor;
            }
        }
    }
}