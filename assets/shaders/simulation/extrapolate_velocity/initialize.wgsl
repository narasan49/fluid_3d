@group(0) @binding(0) var levelset_air0: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var velocity_fixed: texture_storage_3d<r8uint, write>;

@compute @workgroup_size(8, 8, 4)
fn initialize(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(levelset_air0);
    if any(gid >= dim) {
        return;
    }

    let level = textureLoad(levelset_air0, gid).x;
    var fixed = 1u;
    if level >= 0.0 {
        fixed = 0u;
    }

    textureStore(velocity_fixed, gid, vec4u(fixed, 0, 0, 0));
}