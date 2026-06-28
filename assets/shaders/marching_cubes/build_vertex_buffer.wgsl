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
    if any(gid >= dim) {
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
        textureLoad(sdf, gid + offsets_unit[0]).r,
        textureLoad(sdf, gid + offsets_unit[1]).r,
        textureLoad(sdf, gid + offsets_unit[2]).r,
        textureLoad(sdf, gid + offsets_unit[3]).r,
        textureLoad(sdf, gid + offsets_unit[4]).r,
        textureLoad(sdf, gid + offsets_unit[5]).r,
        textureLoad(sdf, gid + offsets_unit[6]).r,
        textureLoad(sdf, gid + offsets_unit[7]).r,
    );

    let lut_idx = cube_levels_to_idx(cube_levels);
    let triangles = lookup_table[lut_idx];
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