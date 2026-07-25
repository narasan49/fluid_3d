use bevy::prelude::*;

use crate::{
    diagnostics::fluid_bounds::FluidBounds,
    fluid::{
        BoundaryConditions, Fluid3d, FluidBoundaryMethod,
        simulation::fluid_source::{
            FluidSource, FluidSourceMode, FluidSourceShape, FluidSourceTiming,
        },
    },
    game::scene::SceneRoot,
};

pub fn spawn_simple_scene(commands: &mut Commands) {
    commands.spawn((
        SceneRoot,
        children![
            (
                Camera3d::default(),
                Transform::default()
                    .with_translation(Vec3::new(0.0, 0.0, 1.0))
                    .looking_at(Vec3::ZERO, Vec3::Y),
            ),
            single_fluid_scene()
        ],
    ));
}

pub fn single_fluid_scene() -> impl Bundle {
    (
        Fluid3d {
            resolution: UVec3::splat(64),
            rho: 997.0,
            gravity: 9.8 * Vec3::NEG_Y,
        },
        FluidBounds,
        BoundaryConditions {
            y_max: FluidBoundaryMethod::Open,
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
        children![(
            FluidSource {
                active: true,
                mode: FluidSourceMode::Source,
            },
            FluidSourceShape::Aabb {
                half_size: Vec3::new(0.95, 0.4, 0.95) * 0.5
            },
            FluidSourceTiming::OnReset,
            Transform::from_xyz(0.0, -0.5, 0.0),
        )],
    )
}
