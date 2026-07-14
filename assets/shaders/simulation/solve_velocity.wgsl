#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u_mac: texture_storage_3d<r16float, read_write>;
@group(0) @binding(1) var v_mac: texture_storage_3d<r16float, read_write>;
@group(0) @binding(2) var w_mac: texture_storage_3d<r16float, read_write>;
@group(0) @binding(3) var p: texture_storage_3d<r32float, read>;
@group(0) @binding(4) var non_solid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(5) var u_solid: texture_storage_3d<rgba16float, read>;
@group(0) @binding(6) var levelset_air0: texture_storage_3d<r32float, read>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

const X = vec3u(1, 0, 0);
const Y = vec3u(0, 1, 0);
const Z = vec3u(0, 0, 1);

const EPSIRON = 1e-2;

@compute @workgroup_size(8, 8, 4)
fn solve_velocity(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(p);
    if any(gid >= (dim + vec3u(1))) {
        return;
    }
    let factor = fluid_uniform.dt / (fluid_uniform.dx * fluid_uniform.rho);

    let f = textureLoad(non_solid_fraction, gid).xyz;
    if all(gid < (dim + X)) {
        if f.x == 0.0 {
            let u_solid_plus = textureLoad(u_solid, gid).x;
            let u_solid_minus = textureLoad(u_solid, gid - X).x;
            textureStore(u_mac, gid, vec4f(0.5 * (u_solid_plus + u_solid_minus), 0.0, 0.0, 0.0));
        } else {
            solve_u(gid, factor);
        }
    }
    
    if all(gid < (dim + Y)) {
        if f.y == 0.0 {
            let u_solid_plus = textureLoad(u_solid, gid).y;
            let u_solid_minus = textureLoad(u_solid, gid - Y).y;
            textureStore(v_mac, gid, vec4f(0.5 * (u_solid_plus + u_solid_minus), 0.0, 0.0, 0.0));
        } else {
            solve_v(gid, factor);
        }
    }

    if all(gid < (dim + Z)) {
        if f.z == 0.0 {
            let u_solid_plus = textureLoad(u_solid, gid).z;
            let u_solid_minus = textureLoad(u_solid, gid - Z).z;
            textureStore(w_mac, gid, vec4f(0.5 * (u_solid_plus + u_solid_minus), 0.0, 0.0, 0.0));
        } else {
            solve_w(gid, factor);
        }
    }
}

fn solve_u(
    idx: vec3u,
    velocity_scale: f32,
) {
    let level_plus = textureLoad(levelset_air0, idx).x;
    let level_minus = textureLoad(levelset_air0, idx - X).x;
    let level_edge = 0.5 * (level_plus + level_minus);
    if level_edge >= 0.0 || (level_plus >= 0.0 && level_minus >= 0.0) {
        // MACグリッド上のu_mac[idx]の位置(i - 0.5, j, k)が空気なら速度は0
        textureStore(u_mac, idx, vec4f(0.0));
        return;
    }

    let u = textureLoad(u_mac, idx).x;
    let du = delta_velocity(idx, idx - X, level_plus, level_minus, velocity_scale);
    textureStore(u_mac, idx, vec4f(u - du, 0.0, 0.0, 0.0));
}

fn solve_v(
    idx: vec3u,
    velocity_scale: f32,
) {
    let level_plus = textureLoad(levelset_air0, idx).x;
    let level_minus = textureLoad(levelset_air0, idx - Y).x;
    let level_edge = 0.5 * (level_plus + level_minus);
    if level_edge >= 0.0 || (level_plus >= 0.0 && level_minus >= 0.0){
        // MACグリッド上のv_mac[idx]の位置(i, j - 0.5, k)が空気なら速度は0
        textureStore(v_mac, idx, vec4f(0.0));
        return;
    }

    let v = textureLoad(v_mac, idx).x;
    let dv = delta_velocity(idx, idx - Y, level_plus, level_minus, velocity_scale);
    textureStore(v_mac, idx, vec4f(v - dv, 0.0, 0.0, 0.0));
}

fn solve_w(
    idx: vec3u,
    velocity_scale: f32,
) {
    let level_plus = textureLoad(levelset_air0, idx).x;
    let level_minus = textureLoad(levelset_air0, idx - Z).x;
    let level_edge = 0.5 * (level_plus + level_minus);
    if level_edge >= 0.0 || (level_plus >= 0.0 && level_minus >= 0.0){
        // MACグリッド上のw_mac[idx]の位置(i, j, k - 0.5)が空気なら速度は0
        textureStore(w_mac, idx, vec4f(0.0));
        return;
    }

    let w = textureLoad(w_mac, idx).x;
    let dw = delta_velocity(idx, idx - Z, level_plus, level_minus, velocity_scale);
    textureStore(w_mac, idx, vec4f(w - dw, 0.0, 0.0, 0.0));
}

fn delta_velocity(
    idx_plus: vec3u,
    idx_minus: vec3u,
    level_plus: f32,
    level_minus: f32,
    scale:f32,
) -> f32 {
    var p_plus = textureLoad(p, idx_plus).x;
    var p_minus = textureLoad(p, idx_minus).x;
    if level_plus >= 0.0 && level_minus < 0.0 {
        p_plus = p_minus * level_plus / level_minus;
    } else if level_plus < 0.0 && level_minus >= 0.0 {
        p_minus = p_plus * level_minus / level_plus;
    }

    return scale * (p_plus - p_minus);
}