#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u_mac: texture_storage_3d<r16float, read_write>;
@group(0) @binding(1) var v_mac: texture_storage_3d<r16float, read_write>;
@group(0) @binding(2) var w_mac: texture_storage_3d<r16float, read_write>;
@group(0) @binding(3) var p: texture_storage_3d<r32float, read>;
@group(0) @binding(4) var fluid_fraction: texture_storage_3d<rgba16float, read>;
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

    let f = textureLoad(fluid_fraction, gid).xyz;
    if all(gid < (dim + X)) {
        if f.x == 0.0 {
            let u_solid_plus = textureLoad(u_solid, gid).x;
            let u_solid_minus = textureLoad(u_solid, gid - X).x;
            textureStore(u_mac, gid, vec4f(0.5 * (u_solid_plus + u_solid_minus), 0.0, 0.0, 0.0));
        } else {
            var p_plus = textureLoad(p, gid).x;
            var p_minus = textureLoad(p, gid - X).x;
            let level_plus = textureLoad(levelset_air0, gid).x;
            let level_minus = textureLoad(levelset_air0, gid - X).x;
            if level_plus >= 0.0 && level_minus < 0.0 {
                p_plus = p_minus * level_plus / (level_minus - EPSIRON);
            } else if level_plus < 0.0 && level_minus >= 0.0 {
                p_minus = p_plus * level_minus / (level_plus - EPSIRON);
            }

            if level_plus < 0.0 || level_minus < 0.0 {
                let u = textureLoad(u_mac, gid).x;
                let du = factor * (p_plus - p_minus);
                textureStore(u_mac, gid, vec4f(u - du, 0.0, 0.0, 0.0));
            } else {
                textureStore(u_mac, gid, vec4f(0.0));
            }
        }
    }
    
    if all(gid < (dim + Y)) {
        if f.y == 0.0 {
            let u_solid_plus = textureLoad(u_solid, gid).y;
            let u_solid_minus = textureLoad(u_solid, gid - Y).y;
            textureStore(v_mac, gid, vec4f(0.5 * (u_solid_plus + u_solid_minus), 0.0, 0.0, 0.0));
        } else {
            var p_plus = textureLoad(p, gid).x;
            var p_minus = textureLoad(p, gid - Y).x;
            let level_plus = textureLoad(levelset_air0, gid).x;
            let level_minus = textureLoad(levelset_air0, gid - Y).x;
            if level_plus >= 0.0 && level_minus < 0.0 {
                p_plus = p_minus * level_plus / (level_minus - EPSIRON);
            } else if level_plus < 0.0 && level_minus >= 0.0 {
                p_minus = p_plus * level_minus / (level_plus - EPSIRON);
            }

            if level_plus < 0.0 || level_minus < 0.0 {
                let v = textureLoad(v_mac, gid).x;
                let dv = factor * (p_plus - p_minus);
                textureStore(v_mac, gid, vec4f(v - dv, 0.0, 0.0, 0.0));
            } else {
                textureStore(v_mac, gid, vec4f(0.0));
            }
        }
    }

    if all(gid < (dim + Z)) {
        if f.z == 0.0 {
            let u_solid_plus = textureLoad(u_solid, gid).z;
            let u_solid_minus = textureLoad(u_solid, gid - Z).z;
            textureStore(w_mac, gid, vec4f(0.5 * (u_solid_plus + u_solid_minus), 0.0, 0.0, 0.0));
        } else {
            var p_plus = textureLoad(p, gid).x;
            var p_minus = textureLoad(p, gid - Z).x;
            let level_plus = textureLoad(levelset_air0, gid).x;
            let level_minus = textureLoad(levelset_air0, gid - Z).x;
            if level_plus >= 0.0 && level_minus < 0.0 {
                p_plus = p_minus * level_plus / (level_minus - EPSIRON);
            } else if level_plus < 0.0 && level_minus >= 0.0 {
                p_minus = p_plus * level_minus / (level_plus - EPSIRON);
            }

            if level_plus < 0.0 || level_minus < 0.0 {
                let w = textureLoad(w_mac, gid).x;
                let dw = factor * (p_plus - p_minus);
                textureStore(w_mac, gid, vec4f(w - dw, 0.0, 0.0, 0.0));
            } else {
                textureStore(w_mac, gid, vec4f(0.0));
            }
        }
    }
}

fn adjacent_pressure(level: f32, level_adj: f32, p: f32, p_adj: f32, f:f32) -> f32 {
    if level_adj >= 0.0 {
        return p * level_adj / level;
    } else {
        return (1.0 - f) * p + f * p_adj;
    }
}