@group(0) @binding(0) var levelset_air0: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var levelset_solid: texture_storage_3d<r32float, read>;
@group(0) @binding(2) var levelset_and_grad_air: texture_storage_3d<rgba16float, write>;

@compute @workgroup_size(8, 8, 4)
fn update_grad_levelset(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(levelset_and_grad_air);
    if any(gid >= dim) {
        return;
    }

    let level_air = textureLoad(levelset_air0, gid).r;
    let level_solid = textureLoad(levelset_solid, gid).r;
    var level_fluid = 0.0;
    if level_solid >= 0.0 {
        level_fluid = level_air;
    } else {
        level_fluid = -level_solid;
    }

    var grad = vec3f(0.0);
    if any(gid == vec3u(0)) || any(gid == (dim - vec3u(1))) {
        textureStore(levelset_and_grad_air, gid, vec4f(0.0));
        return;
    }

    if all(gid > vec3u(0)) && all(gid < (dim - vec3u(1))) {
        grad = vec3f(
            0.5 * (textureLoad(levelset_air0, gid + vec3u(1, 0, 0)).x - textureLoad(levelset_air0, gid - vec3u(1, 0, 0)).x),
            0.5 * (textureLoad(levelset_air0, gid + vec3u(0, 1, 0)).x - textureLoad(levelset_air0, gid - vec3u(0, 1, 0)).x),
            0.5 * (textureLoad(levelset_air0, gid + vec3u(0, 0, 1)).x - textureLoad(levelset_air0, gid - vec3u(0, 0, 1)).x),
        );
    }

    textureStore(levelset_and_grad_air, gid, vec4f(level_fluid, normalize(grad)));
}