@group(0) @binding(0) var levelset_air0: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var grad_levelset_air: texture_storage_3d<rgba16snorm, write>;

@compute @workgroup_size(8, 8, 4)
fn update_grad_levelset(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(grad_levelset_air);
    if any(gid >= dim) {
        return;
    }

    if any(gid == vec3u(0)) || any(gid == (dim - vec3u(1))) {
        textureStore(grad_levelset_air, gid, vec4f(0.0));
    }

    let grad = vec3f(
        0.5 * (textureLoad(levelset_air0, gid + vec3u(1, 0, 0)).x - textureLoad(levelset_air0, gid - vec3u(1, 0, 0)).x),
        0.5 * (textureLoad(levelset_air0, gid + vec3u(0, 1, 0)).x - textureLoad(levelset_air0, gid - vec3u(0, 1, 0)).x),
        0.5 * (textureLoad(levelset_air0, gid + vec3u(0, 0, 1)).x - textureLoad(levelset_air0, gid - vec3u(0, 0, 1)).x),
    );

    textureStore(grad_levelset_air, gid, vec4f(normalize(grad), 0.0));
}