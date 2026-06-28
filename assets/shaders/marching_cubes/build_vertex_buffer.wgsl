// Cubeの辺。Cubeの頂点のインデックス(0-7)で表す
struct Edge {
    a: u32,
    b: u32,
}

struct EdgeTriangle {
    edges: array<Edge, 3>,
}

// MarchingCubesの面を構成する最大5個の三角形
struct EdgeTriangles {
    triangles: array<EdgeTriangle, 5>,
    count: u32,
}

struct Vertex {
    position: vec4f,
    normal: vec4f,
}

struct DrawIndirectArgs {
    vertex_count: atomic<u32>,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32
}

@group(0) @binding(0) var<storage, read_write> vertices: array<Vertex>;
@group(0) @binding(1) var<storage, read_write> indirect_args: DrawIndirectArgs;
@group(0) @binding(2) var sdf: texture_storage_3d<r32float, read>;
@group(0) @binding(3) var grad_sdf: texture_storage_3d<rgba16snorm, read>;
@group(0) @binding(4) var<storage, read> lookup_table: array<EdgeTriangles, 256>;

const offsets_unit: array<vec3u, 8> = array<vec3u, 8>(
    vec3u(0, 0, 0),
    vec3u(1, 0, 0),
    vec3u(0, 1, 0),
    vec3u(1, 1, 0),
    vec3u(0, 0, 1),
    vec3u(1, 0, 1),
    vec3u(0, 1, 1),
    vec3u(1, 1, 1),
);

@compute @workgroup_size(8, 8, 8)
fn build_vertex_buffer(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(sdf);
    if any(gid >= (dim - vec3u(1))) {
        return;
    }
    let dimf = vec3f(dim);
    let x = vec3f(gid) / dimf - vec3f(0.5);
    let offsets = array<vec3f, 8>(
        vec3f(offsets_unit[0]) / dimf,
        vec3f(offsets_unit[1]) / dimf,
        vec3f(offsets_unit[2]) / dimf,
        vec3f(offsets_unit[3]) / dimf,
        vec3f(offsets_unit[4]) / dimf,
        vec3f(offsets_unit[5]) / dimf,
        vec3f(offsets_unit[6]) / dimf,
        vec3f(offsets_unit[7]) / dimf,
    );

    let cube_levels = array<f32, 8>(
        textureLoad(sdf, gid + offsets_unit[0]).x,
        textureLoad(sdf, gid + offsets_unit[1]).x,
        textureLoad(sdf, gid + offsets_unit[2]).x,
        textureLoad(sdf, gid + offsets_unit[3]).x,
        textureLoad(sdf, gid + offsets_unit[4]).x,
        textureLoad(sdf, gid + offsets_unit[5]).x,
        textureLoad(sdf, gid + offsets_unit[6]).x,
        textureLoad(sdf, gid + offsets_unit[7]).x,
    );

    let cube_normanls = array<vec3f, 8>(
        textureLoad(grad_sdf, gid + offsets_unit[0]).xyz,
        textureLoad(grad_sdf, gid + offsets_unit[1]).xyz,
        textureLoad(grad_sdf, gid + offsets_unit[2]).xyz,
        textureLoad(grad_sdf, gid + offsets_unit[3]).xyz,
        textureLoad(grad_sdf, gid + offsets_unit[4]).xyz,
        textureLoad(grad_sdf, gid + offsets_unit[5]).xyz,
        textureLoad(grad_sdf, gid + offsets_unit[6]).xyz,
        textureLoad(grad_sdf, gid + offsets_unit[7]).xyz,
    );

    let lut_idx = cube_levels_to_idx(cube_levels);
    let triangles = lookup_table[lut_idx];
    for (var i = 0u; i < triangles.count; i++) {
        let triangle = triangles.triangles[i];
        let base_vertex_idx = atomicAdd(&indirect_args.vertex_count, 3u);
        for (var j = 0u; j < 3u; j++) {
            let edge = triangle.edges[j];

            let phi0 = cube_levels[edge.a];
            let phi1 = cube_levels[edge.b];
            let t = phi0 / (phi0 - phi1);
            let vertex_offset = mix(offsets[edge.a], offsets[edge.b], t);
            let position = vec4f(x + vertex_offset, 1.0);
            let normal = mix(cube_normanls[edge.a], cube_normanls[edge.b], t);

            vertices[base_vertex_idx + j] = Vertex(position, vec4f(normal, 0.0));
        }
    }
}

fn sdf_sign_flag(value: f32) -> u32 {
    if value < 0.0 {
        // 内側
        return 0;
    } else {
        // 外側
        return 1;
    }
}

// 8頂点のSDFをMarchingCubesのlook up table用インデックスに変換
fn cube_levels_to_idx(cube_levels: array<f32, 8>) -> u32 {
    let cube_signs = array<u32, 8>(
        sdf_sign_flag(cube_levels[0]),
        sdf_sign_flag(cube_levels[1]),
        sdf_sign_flag(cube_levels[2]),
        sdf_sign_flag(cube_levels[3]),
        sdf_sign_flag(cube_levels[4]),
        sdf_sign_flag(cube_levels[5]),
        sdf_sign_flag(cube_levels[6]),
        sdf_sign_flag(cube_levels[7]),
    );

    var lut_idx = 0u;
    for (var i = 0u; i < 8u; i++) {
        lut_idx += cube_signs[i] << i;
    }

    return lut_idx;
}