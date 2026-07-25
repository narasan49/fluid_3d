#define_import_path fluid3d::primitive_sdf

const SHAPE_CAPSULE = 1u;
const SHAPE_CUBE = 2u;
const SHAPE_TRIANGULAR_PRISM = 3u;

const LARGE_FLOAT = 1e6;

struct SolidBody {
    shape: u32,
    values: array<f32, 8>,
    linear_velocity: vec3f,
    angular_velocity: vec3f,
    transform: mat4x4f,
    inverse_transform: mat4x4f,
}

struct Capsule {
    radius: f32,
    half_length: f32,
}

struct Cube {
    half_size: vec3f,
}

struct TriangularPrism {
    triangle: array<vec2f, 3>,
    half_depth: f32,
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

fn to_triangular_prism(solid: SolidBody) -> TriangularPrism {
    return TriangularPrism(
        array(
            vec2f(solid.values[0], solid.values[1]),
            vec2f(solid.values[2], solid.values[3]),
            vec2f(solid.values[4], solid.values[5]),
        ),
        solid.values[6],
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
    //             return max(d.x, max(d.y, d.z));
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
        return max(d.x, max(d.y, d.z));
    } else {
        return length(select(d, vec3f(0), is_inside));
    }
}

fn sdf_triangular_prism(triangular_prism: TriangularPrism, inverse_transform: mat4x4f, x: vec3f) -> f32 {
    let xl = (inverse_transform * vec4f(x, 1.0)).xyz;
    let xl_abs = abs(xl);
    // abs(xl.z) < half_depth => 2D三角形とのSDF 
    // abs(x1.z) >= halp_depth
    //    - z軸射影が三角形の内側 => half_depth - abs(xl)
    //    - z軸射影が三角形の外側 => sqrt(2D三角形とのSDF^2 + (half_depth - abs(xl))^2)
    let sdf_triangle_2d = sdf_triangle(triangular_prism.triangle, xl.xy);
    let dz = abs(xl.z) - triangular_prism.half_depth;
    if dz < 0.0 {
        return sdf_triangle_2d;
    } else if is_inside_triangle(triangular_prism.triangle, xl.xy) {
        return dz;
    } else {
        return length(vec2f(sdf_triangle_2d, dz));
    }
}

fn sdf_triangle(triangle: array<vec2f, 3>, x: vec2f) -> f32 {
    let seg01 = distance_to_segment(x, triangle[0], triangle[1]);
    let seg12 = distance_to_segment(x, triangle[1], triangle[2]);
    let seg20 = distance_to_segment(x, triangle[2], triangle[0]);
    if is_inside_triangle(triangle, x) {
        return -min(seg01, min(seg12, seg20));
    } else {
        return min(seg01, min(seg12, seg20));
    }
}

fn distance_to_segment(x: vec2f, a: vec2f, b: vec2f) -> f32 {
    let ab = b - a;
    let xa = x - a;
    let t = clamp(dot(xa, ab) / dot(ab, ab), 0.0, 1.0);
    return length(a + t * ab - x);
}

fn is_inside_triangle(triangle: array<vec2f, 3>, x: vec2f) -> bool {
    let cross0 = cross_2d(x - triangle[0], triangle[1] - triangle[0]) > 0.0;
    let cross1 = cross_2d(x - triangle[1], triangle[2] - triangle[1]) > 0.0;
    let cross2 = cross_2d(x - triangle[2], triangle[0] - triangle[2]) > 0.0;

    return cross0 && cross1 && cross2;
}

fn cross_2d(a: vec2f, b: vec2f) -> f32 {
    return a.y * b.x - a.x * b.y;
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
        case SHAPE_TRIANGULAR_PRISM:
        {
            let triangular_prism = to_triangular_prism(solid);
            return sdf_triangular_prism(triangular_prism, solid.inverse_transform, x);
        }
        default:
        {
            return LARGE_FLOAT;
        }
    }
}