#import fluid3d::fluid_uniform::FluidUniform

@group(0) @binding(0) var u_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(1) var v_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(2) var w_mac: texture_storage_3d<r16float, read>;
@group(0) @binding(3) var u_solid: texture_storage_3d<rgba16float, read>;
@group(0) @binding(4) var non_solid_fraction: texture_storage_3d<rgba16float, read>;
@group(0) @binding(5) var div: texture_storage_3d<r32float, write>;

@group(1) @binding(0) var<uniform> fluid_uniform: FluidUniform;

const X = vec3u(1, 0, 0);
const Y = vec3u(0, 1, 0);
const Z = vec3u(0, 0, 1);

@compute @workgroup_size(8, 8, 4)
fn divergence(
    @builtin(global_invocation_id) gid: vec3u,
) {
    let dim = textureDimensions(div);
    if any(gid >= dim) {
        return;
    }

    let u_minus = vec3f(
        textureLoad(u_mac, gid).x,
        textureLoad(v_mac, gid).x,
        textureLoad(w_mac, gid).x,
    );
    let u_plus = vec3f(
        textureLoad(u_mac, gid + X).x,
        textureLoad(v_mac, gid + Y).x,
        textureLoad(w_mac, gid + Z).x,
    );

    let u_solid_center = textureLoad(u_solid, gid).xyz;
    
    let f_minus = textureLoad(non_solid_fraction, gid).xyz;
    let f_plus = vec3f(
        textureLoad(non_solid_fraction, gid + X).x,
        textureLoad(non_solid_fraction, gid + Y).y,
        textureLoad(non_solid_fraction, gid + Z).z,
    );

    var du = f_plus * u_plus - f_minus * u_minus;
    let du_solid = (f_minus - f_plus) * u_solid_center;
    let result = - (du.x + du.y + du.z + du_solid.x + du_solid.y + du_solid.z) / fluid_uniform.dx;

    textureStore(div, gid, vec4f(result, 0.0, 0.0, 0.0));
}