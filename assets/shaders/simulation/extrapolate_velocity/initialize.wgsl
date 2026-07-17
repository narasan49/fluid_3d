@group(0) @binding(0) var non_fluid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(1) var u_fixed: texture_storage_3d<r8uint, write>;
@group(0) @binding(2) var v_fixed: texture_storage_3d<r8uint, write>;
@group(0) @binding(3) var w_fixed: texture_storage_3d<r8uint, write>;

@compute @workgroup_size(8, 8, 4)
fn initialize(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(non_fluid_fraction);
    if any(gid >= dim) {
        return;
    }

    let f = textureLoad(non_fluid_fraction, gid);
    if all(gid < (dim - vec3u(0, 1, 1))) {
        var fixed = 1u;
        if f.x == 1.0 {
            fixed = 0u;
        }
        textureStore(u_fixed, gid, vec4u(fixed, 0, 0, 0));
    }
    if all(gid < (dim - vec3u(1, 0, 1))) {
        var fixed = 1u;
        if f.y == 1.0 {
            fixed = 0u;
        }
        textureStore(v_fixed, gid, vec4u(fixed, 0, 0, 0));
    }
    if all(gid < (dim - vec3u(1, 1, 0))) {
        var fixed = 1u;
        if f.z == 1.0 {
            fixed = 0u;
        }
        textureStore(w_fixed, gid, vec4u(fixed, 0, 0, 0));
    }
}