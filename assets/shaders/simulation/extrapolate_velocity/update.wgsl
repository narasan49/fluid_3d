@group(0) @binding(0) var u0: texture_storage_3d<rgba16float, read_write>;
@group(0) @binding(1) var in_velocity_fixed: texture_storage_3d<r8uint, read>;
@group(0) @binding(2) var out_velocity_fixed: texture_storage_3d<r8uint, write>;

@compute @workgroup_size(8, 8, 4)
fn update(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u0);
    if any(gid >= dim) {
        return;
    }

    let fixed = textureLoad(in_velocity_fixed, gid).x;
    if fixed == 1 {
        textureStore(out_velocity_fixed, gid, vec4u(1, 0, 0, 0));
        return;
    } else {
        var count = 0;
        var new_u = vec3f(0.0);
        let idx = vec3i(gid);
        let dimi = vec3i(dim);
        let neighbors = array(
            vec3i(-1, 0, 0),
            vec3i(1, 0, 0),
            vec3i(0, -1, 0),
            vec3i(0, 1, 0),
            vec3i(0, 0, -1),
            vec3i(0, 0, 1),
        );

        for (var i = 0; i < 6; i++) {
            let neighbor = idx + neighbors[i];
            if all(neighbor >= vec3i(0)) && all(neighbor < dimi) {
                let neibor_fixed = textureLoad(in_velocity_fixed, neighbor).x;
                if neibor_fixed == 1 {
                    count += 1;
                    new_u += textureLoad(u0, neighbor).xyz;
                }
            }
        }

        if count > 0 {
            new_u /= f32(count);
            textureStore(u0, gid, vec4f(new_u, 0.0));
            textureStore(out_velocity_fixed, gid, vec4u(1, 0, 0, 0));
        } else {
            textureStore(out_velocity_fixed, gid, vec4u(0, 0, 0, 0));
        }
    }
}