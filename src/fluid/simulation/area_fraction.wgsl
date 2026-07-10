#define_import_path fluid3d::area_fraction

fn load_area_fraction(
    area_fraction: texture_storage_3d<rgba16float, read>,
    idx: vec3i,
) -> array<f32, 6> {
    let fractions_minus = textureLoad(area_fraction, idx).xyz;
    let fraction_x_plus = textureLoad(area_fraction, idx + vec3i(1, 0, 0)).x;
    let fraction_y_plus = textureLoad(area_fraction, idx + vec3i(0, 1, 0)).y;
    let fraction_z_plus = textureLoad(area_fraction, idx + vec3i(0, 0, 1)).z;

    return array<f32, 6>(
        fractions_minus.x,
        fraction_x_plus,
        fractions_minus.y,
        fraction_y_plus,
        fractions_minus.z,
        fraction_z_plus,
    );
}

fn fully_solid(fractions: array<f32, 6>) -> bool {
    return fractions[0] == 0.0 && fractions[1] == 0.0 && fractions[2] == 0.0 && fractions[3] == 0.0 && fractions[4] == 0.0 && fractions[5] == 0.0;
}