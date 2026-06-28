#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u0: texture_storage_3d<rgba16float, read>;
@group(0) @binding(1) var u1: texture_storage_3d<rgba16float, write>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@compute @workgroup_size(8, 8, 4)
fn advect_velocity(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(u0);
    if any(gid >= dim) {
        return;
    }

    let backtraced_x = backtrace(u0, gid, fluid_uniform.dt);
    if is_inside(backtraced_x, vec3f(dim)) {
        let backtraced_u = trilinear(u0, backtraced_x);
        textureStore(u1, gid, vec4f(backtraced_u, 0.0));
    } else {
        textureStore(u1, gid, vec4f(0.0));
    }

}

fn is_inside(x: vec3f, dimf: vec3f) -> bool {
    return all(x >= vec3f(0.0)) && all(x <= (dimf - vec3f(1.0)));
}

fn backtrace(
    u: texture_storage_3d<rgba16float, read>,
    x: vec3u,
    dt: f32,
) -> vec3f {
    let velocity = textureLoad(u, x).xyz;
    let x_mid = vec3f(x) - 0.5 * dt * velocity;
    let velocity_mid = trilinear(u, x_mid);

    return vec3f(x) - dt * velocity_mid;
}

fn trilinear(
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