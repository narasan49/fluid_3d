@group(0) @binding(0) var levelset_air0: texture_storage_3d<r32float, write>;
// @group(0) @binding(1) var levelset_air1: texture_storage_3d<r32float, write>;

@compute @workgroup_size(8, 8, 4)
fn initialize(
    @builtin(global_invocation_id) gid: vec3u,
) {
    // ToDo: いったん初期値をハードコード
    let surface_level = 10.0;
    textureStore(levelset_air0, gid, vec4f(surface_level - f32(gid.y), vec3f(0));
}