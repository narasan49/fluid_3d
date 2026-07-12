#define_import_path fluid3d::primitive_sdf

const SHAPE_CAPSULE = 1u;
const SHAPE_CUBE = 2u;
const SHAPE_TRIANGLE_EXTRUDE = 3u;

const LARGE_FLOAT = 1e6;

struct SolidBody {
    shape: u32,
    values: array<f32, 4>,
    linear_velocity: vec3f,
    transform: mat4x4f,
    inverse_transform: mat4x4f,
}

struct Capsule {
    half_length: f32,
    radius: f32,
}

struct Cube {
    half_size: vec3f,
}

fn to_capsule(solid: SolidBody) -> Capsule {
    return Capsule(
        solid.values[0],
        solid.values[1],
    );
}

fn to_cuboid(solid: SolidBody) -> Cube {
    return Cube(
        vec3f(solid.values[0], solid.values[1], solid.values[2]),
    );
}

fn sdf_capsule(capsule: Capsule, inverse_transform: mat4x4f, x: vec3f) -> f32 {
    // 剛体のローカル座標でSDFを計算
    let xl = inverse_transform * vec4f(x, 1.0);
    if abs(xl.y) < capsule.half_length {
        return length(vec2f(xl.x, xl.z)) - capsule.radius;
    } else {
        let v = vec3f(xl.x, abs(xl.y) - capsule.half_length, xl.z);
        return length(v) - capsule.radius;
    }
}

fn sdf_cube(cube: Cube, inverse_transform: mat4x4f, x: vec3f) -> f32 {
    let xl = (inverse_transform * vec4f(x, 1.0)).xyz;
    let xl_abs = abs(xl);
    let d = xl_abs - cube.half_size;
    let is_inside = xl_abs < cube.half_size;

    // if is_inside.x {
    //     if is_inside.y {
    //         if is_inside.z {
    //             return min(d.x, min(d.y, d.z));
    //         } else {
    //             return d.z;
    //         }
    //     } else {
    //         if is_inside.z {
    //             return d.y;
    //         } else {
    //             return length(vec2f(d.y, d.z));
    //         }
    //     }
    // } else {
    //     if is_inside.y {
    //         if is_inside.z {
    //             return d.x;
    //         } else {
    //             return length(vec2f(d.x, d.z));
    //         }
    //     } else {
    //         if is_inside.z {
    //             return length(vec2f(d.x, d.y));
    //         } else {
    //             return length(vec3f(d.x, d.y, d.z));
    //         }
    //     }
    // }
    // を以下のように短縮
    if all(is_inside) {
        return min(d.x, min(d.y, d.z));
    } else {
        return length(select(d, vec3f(0), is_inside));
    }
}

fn sdf_solid_body(solid: SolidBody, x: vec3f) -> f32 {
    switch solid.shape {
        case SHAPE_CAPSULE:
        {
            let capsule = to_capsule(solid);
            return sdf_capsule(capsule, solid.inverse_transform, x);
        }
        case SHAPE_CUBE:
        {
            let cube = to_cuboid(solid);
            return sdf_cube(cube, solid.inverse_transform, x);
        }
        default:
        {
            return LARGE_FLOAT;
        }
    }
}