#import fluid3d::fluid_uniform::FluidUniform
#import fluid3d::primitive_sdf::{SolidBody, SHAPE_CAPSULE, Capsule, sdf_solid_body}

@group(0) @binding(0) var u_solid: texture_storage_3d<rgba16float, read_write>;
@group(0) @binding(1) var levelset_solid: texture_storage_3d<r32float, write>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@group(2) @binding(0) var<storage, read_write> solid_bodies: array<SolidBody>;
@group(2) @binding(1) var<uniform> array_length: u32;

@compute @workgroup_size(8, 8, 4)
fn update_solid(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u_solid);
    if any(gid >= dim) {
        return;
    }
    let local_position = 2.0 * (vec3f(gid) / vec3f(dim) - 0.5) * fluid_uniform.half_size;
    let global_position = (fluid_uniform.transform * vec4f(local_position, 1.0)).xyz;
    let location_to_voxel_scale = length(vec3f(gid));

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
    var velocity = vec3f(0.0);
    level_solid = min(level_solid, f32(gid.x));
    level_solid = min(level_solid, f32(dim.x - gid.x) - 1.0);
    level_solid = min(level_solid, f32(gid.y));
    level_solid = min(level_solid, f32(dim.y - gid.y) - 1.0);
    level_solid = min(level_solid, f32(gid.z));
    level_solid = min(level_solid, f32(dim.z - gid.z) - 1.0);
    
    for (var i = 0u; i < array_length; i++) {
        // meter -> pixel
        let sdf0 = sdf_solid_body(solid_bodies[i], global_position) / fluid_uniform.dx;
        if sdf0 < level_solid {
            level_solid = sdf0;
        }
        if sdf0 < 0.5 {
            // m/s -> pixel/s
            velocity = solid_bodies[i].linear_velocity / fluid_uniform.dx;
        }
    }

    textureStore(levelset_solid, gid, vec4f(level_solid, 0.0, 0.0, 0.0));
    textureStore(u_solid, gid, vec4f(velocity, 0.0));
}