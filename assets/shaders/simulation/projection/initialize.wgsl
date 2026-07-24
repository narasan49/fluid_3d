#import fluid3d::constants::APRON_WIDTH

@group(0) @binding(0) var levelset_air_col: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var levelset_air: texture_storage_3d<r32float, write>;

@compute @workgroup_size(8, 8, 4)
fn initialize(
    @builtin(global_invocation_id) gid: vec3u,
) {
    if any(gid >= textureDimensions(levelset_air)) {
        return;
    }

    let level_with_apron = textureLoad(levelset_air_col, gid + vec3u(APRON_WIDTH)).x;
    textureStore(levelset_air, gid, vec4f(level_with_apron, 0.0, 0.0, 0.0));
}