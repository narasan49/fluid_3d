#import fluid3d::fluid_uniform::FluidUniform
#import fluid3d::interp::{trilinear, trilinear_rgba16float}

@group(0) @binding(0) var u0: texture_storage_3d<rgba16float, read>;
@group(0) @binding(1) var levelset_air0: texture_storage_3d<r32float, read>;
@group(0) @binding(2) var levelset_air1: texture_storage_3d<r32float, write>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

@compute @workgroup_size(8, 8, 4)
fn advect_levelset(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(levelset_air0);
    if any(gid >= dim) {
        return;
    }

    var backtraced_x = backtrace(u0, gid, fluid_uniform.dt);
    backtraced_x = clamp(backtraced_x, vec3f(0.0), vec3f(dim) - vec3f(1.0));
    var new_level = trilinear(levelset_air0, backtraced_x);
    
    // インターフェース付近は精度よくもう一度補間する
    if abs(new_level) < 2.0 {
        let base = floor(backtraced_x);
        let t = backtraced_x - base;
        let new_level_cubic = cubic_xyz(levelset_air0, vec3i(base), t);
        if abs(new_level_cubic - new_level) < 0.1 {
            // レベルセットの再初期化が行われていない点が補間に含まれると、精度が悪化することに対処するワークアラウンド。
            new_level = new_level_cubic;
        }
    }

    if abs(new_level) < 100.0 {
        textureStore(levelset_air1, gid, vec4f(new_level, 0.0, 0.0, 0.0));
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
    let velocity_mid = trilinear_rgba16float(u, x_mid);

    return vec3f(x) - dt * velocity_mid;
}

// cutmull-romによるcubic補間
fn cubic(y: vec4f, t: f32) -> f32 {
    let dydx1 = 0.5 * (y.z - y.x);
    let dydx2 = 0.5 * (y.w - y.y);
    let dydx3 = y.z - y.y;

    let a = vec4f(
        y.y,
        dydx1,
        -2.0 * dydx1 - dydx2 + 3.0 * dydx3,
        dydx1 + dydx2 - 2.0 * dydx3,
    );

    return a.x + a.y * t + a.z * t * t + a.w * t * t * t;
}

fn cubic_x(
    tex: texture_storage_3d<r32float, read>,
    base_idx: vec3i,
    t: f32,
) -> f32 {
    let y = vec4f(
        textureLoad(tex, base_idx - vec3i(1, 0, 0)).x,
        textureLoad(tex, base_idx).x,
        textureLoad(tex, base_idx + vec3i(1, 0, 0)).x,
        textureLoad(tex, base_idx + vec3i(2, 0, 0)).x,
    );

    return cubic(y, t);
}

fn cubic_xy(
    tex: texture_storage_3d<r32float, read>,
    base_idx: vec3i,
    t: vec2f,
) -> f32 {
    let y = vec4f(
        cubic_x(tex, base_idx - vec3i(0, 1, 0), t.x),
        cubic_x(tex, base_idx, t.x),
        cubic_x(tex, base_idx + vec3i(0, 1, 0), t.x),
        cubic_x(tex, base_idx + vec3i(0, 2, 0), t.x),
    );

    return cubic(y, t.y);
}

fn cubic_xyz(
    tex: texture_storage_3d<r32float, read>,
    base_idx: vec3i,
    t: vec3f,
) -> f32 {
    let y = vec4f(
        cubic_xy(tex, base_idx - vec3i(0, 0, 1), t.xy),
        cubic_xy(tex, base_idx, t.xy),
        cubic_xy(tex, base_idx + vec3i(0, 0, 1), t.xy),
        cubic_xy(tex, base_idx + vec3i(0, 0, 2), t.xy),
    );

    return cubic(y, t.z);
}