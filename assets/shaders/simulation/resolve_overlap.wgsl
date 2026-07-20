#import fluid3d::fluid_uniform::FluidUniform
#import fluid3d::interp::{trilinear_rw, trilinear_rgba16float}

struct OtherFluidUniform {
    inverse_transform: mat4x4f,
    half_size: vec3f,
}

@group(0) @binding(0) var levelset_air_this: texture_storage_3d<r32float, read_write>;
@group(0) @binding(1) var u0_this: texture_storage_3d<rgba16float, read_write>;
// workaround: readアクセスで十分だが、wgpuの問題で型の異なるテクスチャでtextureDimensionsを使用すると、`redefinition of 'NagaRWDimensions3D'`のエラーとなってしまう。
@group(0) @binding(2) var levelset_air_other: texture_storage_3d<r32float, read_write>;
@group(0) @binding(3) var u0_other: texture_storage_3d<rgba16float, read>;
@group(0) @binding(4) var<uniform> fluid_uniform_other: OtherFluidUniform;

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

    let level_other = trilinear_rw(levelset_air_other, idx_other);
    let level_this = textureLoad(levelset_air_this, gid).x;

    if level_other < level_this {
        textureStore(levelset_air_this, gid, vec4f(level_other, 0.0, 0.0, 0.0));
        textureStore(u0_this, gid, vec4f(trilinear_rgba16float(u0_other, idx_other), 0.0));
    }
}
