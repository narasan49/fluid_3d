#define_import_path fluid3d::interp

fn trilinear(
    levelset: texture_storage_3d<r32float, read>,
    x: vec3f,
) -> f32 {
    let base = floor(x);
    let fract = x - base;
    let idx = vec3u(base);

    let y = array<f32, 8>(
        textureLoad(levelset, idx + vec3u(0, 0, 0)).x,
        textureLoad(levelset, idx + vec3u(1, 0, 0)).x,
        textureLoad(levelset, idx + vec3u(0, 1, 0)).x,
        textureLoad(levelset, idx + vec3u(1, 1, 0)).x,
        textureLoad(levelset, idx + vec3u(0, 0, 1)).x,
        textureLoad(levelset, idx + vec3u(1, 0, 1)).x,
        textureLoad(levelset, idx + vec3u(0, 1, 1)).x,
        textureLoad(levelset, idx + vec3u(1, 1, 1)).x,
    );

    return mix(
        mix(mix(y[0], y[1], fract.x), mix(y[2], y[3], fract.x), fract.y),
        mix(mix(y[4], y[5], fract.x), mix(y[6], y[7], fract.x), fract.y),
        fract.z
    );
}

fn trilinear_rw(
    levelset: texture_storage_3d<r32float, read_write>,
    x: vec3f,
) -> f32 {
    let base = floor(x);
    let fract = x - base;
    let idx = vec3u(base);

    let y = array<f32, 8>(
        textureLoad(levelset, idx + vec3u(0, 0, 0)).x,
        textureLoad(levelset, idx + vec3u(1, 0, 0)).x,
        textureLoad(levelset, idx + vec3u(0, 1, 0)).x,
        textureLoad(levelset, idx + vec3u(1, 1, 0)).x,
        textureLoad(levelset, idx + vec3u(0, 0, 1)).x,
        textureLoad(levelset, idx + vec3u(1, 0, 1)).x,
        textureLoad(levelset, idx + vec3u(0, 1, 1)).x,
        textureLoad(levelset, idx + vec3u(1, 1, 1)).x,
    );

    return mix(
        mix(mix(y[0], y[1], fract.x), mix(y[2], y[3], fract.x), fract.y),
        mix(mix(y[4], y[5], fract.x), mix(y[6], y[7], fract.x), fract.y),
        fract.z
    );
}

fn trilinear_rgba16float(
    u: texture_storage_3d<rgba16float, read>,
    x: vec3f,
) -> vec3f {
    let base = floor(x);
    let fract = x - base;
    let idx = vec3u(base);

    let y = array<vec3f, 8>(
        textureLoad(u, idx + vec3u(0, 0, 0)).xyz,
        textureLoad(u, idx + vec3u(1, 0, 0)).xyz,
        textureLoad(u, idx + vec3u(0, 1, 0)).xyz,
        textureLoad(u, idx + vec3u(1, 1, 0)).xyz,
        textureLoad(u, idx + vec3u(0, 0, 1)).xyz,
        textureLoad(u, idx + vec3u(1, 0, 1)).xyz,
        textureLoad(u, idx + vec3u(0, 1, 1)).xyz,
        textureLoad(u, idx + vec3u(1, 1, 1)).xyz,
    );

    return mix(
        mix(mix(y[0], y[1], fract.x), mix(y[2], y[3], fract.x), fract.y),
        mix(mix(y[4], y[5], fract.x), mix(y[6], y[7], fract.x), fract.y),
        fract.z
    );
}