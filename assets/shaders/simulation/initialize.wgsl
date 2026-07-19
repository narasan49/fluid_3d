@group(0) @binding(0) var levelset_air0: texture_storage_3d<r32float, write>;
@group(0) @binding(1) var levelset_air1: texture_storage_3d<r32float, write>;
@group(0) @binding(2) var levelset_and_grad_air: texture_storage_3d<rgba16float, write>;
@group(0) @binding(3) var u0: texture_storage_3d<rgba16float, write>;

@compute @workgroup_size(8, 8, 4)
fn initialize(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(levelset_air0);
    if all(gid < dim) {
        // ToDo: いったん初期値をハードコード
        let surface_level = 0.0;
        let level = f32(gid.y) - surface_level;
        
        textureStore(levelset_air0, gid, vec4f(level, vec3f(0)));
        textureStore(levelset_air1, gid, vec4f(level, vec3f(0)));
        textureStore(levelset_and_grad_air, gid, vec4f(level, 0.0, 1.0, 0.0));
    }

    if all(gid < textureDimensions(u0)) {
        textureStore(u0, gid, vec4f(0.0));
    }
}