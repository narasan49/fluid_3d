@group(0) @binding(0) var u_solid: texture_storage_3d<rgba16float, read_write>;
@group(0) @binding(1) var levelset_solid: texture_storage_3d<r32float, write>;

@compute @workgroup_size(8, 8, 4)
fn update_solid(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u_solid);
    if any(gid >= dim) {
        return;
    }

    // ToDo: いったん外周を壁で覆う
    let u_solid_current = textureLoad(u_solid, gid).xyz;
    if gid.x == 0 || gid.x == dim.x - 1{
        textureStore(u_solid, gid, vec4f(0.0, u_solid_current.y, u_solid_current.z, 0.0));
    }
    if gid.y == 0 || gid.y == dim.y - 1{
        textureStore(u_solid, gid, vec4f(u_solid_current.x, 0.0, u_solid_current.z, 0.0));
    }
    if gid.z == 0 || gid.z == dim.z - 1{
        textureStore(u_solid, gid, vec4f(u_solid_current.x, u_solid_current.y, 0.0, 0.0));
    }

    var level_solid = 1e6;
    level_solid = min(level_solid, f32(gid.x) - 0.5);
    level_solid = min(level_solid, f32(dim.x - gid.x) - 1.0 - 0.5);
    level_solid = min(level_solid, f32(gid.y) - 0.5);
    level_solid = min(level_solid, f32(dim.y - gid.y) - 1.0 - 0.5);
    level_solid = min(level_solid, f32(gid.z) - 0.5);
    level_solid = min(level_solid, f32(dim.z - gid.z) - 1.0 - 0.5);

    textureStore(levelset_solid, gid, vec4f(level_solid, 0.0, 0.0, 0.0));
}