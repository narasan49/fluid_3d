use bevy::{color::palettes, prelude::*};

use crate::fluid::{Fluid3d, GridLength};

pub struct FluidBoundsPlugin;

impl Plugin for FluidBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_fluid_bounds);
    }
}

#[derive(Component)]
pub struct FluidBounds;

fn update_fluid_bounds(
    query: Query<(&Fluid3d, &GlobalTransform), With<FluidBounds>>,
    grid_length: Res<GridLength>,
    mut gizmos: Gizmos,
) {
    for (fluid, transform) in &query {
        let size = fluid.resolution.as_vec3() * (grid_length.0 as f32);

        gizmos.primitive_3d(
            &Cuboid::from_size(size),
            Isometry3d::from_translation(transform.translation()),
            Color::Srgba(palettes::basic::GREEN),
        );
    }
}
