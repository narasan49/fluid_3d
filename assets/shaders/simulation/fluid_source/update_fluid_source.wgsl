const MODE_SOURCE = 0u;
const MODE_SINK = 1u;

const SHAPE_SPHERE = 0u;
const SHAPE_AABB = 1u;

const LARGE_FLOAT = 1e6;

struct FluidSource {
    mode: u32,
    shape: u32,
    // Fluidとの相対座標
    location: vec3f,
    velocity: vec3f,
    shape_values: array<f32, 3>,
}

struct FluidSources {
    data: array<FluidSource, 8>,
    count: u32,
}

@group(0) @binding(0) var levelset_air0: texture_storage_3d<r32float, read_write>;
@group(0) @binding(1) var u0: texture_storage_3d<rgba16float, write>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@group(2) @binding(0) var<uniform> sources: FluidSources

@compute @workgroup_size(8, 8, 4)
fn update_fluid_source(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(levelset_air0);
    let local_position = 2.0 * (vec3f(gid) / vec3f(dim) - 0.5) * fluid_uniform.half_size;

    var new_level = textureLoad(levelset_air0, gid).x;
    var new_velocity = vec3f(0.0);
    var need_velocity_update = false;
    var has_source = false;
    for (var i = 0; i < sources.count; i++) {
        let source = sources.data[i];
        let source_level = source_sdf(source, local_position);
        switch source.mode {
            case MODE_SOURCE:
            {
                has_source = true;
                new_level = min(new_level, source_level);
                if source_level < 0.0 {
                    need_velocity_update = true;
                    new_velocity = new_velocity + source.velocity;
                }
            }
            case MODE_SINK:
            {
                if !has_source {
                    new_level = min(new_level, -source_level);
                }
            }
            default:
            {}
        }
    }

    textureStore(levelset_air0, gid, vec4f(new_level, 0.0, 0.0, 0.0));
    if need_velocity_update {
        textureStore(u0, gid, vec4f(new_velocity, 0.0));
    }
}

fn source_sdf(source: FluidSource, world_pos: vec3f) -> f32 {
    switch source.shape {
        case SHAPE_SPHERE:
        {
            let radius = source.shape_values[0];
            return distance(source.location, world_pos) - radius;
        }
        case SHAPE_AABB:
        {
            let half_size = vec3f(source.shape_values[0], source.shape_values[1], source.shape_values[2]);
            return sdf_aabb(half_size, world_pos);
        }
        default:
        {
            return LARGE_FLOAT;
        }
    }
}

fn sdf_aabb(half_size: vec3f, x: vec3f) -> f32 {
    let d = abs(x) - half_size;
    let is_inside = d < vec3f(0.0);

    if all(is_inside) {
        return max(d.x, max(d.y, d.z));
    } else {
        return length(select(d, vec3f(0), is_inside));
    }
}