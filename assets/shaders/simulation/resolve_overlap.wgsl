#import fluid3d::fluid_uniform::FluidUniform

struct OtherFluidUniform {
    inverse_transform: mat4x4f,
    half_size: vec3f,
}

@group(0) @binding(0) var levelset_air_this: texture_storage_3d<r32float, read_write>;
// workaround: readアクセスで十分だが、wgpuの問題で型の異なるテクスチャでtextureDimensionsを使用すると、`redefinition of 'NagaRWDimensions3D'`のエラーとなってしまう。
@group(0) @binding(1) var levelset_air_other: texture_storage_3d<r32float, read_write>;
@group(0) @binding(2) var<uniform> fluid_uniform_other: OtherFluidUniform;

@group(1) @binding(0) var<uniform> fluid_uniform_this: FluidUniform;


@compute @workgroup_size(8, 8, 4)
fn resolve_overlap(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(levelset_air_this);
    let local_position = 2.0 * (vec3f(gid) / vec3f(dim) - 0.5) * fluid_uniform_this.half_size;
    let global_position: vec4f = fluid_uniform_this.transform * vec4f(local_position, 1.0);

    let local_position_other = fluid_uniform_other.inverse_transform * global_position;

    let uv_other = 0.5 * (local_position_other.xyz / fluid_uniform_other.half_size + 1.0);
    if any(uv_other < vec3f(0.0)) || any(vec3f(1.0) <= uv_other) {
        return;
    }
    let dim_other = textureDimensions(levelset_air_other);
    let dimf_other = vec3f(dim_other);
    let idx_other = uv_other * dimf_other;

    let level_other = trilinear(levelset_air_other, idx_other);
    let level_this = textureLoad(levelset_air_this, gid).x;

    let new_level = min(level_other, level_this);
    textureStore(levelset_air_this, gid, vec4f(new_level, 0.0, 0.0, 0.0));
}

fn trilinear(
    levelset: texture_storage_3d<r32float, read_write>,
    x: vec3f,
) -> f32 {
    let base = floor(x);
    let fract = x - base;
    let idx = vec3u(base);

    let y = array<f32, 8>(
        textureLoad(levelset, idx + vec3u(0, 0, 0)).x,
        textureLoad(levelset, idx + vec3u(1, 0, 0)).x,
        textureLoad(levelset, idx + vec3u(0, 1, 0)).x,
        textureLoad(levelset, idx + vec3u(1, 1, 0)).x,
        textureLoad(levelset, idx + vec3u(0, 0, 1)).x,
        textureLoad(levelset, idx + vec3u(1, 0, 1)).x,
        textureLoad(levelset, idx + vec3u(0, 1, 1)).x,
        textureLoad(levelset, idx + vec3u(1, 1, 1)).x,
    );

    return mix(
        mix(mix(y[0], y[1], fract.x), mix(y[2], y[3], fract.x), fract.y),
        mix(mix(y[4], y[5], fract.x), mix(y[6], y[7], fract.x), fract.y),
        fract.z
    );
}