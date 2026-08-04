#import fluid3d::primitive_sdf::{SolidBody, sdf_solid_body, grad_sdf_solid_body}

@group(0) @binding(0) var grad_sdf: texture_storage_3d<rgba16float, write>;
@group(0) @binding(1) var<storage, read_write> shapes: array<SolidBody>;

@compute @workgroup_size(8, 8, 8)
fn shapes_to_sdf(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(grad_sdf);
    if any(gid >= dim) {
        return;
    }

    let x = vec3f(gid) / vec3f(dim) - 0.5;

    let n = arrayLength(&shapes);
    var new_sdf = 1e6;
    var grad = vec3f(0.0, 1.0, 0.0);
    for (var i = 0u; i < n; i++) {
        let shape = shapes[i];
        let sdf = sdf_solid_body(shape, x);
        if sdf < new_sdf {
            new_sdf = sdf;
            grad = grad_sdf_solid_body(shape, x);
        }
    }

    textureStore(grad_sdf, gid, vec4f(new_sdf, grad));
}