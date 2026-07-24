#import fluid3d::fluid_uniform::{FluidUniform, BOUNDARY_WALL, BOUNDARY_OPEN}
#import fluid3d::primitive_sdf::{SolidBody, SHAPE_CAPSULE, Capsule, sdf_solid_body}
#import fluid3d::constants::APRON_WIDTH

@group(0) @binding(0) var u_solid: texture_storage_3d<rgba16float, read_write>;
@group(0) @binding(1) var levelset_solid: texture_storage_3d<r32float, write>;
@group(0) @binding(2) var levelset_air0: texture_storage_3d<r32float, write>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@group(2) @binding(0) var<storage, read_write> solid_bodies: array<SolidBody>;
@group(2) @binding(1) var<uniform> array_length: u32;

@compute @workgroup_size(8, 8, 4)
fn update_solid_and_apron(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim_apron = textureDimensions(levelset_solid);
    let dim = dim_apron - vec3u(2u * APRON_WIDTH);
    if any(gid >= dim_apron) {
        return;
    }
    let local_position = 2.0 * ((vec3f(gid) - vec3f(f32(APRON_WIDTH))) / vec3f(fluid_uniform.resolution) - 0.5) * fluid_uniform.half_size;
    let global_position = (fluid_uniform.transform * vec4f(local_position, 1.0)).xyz;
    let location_to_voxel_scale = length(vec3f(gid));

    let u_solid_current = textureLoad(u_solid, gid).xyz;
    if gid.x == 0 {
        if fluid_uniform.boundary_condition_min.x == BOUNDARY_OPEN {
            textureStore(levelset_air0, gid, vec4f(1.0, 0.0, 0.0, 0.0));
        }
    }
    if gid.x == dim_apron.x - 1 {
        if fluid_uniform.boundary_condition_max.x == BOUNDARY_OPEN {
            textureStore(levelset_air0, gid, vec4f(1.0, 0.0, 0.0, 0.0));
        }
    }
    if gid.y == 0 {
        if fluid_uniform.boundary_condition_min.y == BOUNDARY_OPEN {
            textureStore(levelset_air0, gid, vec4f(1.0, 0.0, 0.0, 0.0));
        }
    }
    if gid.y == dim_apron.y - 1 {
        if fluid_uniform.boundary_condition_max.y == BOUNDARY_OPEN {
            textureStore(levelset_air0, gid, vec4f(1.0, 0.0, 0.0, 0.0));
        }
    }
    if gid.z == 0 {
        if fluid_uniform.boundary_condition_min.z == BOUNDARY_OPEN {
            textureStore(levelset_air0, gid, vec4f(1.0, 0.0, 0.0, 0.0));
        }
    }
    if gid.z == dim_apron.z - 1 {
        if fluid_uniform.boundary_condition_max.z == BOUNDARY_OPEN {
            textureStore(levelset_air0, gid, vec4f(1.0, 0.0, 0.0, 0.0));
        }
    }

    var level_solid = 1e6;
    var velocity = vec3f(0.0);
    if fluid_uniform.boundary_condition_min.x == BOUNDARY_WALL {
        level_solid = min(level_solid, f32(gid.x) - 1.0 + 0.5);
    }
    if fluid_uniform.boundary_condition_max.x == BOUNDARY_WALL {
        level_solid = min(level_solid, f32(dim_apron.x - gid.x) - 2.0 + 0.5);
    }
    if fluid_uniform.boundary_condition_min.y == BOUNDARY_WALL {
        level_solid = min(level_solid, f32(gid.y) - 1.0 + 0.5);
    }
    if fluid_uniform.boundary_condition_max.y == BOUNDARY_WALL {
        level_solid = min(level_solid, f32(dim_apron.y - gid.y) - 2.0 + 0.5);
    }
    if fluid_uniform.boundary_condition_min.z == BOUNDARY_WALL {
        level_solid = min(level_solid, f32(gid.z) - 1.0 + 0.5);
    }
    if fluid_uniform.boundary_condition_max.z == BOUNDARY_WALL {
        level_solid = min(level_solid, f32(dim_apron.z - gid.z) - 2.0 + 0.5);
    }
    
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